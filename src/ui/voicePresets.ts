/**
 * Voice presets: a body, a source and a speaking style together. The twin of the engine's
 * `chimera::voice::preset` (keep the numbers equal).
 */
import type { BodyMix, VoiceSource } from "../wasm/types.ts";
import { sameBody } from "./body.ts";

export interface VoicePreset {
  body: BodyMix;
  source: Partial<VoiceSource>;
  /** the planner's stressed-syllable pitch excursion (its default is 0.14) */
  accent: number;
}

export const ANIME: VoicePreset = {
  body: { female: 1, child: 0.6 },
  source: { f0: 320, glide: 0.045, pressure: 8.5, breathiness: 0.3, tension: 0.4, fry: 0, roughness: 0.03, vibratoDepth: 0, vibratoRate: 6, drift: 3, voiced: true, brightness: 0.5 },
  accent: 0.3,
};

export const isAnime = (b: BodyMix) => sameBody(b, ANIME.body);

/** The speech accent for this body: the preset's, or the planner's default. */
export const accentFor = (b: BodyMix): number | undefined => (isAnime(b) ? ANIME.accent : undefined);
