//! **Hear the score alone** (`sound::score`, 2026-11-22): each planet's
//! music bed and air through the voice bank (its reverb and master), with
//! no game, so a mood can be tuned by ear without playing to it.
//!
//! `cargo run --release --example scoreplay -- OUT_DIR [SECS] [MOOD]`
//! writes `OUT_DIR/score-MOOD.wav` for every mood (or the one named),
//! SECS long (default 40): calm for the first third, the slime closing in
//! (tension rising to 1) for the second, easing off for the last.

use slime_mould::sound::synth::VoiceBank;
use slime_mould::sound::{Mood, SoundCmd};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = std::path::PathBuf::from(args.first().ok_or("usage: scoreplay OUT_DIR [SECS] [MOOD]")?);
    let secs: f32 = args.get(1).map_or(Ok(40.0), |s| s.parse()).map_err(|e| format!("SECS: {e}"))?;
    let moods: Vec<Mood> = match args.get(2) {
        Some(m) => vec![Mood::from_name(m).ok_or(format!("no mood {m}"))?],
        None => Mood::ALL.to_vec(),
    };
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let sr = 48_000;
    for mood in moods {
        let mut bank = VoiceBank::new(sr, 1);
        let mut all = Vec::new();
        // A frame at a time, as the window sends it.
        let frames = (secs * 60.0) as usize;
        for f in 0..frames {
            let t = f as f32 / frames as f32;
            let tension = if t < 1.0 / 3.0 { 0.0 } else if t < 2.0 / 3.0 { ((t - 1.0 / 3.0) * 6.0).min(1.0) } else { (1.0 - (t - 2.0 / 3.0) * 6.0).max(0.0) };
            bank.apply(SoundCmd::Score { mood, tension, level: 1.0, hushed: false });
            let mut buf = vec![0.0f32; sr as usize / 60 * 2];
            bank.render(&mut buf, 2);
            all.extend_from_slice(&buf);
        }
        let name = format!("{mood:?}").to_lowercase();
        let path = out.join(format!("score-{name}.wav"));
        slime_mould::wav::write(&path, &all, 2, sr).map_err(|e| e.to_string())?;
        let rms = (all.iter().map(|x| x * x).sum::<f32>() / all.len() as f32).sqrt();
        println!("{}: {:.1} dBFS", path.display(), 20.0 * rms.max(1e-9).log10());
    }
    Ok(())
}
