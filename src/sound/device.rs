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
//! **The callback thread's priority** (2026-11-22, the owner: "sound cutting
//! out in the game is still an issue"). CoreAudio runs its callback at
//! real-time priority on its own. WASAPI does not: cpal raises it only with
//! its `realtime` feature, which pulls an MPL-2.0 crate the licence gate
//! rejects (`scripts/licensecheck.sh`), so on Windows the callback ran at
//! normal priority, level with the game's own threads, the GPU driver's and
//! every other program's. On the owner's PC that is a 4K window keeping
//! the CPU busy while the strategy league runs games on every core, so the
//! callback could wait longer than the buffer (about 10 ms of sound) and
//! the device played gaps (the likely cause; the play log's `sound` line
//! now shows it). The cut-outs fixed on 11-20 (a NaN, a dead
//! stream) were real but were not this. The callback now raises its own
//! thread on its first call ([`raise_priority`]): the multimedia scheduler's
//! "Pro Audio" class, which every Windows audio program uses, or failing
//! that the highest normal priority. A few lines of FFI to two system DLLs,
//! no new crate. Linux still runs at normal priority (only with
//! `--features audio-alsa`, nobody plays there).
//!
//! **Health** ([`Health`]): the callback counts its calls, the gaps between
//! them, and its own render time, so the play log can say whether the
//! sound starved (`sound` line, every 10 s) instead of guessing.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
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

/// A gap between two callbacks this long counts as late in [`Health`]: on
/// WASAPI's shared mode the buffer holds about 10-20 ms, so a gap past
/// 30 ms has almost surely played a stretch of silence.
const LATE_US: u64 = 30_000;

/// **What the callback thread saw**, shared with the game thread, which
/// reads it for the play log. Counters only grow; `gap_us` and `render_us`
/// are the most since the last [`Health::take_most`].
#[derive(Default)]
pub struct Health {
    /// Callbacks so far (also the stall check's count).
    pub calls: AtomicU64,
    /// Callbacks that came more than `LATE_US` after the one before.
    pub late: AtomicU64,
    /// The longest gap between callbacks, µs.
    pub gap_us: AtomicU64,
    /// The longest one render, µs.
    pub render_us: AtomicU64,
    /// Errors from the stream that it plays on through (an xrun, a route
    /// change it followed); fatal ones set `Device::broken` instead.
    pub soft_errors: AtomicU64,
    /// How the callback thread's priority went: 0 not tried yet, 1 the
    /// "Pro Audio" class, 2 time-critical, 3 neither took, 4 not a
    /// platform that needs it.
    pub priority: AtomicU32,
}

impl Health {
    /// The longest gap and render since the last call, ms, and reset them.
    pub fn take_most(&self) -> (f32, f32) {
        (self.gap_us.swap(0, Ordering::Relaxed) as f32 / 1e3, self.render_us.swap(0, Ordering::Relaxed) as f32 / 1e3)
    }

    pub fn priority_name(&self) -> &'static str {
        match self.priority.load(Ordering::Relaxed) {
            1 => "proaudio",
            2 => "critical",
            3 => "normal",
            4 => "host",
            _ => "none",
        }
    }
}

pub struct Device {
    _stream: cpal::Stream,
    tx: SyncSender<SoundCmd>,
    /// Commands dropped because the channel was full (should stay 0).
    pub dropped: u64,
    /// Set by the stream's error callback: on Windows a device that went
    /// away (unplugged, switched, the default changed, a driver reset)
    /// errors once and then the stream plays nothing, ever.
    broken: Arc<AtomicBool>,
    /// The callback's counts; `calls` and when it last moved (`seen`) are
    /// the stall check: a stream that stops asking for samples without an
    /// error is dead too.
    pub health: Arc<Health>,
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
        let (broken, health) = (Arc::new(AtomicBool::new(false)), Arc::new(Health::default()));
        let flags = (broken.clone(), health.clone());
        let stream = match format {
            cpal::SampleFormat::F32 => build::<f32>(&device, config, bank, rx, flags),
            cpal::SampleFormat::I16 => build::<i16>(&device, config, bank, rx, flags),
            cpal::SampleFormat::U16 => build::<u16>(&device, config, bank, rx, flags),
            cpal::SampleFormat::I32 => build::<i32>(&device, config, bank, rx, flags),
            other => return Err(format!("unsupported sample format {other:?}")),
        }?;
        stream.play().map_err(|e| format!("could not start the stream: {e}"))?;
        Ok(Device { _stream: stream, tx, dropped: 0, broken, health, seen: (0, Instant::now()) })
    }

    /// Whether the stream still plays: no error from it, and it asked for
    /// samples in the last `STALL`. `sound::Output` reopens the device when
    /// it does not (2026-11-20, sound cutting out mid-planet).
    pub fn healthy(&mut self) -> bool {
        if self.broken.load(Ordering::Relaxed) {
            return false;
        }
        let n = self.health.calls.load(Ordering::Relaxed);
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

fn build<T>(device: &cpal::Device, config: cpal::StreamConfig, mut bank: VoiceBank, rx: Receiver<SoundCmd>, (broken, health): (Arc<AtomicBool>, Arc<Health>)) -> Result<cpal::Stream, String>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;
    // Scratch for non-f32 devices, sized once; grown only if a host hands us
    // a bigger buffer than any seen so far (rare, and then only once).
    let mut scratch: Vec<f32> = vec![0.0; 8192 * channels.max(1)];
    let mut last: Option<Instant> = None;
    let soft = health.clone();
    device
        .build_output_stream(
            config,
            move |data: &mut [T], _info: &cpal::OutputCallbackInfo| {
                let now = Instant::now();
                match last {
                    None => health.priority.store(raise_priority(), Ordering::Relaxed),
                    Some(t) => {
                        let gap = now.duration_since(t).as_micros() as u64;
                        health.gap_us.fetch_max(gap, Ordering::Relaxed);
                        if gap > LATE_US {
                            health.late.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                last = Some(now);
                health.calls.fetch_add(1, Ordering::Relaxed);
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
                health.render_us.fetch_max(now.elapsed().as_micros() as u64, Ordering::Relaxed);
            },
            move |e: cpal::Error| {
                // An xrun or a route the stream already followed is not a
                // dead stream: tearing it down for one (as until 11-22)
                // cost a fresh bank, the score restarting, for nothing.
                if matches!(e.kind(), cpal::ErrorKind::Xrun | cpal::ErrorKind::DeviceChanged | cpal::ErrorKind::RealtimeDenied) {
                    soft.soft_errors.fetch_add(1, Ordering::Relaxed);
                    return;
                }
                eprintln!("sound: stream error: {e}; reopening");
                broken.store(true, Ordering::Relaxed);
            },
            None,
        )
        .map_err(|e| format!("could not open the stream: {e}"))
}

/// **Raise the callback thread** (see the module docs), once, from inside
/// the callback. Windows: the multimedia class scheduler's "Pro Audio" task
/// (`AvSetMmThreadCharacteristicsW` in avrt.dll, loaded at run time so the
/// build links nothing new), else `THREAD_PRIORITY_TIME_CRITICAL`. The
/// thread is cpal's and ends with the stream, so nothing is undone.
/// Returns the code `Health::priority` keeps.
#[cfg(target_os = "windows")]
fn raise_priority() -> u32 {
    use std::ffi::c_void;
    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryW(name: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn GetCurrentThread() -> *mut c_void;
        fn SetThreadPriority(thread: *mut c_void, priority: i32) -> i32;
    }
    type AvSet = unsafe extern "system" fn(task: *const u16, index: *mut u32) -> *mut c_void;
    let wide = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    // SAFETY: plain Win32 calls with NUL-terminated strings that outlive
    // them; the function pointer is the documented signature of
    // `AvSetMmThreadCharacteristicsW`.
    unsafe {
        let avrt = LoadLibraryW(wide("avrt.dll").as_ptr());
        if !avrt.is_null() {
            let f = GetProcAddress(avrt, c"AvSetMmThreadCharacteristicsW".as_ptr().cast());
            if !f.is_null() {
                let set: AvSet = std::mem::transmute::<*mut c_void, AvSet>(f);
                let mut index = 0u32;
                if !set(wide("Pro Audio").as_ptr(), &mut index).is_null() {
                    return 1;
                }
            }
        }
        const THREAD_PRIORITY_TIME_CRITICAL: i32 = 15;
        if SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_TIME_CRITICAL) != 0 {
            2
        } else {
            3
        }
    }
}

/// CoreAudio's callback is already real-time; Linux (opt-in ALSA only)
/// stays as it was.
#[cfg(not(target_os = "windows"))]
fn raise_priority() -> u32 {
    4
}
