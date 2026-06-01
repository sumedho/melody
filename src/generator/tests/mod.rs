use super::arp::{arp_order, rotating_arp_pitch};
use super::bassline::choose_bass_degree_pitch;
use super::buildup_drop::buildup_drop_sections;
use super::chord_pads::{
    chord_pad_pitches, spread_voicing, voice_lead_chord_pad_voicing, voicing_center,
};
use super::chords::{borrowed_chord, chord_style_degree, extension_quality, generate_chords};
use super::common::{
    apply_phrase_memory, chord_at, chord_pitches_in_range, density_notes_per_bar, note_duration,
    octave_to_midi_c, pitch_class_for_degree, quality_for_degree, rhythm_density,
    scale_pitches_in_range,
};
use super::counter_melody::generate_counter_melody_parts;
use super::euclidean::euclidean_pattern;
use super::*;

fn note_signature(notes: &[NoteEvent]) -> Vec<(u8, u32, u32, u8)> {
    notes
        .iter()
        .map(|note| {
            (
                note.pitch,
                note.start_ticks,
                note.duration_ticks,
                note.velocity,
            )
        })
        .collect()
}

fn note_active_at(notes: &[NoteEvent], tick: u32) -> bool {
    notes
        .iter()
        .any(|note| tick >= note.start_ticks && tick < note.start_ticks + note.duration_ticks)
}

mod arp;
mod bassline;
mod buildup_drop;
mod chord_pads;
mod chords;
mod core;
mod counter_melody;
mod hooks;
