use iced::alignment;
use iced::widget::{column, container, row, scrollable, text, text_input, toggler, Column};
use iced::{Element, Length};

use crate::constants::{DEFAULT_EXPORT_FILENAME, MAX_OCTAVE, MAX_TEMPO, MIN_OCTAVE, MIN_TEMPO};
use crate::generator::*;
use crate::music::note_name;
use crate::ui::{grid_line_for_step, PreviewNoteIndex, PreviewStep};

use super::sidebar::{expandable_group, SidebarSection};
use super::widgets::{
    chord_style, field_style, labeled_pick, labeled_slider_u16, labeled_slider_u8, panel_style,
    section_label, segmented_control, summary_style, timeline_cell_style, toolbar_button,
};
use super::{MelodyApp, Message};

impl MelodyApp {
    pub(super) fn view_content(&self) -> Element<'_, Message> {
        let controls = self.controls();
        let preview = self.preview();

        container(
            column![
                self.top_bar(),
                row![controls, preview].spacing(14).height(Length::Fill),
                text(&self.ui.status).size(14)
            ]
            .padding(14)
            .spacing(12),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn top_bar(&self) -> Element<'_, Message> {
        container(
            column![
                row![
                    column![
                        text("Melody").size(22),
                        text(format!("{} generator", self.music.settings.mode)).size(12)
                    ]
                    .spacing(1)
                    .width(Length::Fill),
                    toolbar_button("Generate", Message::Generate, true),
                    toolbar_button("Randomize", Message::RandomizeSeed, false),
                    toolbar_button("Browse", Message::BrowseExportDirectory, false),
                    toolbar_button("Export", Message::Export, false),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),
                row![
                    text("Directory").size(12).width(Length::Fixed(62.0)),
                    container(text(self.export_directory_label()).size(13))
                        .padding([7, 9])
                        .width(Length::FillPortion(2))
                        .style(field_style()),
                    text("Filename").size(12).width(Length::Fixed(58.0)),
                    text_input(DEFAULT_EXPORT_FILENAME, &self.export.filename)
                        .on_input(Message::ExportFilenameChanged)
                        .padding(7)
                        .width(Length::FillPortion(2)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),
            ]
            .spacing(9),
        )
        .padding(10)
        .style(panel_style())
        .into()
    }

    fn controls(&self) -> Element<'_, Message> {
        let mode_help = match self.music.settings.mode {
            GeneratorMode::Melodic => "Pattern grammar with chord-aware melodic contour.",
            GeneratorMode::Euclidean => "Evenly distributed pulses with rotation and accents.",
            GeneratorMode::Arp => "Chord tones unfolded as musical arpeggios.",
            GeneratorMode::Chiptune => "Gated leads, octave jumps, motifs, and pulse-bass flavor.",
            GeneratorMode::Bassline => {
                "Genre-shaped monophonic basslines for dance and beat production."
            }
            GeneratorMode::ChordPads => {
                "Sustained playable chords with gentle timing and velocity drift."
            }
        };

        let mode_controls = column![
            labeled_pick(
                "Preset",
                GeneratorPreset::ALL.to_vec(),
                self.music.settings.preset,
                Message::PresetChanged
            ),
            text("Generator").size(13),
            segmented_control(
                &GeneratorMode::ALL,
                self.music.settings.mode,
                Message::ModeChanged,
                3
            ),
            text(mode_help).size(13),
            self.arp_controls(),
            self.bassline_controls(),
        ]
        .spacing(10);

        let music_controls = column![
            labeled_pick(
                "Key",
                Key::ALL.to_vec(),
                self.music.settings.key,
                Message::KeyChanged
            ),
            labeled_pick(
                "Scale",
                Scale::ALL.to_vec(),
                self.music.settings.scale,
                Message::ScaleChanged
            ),
            labeled_slider_u16(
                "Bars",
                self.music.settings.bars,
                1..=16,
                Message::BarsChanged
            ),
            labeled_slider_u16(
                "Tempo",
                self.music.settings.tempo,
                MIN_TEMPO..=MAX_TEMPO,
                Message::TempoChanged
            ),
            labeled_slider_u8(
                "Min octave",
                self.music.settings.min_octave,
                MIN_OCTAVE..=MAX_OCTAVE,
                Message::MinOctaveChanged
            ),
            labeled_slider_u8(
                "Max octave",
                self.music.settings.max_octave,
                MIN_OCTAVE..=MAX_OCTAVE,
                Message::MaxOctaveChanged
            ),
        ]
        .spacing(10);

        let harmony_controls = column![
            toggler(
                Some("Lock chords".to_string()),
                self.music.locked_chords.is_some(),
                Message::ChordLockChanged
            ),
            labeled_pick(
                "Chord style",
                ChordStyle::ALL.to_vec(),
                self.music.settings.chord_style,
                Message::ChordStyleChanged
            ),
            labeled_slider_u8(
                "Tension",
                self.music.settings.tension,
                0..=100,
                Message::TensionChanged
            ),
            labeled_slider_u8(
                "Chord surprise",
                self.music.settings.surprise,
                0..=100,
                Message::SurpriseChanged
            ),
            labeled_slider_u8(
                "Resolution",
                self.music.settings.cadence,
                0..=100,
                Message::CadenceChanged
            ),
            labeled_slider_u8(
                "Chord inversion",
                self.music.settings.chord_inversion_amount,
                0..=100,
                Message::ChordInversionChanged
            ),
        ]
        .spacing(10);

        let rhythm_controls = column![
            labeled_pick(
                "Rhythm style",
                RhythmStyle::ALL.to_vec(),
                self.music.settings.rhythm_style,
                Message::RhythmStyleChanged
            ),
            labeled_slider_u8(
                "Note density",
                self.music.settings.density,
                10..=100,
                Message::DensityChanged
            ),
            labeled_slider_u8(
                "Gate / overlap",
                self.music.settings.note_length,
                0..=100,
                Message::NoteLengthChanged
            ),
        ]
        .spacing(10);

        let phrase_controls = column![
            labeled_slider_u8(
                "Phrase bars",
                self.music.settings.phrase_length,
                1..=self.music.settings.bars.min(8) as u8,
                Message::PhraseLengthChanged
            ),
            labeled_slider_u8(
                "Repeat",
                self.music.settings.repeat_amount,
                0..=100,
                Message::RepeatAmountChanged
            ),
            labeled_slider_u8(
                "Variation",
                self.music.settings.variation_amount,
                0..=100,
                Message::VariationAmountChanged
            ),
        ]
        .spacing(10);

        let velocity_controls = column![
            text("Velocity").size(13),
            segmented_control(
                &VelocityMode::ALL,
                self.music.settings.velocity_mode,
                Message::VelocityModeChanged,
                4
            ),
            row![
                text("Random range").size(14),
                text(format!(
                    "{}-{}",
                    self.music.settings.random_velocity_min,
                    self.music.settings.random_velocity_max
                ))
                .size(14)
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center),
            labeled_slider_u8(
                "Lower",
                self.music.settings.random_velocity_min,
                1..=127,
                Message::RandomVelocityMinChanged
            ),
            labeled_slider_u8(
                "Upper",
                self.music.settings.random_velocity_max,
                1..=127,
                Message::RandomVelocityMaxChanged
            ),
        ]
        .spacing(10);

        let seed_controls = column![
            labeled_pick(
                "Seed mode",
                SeedBehavior::ALL.to_vec(),
                self.music.settings.seed_behavior,
                Message::SeedBehaviorChanged
            ),
            text_input("Seed", &self.ui.seed_input)
                .on_input(Message::SeedChanged)
                .padding(8),
        ]
        .spacing(10);

        let controls = column![
            expandable_group(
                "Mode",
                SidebarSection::Mode,
                self.ui.sidebar.is_open(SidebarSection::Mode),
                mode_controls.into()
            ),
            expandable_group(
                "Music",
                SidebarSection::Music,
                self.ui.sidebar.is_open(SidebarSection::Music),
                music_controls.into()
            ),
            expandable_group(
                "Harmony",
                SidebarSection::Harmony,
                self.ui.sidebar.is_open(SidebarSection::Harmony),
                harmony_controls.into()
            ),
            expandable_group(
                "Rhythm",
                SidebarSection::Rhythm,
                self.ui.sidebar.is_open(SidebarSection::Rhythm),
                rhythm_controls.into()
            ),
            expandable_group(
                "Phrase",
                SidebarSection::Phrase,
                self.ui.sidebar.is_open(SidebarSection::Phrase),
                phrase_controls.into()
            ),
            expandable_group(
                "Velocity",
                SidebarSection::Velocity,
                self.ui.sidebar.is_open(SidebarSection::Velocity),
                velocity_controls.into()
            ),
            expandable_group(
                "Seed",
                SidebarSection::Seed,
                self.ui.sidebar.is_open(SidebarSection::Seed),
                seed_controls.into()
            ),
        ]
        .spacing(8);

        container(scrollable(container(controls).padding([0, 14, 0, 0])).height(Length::Fill))
            .width(Length::Fixed(286.0))
            .height(Length::Fill)
            .padding(10)
            .style(panel_style())
            .into()
    }

    fn arp_controls(&self) -> Element<'_, Message> {
        if self.music.settings.mode != GeneratorMode::Arp {
            return column![].into();
        }

        column![
            section_label("Arp"),
            labeled_slider_u8(
                "Notes in arp",
                self.music.settings.arp_note_count,
                2..=8,
                Message::ArpNoteCountChanged
            ),
            labeled_pick(
                "Pattern",
                ArpPattern::ALL.to_vec(),
                self.music.settings.arp_pattern,
                Message::ArpPatternChanged
            ),
            labeled_slider_u8(
                "Rotating note",
                self.music.settings.arp_rotate_slot,
                1..=self.music.settings.arp_note_count,
                Message::ArpRotateSlotChanged
            ),
            labeled_pick(
                "Rotation",
                ArpRotation::ALL.to_vec(),
                self.music.settings.arp_rotation,
                Message::ArpRotationChanged
            ),
        ]
        .spacing(10)
        .into()
    }

    fn bassline_controls(&self) -> Element<'_, Message> {
        if self.music.settings.mode != GeneratorMode::Bassline {
            return column![].into();
        }

        column![
            section_label("Bassline"),
            labeled_pick(
                "Style",
                BasslineStyle::ALL.to_vec(),
                self.music.settings.bassline_style,
                Message::BasslineStyleChanged
            ),
            labeled_slider_u8(
                "Accent",
                self.music.settings.bassline_accent,
                0..=100,
                Message::BasslineAccentChanged
            ),
            labeled_slider_u8(
                "Slide",
                self.music.settings.bassline_slide,
                0..=100,
                Message::BasslineSlideChanged
            ),
            labeled_slider_u8(
                "Octave jump",
                self.music.settings.bassline_octave_jump,
                0..=100,
                Message::BasslineOctaveJumpChanged
            ),
            labeled_slider_u8(
                "Pattern mutation",
                self.music.settings.bassline_mutation,
                0..=100,
                Message::BasslineMutationChanged
            ),
        ]
        .spacing(10)
        .into()
    }

    fn preview(&self) -> Element<'_, Message> {
        let chord_lane = self.music.output.chords.iter().fold(
            row![text("Chords").width(Length::Fixed(64.0)).size(12)].spacing(0),
            |row, chord| {
                row.push(
                    container(
                        text(chord.label())
                            .size(13)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                    .width(Length::FillPortion(chord.duration_ticks as u16))
                    .padding(6)
                    .style(chord_style()),
                )
            },
        );

        let note_index = PreviewNoteIndex::new(
            &self.music.output.notes,
            self.music.settings.bars,
            self.music.settings.low_pitch(),
            self.music.settings.high_pitch(),
        );

        let rows = (self.music.settings.low_pitch()..=self.music.settings.high_pitch())
            .rev()
            .fold(Column::new().spacing(1), |column, pitch| {
                let cells = self.timeline_cells(pitch, &note_index);
                column.push(
                    row![
                        text(note_name(pitch)).width(Length::Fixed(64.0)).size(11),
                        cells
                    ]
                    .align_items(iced::Alignment::Center),
                )
            });

        let summary = self.settings_strip();
        let bar_lane = (0..self.music.settings.bars as u32).fold(
            row![text("Bars").width(Length::Fixed(64.0)).size(12)].spacing(0),
            |row, bar| {
                row.push(
                    container(text(format!("{}", bar + 1)).size(12))
                        .width(Length::FillPortion(16))
                        .padding(4)
                        .style(summary_style()),
                )
            },
        );

        container(
            column![
                row![
                    text("Preview").size(20),
                    text(format!(
                        "{} {} bars",
                        self.music.settings.key, self.music.settings.bars
                    ))
                    .size(13)
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center),
                summary,
                bar_lane,
                chord_lane,
                scrollable(rows).height(Length::Fill),
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(12)
        .style(panel_style())
        .into()
    }

    fn settings_strip(&self) -> Element<'_, Message> {
        let content = format!(
            "{} | {} {} | {} bars | {} bpm | {} notes | {}",
            self.music.settings.mode,
            self.music.settings.key,
            self.music.settings.scale,
            self.music.settings.bars,
            self.music.settings.tempo,
            self.music.output.notes.len(),
            self.export.filename
        );

        container(text(content).size(13))
            .padding([7, 9])
            .width(Length::Fill)
            .style(summary_style())
            .into()
    }

    fn timeline_cells(&self, pitch: u8, note_index: &PreviewNoteIndex) -> Element<'_, Message> {
        let steps = self.music.settings.bars as u32 * 16;

        let mut row = row![].spacing(0).width(Length::Fill);
        let mut step = 0;
        while step < steps {
            let grid_line = grid_line_for_step(step);
            let preview_step = note_index.step_at(pitch, step);
            let (active_velocity, span_steps) = match preview_step {
                PreviewStep::Empty | PreviewStep::NoteContinuation => (None, 1),
                PreviewStep::NoteStart(segment) => {
                    (Some(segment.velocity), segment.span_steps.min(steps - step))
                }
            };

            row = row.push(
                container(text("").size(1))
                    .width(Length::FillPortion(span_steps as u16))
                    .height(Length::Fixed(18.0))
                    .style(timeline_cell_style(active_velocity, grid_line)),
            );
            step += span_steps;
        }

        row.into()
    }
}
