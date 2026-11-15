//! Voice presets: a body, a source and a speaking style taken together. A body preset alone sets
//! only the anatomy and a default pitch. `src/ui/voicePresets.ts` is the page's twin.

use crate::anatomy::human::BodyMix;
use crate::voice::human_voice::SourcePatch;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VoicePreset {
    pub name: &'static str,
    pub body: BodyMix,
    pub source: SourcePatch,
    /// the planner's stressed-syllable pitch excursion (its default is 0.14)
    pub accent: f64,
}

/// An anime heroine: a body between the adult female and the child (a short tract with the
/// female's proportions), pitched above both, soft and breathy at the folds but bright, with a
/// wide, sing-song intonation.
pub const ANIME: VoicePreset = VoicePreset {
    name: "anime",
    body: BodyMix { female: 1.0, child: 0.6 },
    source: SourcePatch {
        f0: Some(320.0),
        glide: Some(0.045),
        pressure: Some(8.5),
        breathiness: Some(0.3),
        tension: Some(0.4),
        fry: Some(0.0),
        roughness: Some(0.03),
        vibrato_depth: Some(0.0),
        vibrato_rate: Some(6.0),
        drift: Some(3.0),
        voiced: Some(true),
        brightness: Some(0.5),
    },
    accent: 0.3,
};

pub const VOICE_PRESETS: [VoicePreset; 1] = [ANIME];

pub fn voice_preset(name: &str) -> Option<VoicePreset> {
    VOICE_PRESETS.iter().copied().find(|p| p.name == name)
}
