//! Voice presets: the anime voice sounds at its pitch and holds at any device rate.
use chimera::analysis::pitch::{YinOpts, yin_pitch};
use chimera::engine::Engine;
use chimera::voice::preset::{ANIME, voice_preset};

fn held_anime(fs: f64) -> Vec<f32> {
    let mut e = Engine::new(fs);
    e.set_body(ANIME.body);
    e.set_source(&ANIME.source);
    e.set_gate(1.0);
    let mut out = vec![0.0f32; 2 * fs as usize];
    for chunk in out.chunks_mut(128) {
        e.render(chunk);
    }
    out.split_off(fs as usize)
}

fn rms(x: &[f32]) -> f64 {
    (x.iter().map(|&v| (v as f64) * (v as f64)).sum::<f64>() / x.len() as f64).sqrt()
}

#[test]
fn the_anime_voice_sounds_at_its_pitch() {
    let tail = held_anime(48000.0);
    let frame: Vec<f64> = tail[..4096].iter().map(|&v| v as f64).collect();
    let p = yin_pitch(&frame, 48000.0, YinOpts { min_f0: Some(80.0), max_f0: Some(900.0), threshold: None });
    let want = ANIME.source.f0.unwrap();
    assert!((p.f0 / want - 1.0).abs() < 0.03, "held anime voice at {:.1} Hz, preset {want}", p.f0);
    assert_eq!(voice_preset("anime"), Some(ANIME));
}

#[test]
fn the_anime_voice_holds_at_low_device_rates() {
    // its body is shorter than the female's: check the airway stays stable where the child's once rang
    let reference = rms(&held_anime(48000.0));
    for fs in [8000.0, 16000.0, 22050.0, 32000.0] {
        let tail = held_anime(fs);
        assert!(tail.iter().all(|x| x.is_finite()), "{fs} Hz: the voice went NaN");
        let r = rms(&tail);
        assert!(r > 0.67 * reference && r < 1.5 * reference, "anime at {fs} Hz: RMS {r:.3}, 48 kHz {reference:.3}");
    }
}
