//! **The sound device**: the only file that touches an audio API. Ported
//! from Pixel_Physics `src/audio/device.rs` unchanged but for the style.
//!
//! Compiled on macOS and Windows, and on Linux only with `--features
//! audio-alsa` (`Cargo.toml` has why). Opens the default output, runs
//! [`VoiceBank::render`] in the device's own callback thread, and drains
//! commands from a bounded channel the game thread fills with `try_send`, so
//! **neither side ever waits on the other**: a full channel drops the command
//! on the game side (textures are re-sent every drawn frame, so a drop costs
//! one frame of staleness), and the callback only ever calls `try_recv`.
//!
//! **No priority raising.** CoreAudio already runs its callback at real-time
//! priority; on Linux cpal's `realtime` feature would do it but pulls an
//! MPL-2.0 crate the licence gate rejects (`scripts/licensecheck.sh`).

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::sync::Arc;
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};

use super::synth::VoiceBank;
use super::SoundCmd;

/// Commands the channel holds before the game side starts dropping. A drawn
/// frame sends about a dozen; this is several seconds of slack.
const CHANNEL: usize = 1024;

/// How long the device may go without asking for samples before the
/// stream counts as dead (`Device::healthy`). A callback comes every 10-20
/// ms on every host we know; two seconds is a hundred missed.
const STALL: Duration = Duration::from_secs(2);

pub struct Device {
    _stream: cpal::Stream,
    tx: SyncSender<SoundCmd>,
    /// Commands dropped because the channel was full (should stay 0).
    pub dropped: u64,
    /// Set by the stream's error callback: on Windows a device that went
    /// away (unplugged, switched, the default changed, a driver reset)
    /// errors once and then the stream plays nothing, ever.
    broken: Arc<AtomicBool>,
    /// Callbacks so far, and when the count last moved: a stream that
    /// stops asking for samples without an error is dead too.
    calls: Arc<AtomicU64>,
    seen: (u64, Instant),
}

impl Device {
    /// Open the default output device, or say why not. `None`-device is the
    /// normal case on a VM, in a container, or on a headless screenshot run,
    /// and the caller then simply runs silent.
    pub fn open(seed: u64) -> Result<Device, String> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| "no output device".to_string())?;
        let supported = device.default_output_config().map_err(|e| format!("no output config: {e}"))?;
        let format = supported.sample_format();
        let config = supported.config();
        let (tx, rx) = sync_channel(CHANNEL);
        let bank = VoiceBank::new(config.sample_rate, seed);
        let (broken, calls) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicU64::new(0)));
        let flags = (broken.clone(), calls.clone());
        let stream = match format {
            cpal::SampleFormat::F32 => build::<f32>(&device, config, bank, rx, flags),
            cpal::SampleFormat::I16 => build::<i16>(&device, config, bank, rx, flags),
            cpal::SampleFormat::U16 => build::<u16>(&device, config, bank, rx, flags),
            cpal::SampleFormat::I32 => build::<i32>(&device, config, bank, rx, flags),
            other => return Err(format!("unsupported sample format {other:?}")),
        }?;
        stream.play().map_err(|e| format!("could not start the stream: {e}"))?;
        Ok(Device { _stream: stream, tx, dropped: 0, broken, calls, seen: (0, Instant::now()) })
    }

    /// Whether the stream still plays: no error from it, and it asked for
    /// samples in the last `STALL`. `sound::Output` reopens the device when
    /// it does not (2026-11-20, sound cutting out mid-planet).
    pub fn healthy(&mut self) -> bool {
        if self.broken.load(Ordering::Relaxed) {
            return false;
        }
        let n = self.calls.load(Ordering::Relaxed);
        if n != self.seen.0 {
            self.seen = (n, Instant::now());
        }
        self.seen.1.elapsed() < STALL
    }

    pub fn send(&mut self, cmds: impl IntoIterator<Item = SoundCmd>) {
        for c in cmds {
            match self.tx.try_send(c) {
                Ok(()) => {}
                Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => self.dropped += 1,
            }
        }
    }
}

fn build<T>(device: &cpal::Device, config: cpal::StreamConfig, mut bank: VoiceBank, rx: Receiver<SoundCmd>, (broken, calls): (Arc<AtomicBool>, Arc<AtomicU64>)) -> Result<cpal::Stream, String>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;
    // Scratch for non-f32 devices, sized once; grown only if a host hands us
    // a bigger buffer than any seen so far (rare, and then only once).
    let mut scratch: Vec<f32> = vec![0.0; 8192 * channels.max(1)];
    device
        .build_output_stream(
            config,
            move |data: &mut [T], _info: &cpal::OutputCallbackInfo| {
                calls.fetch_add(1, Ordering::Relaxed);
                while let Ok(cmd) = rx.try_recv() {
                    bank.apply(cmd);
                }
                if scratch.len() < data.len() {
                    scratch.resize(data.len(), 0.0);
                }
                let buf = &mut scratch[..data.len()];
                bank.render(buf, channels);
                for (d, s) in data.iter_mut().zip(buf.iter()) {
                    *d = T::from_sample(*s);
                }
            },
            move |e| {
                eprintln!("sound: stream error: {e}; reopening");
                broken.store(true, Ordering::Relaxed);
            },
            None,
        )
        .map_err(|e| format!("could not open the stream: {e}"))
}
