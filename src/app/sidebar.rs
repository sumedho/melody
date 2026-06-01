use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};

use super::widgets::group_style;
use super::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SidebarSection {
    Mode,
    Music,
    Harmony,
    Rhythm,
    Phrase,
    Velocity,
    Seed,
}

pub(super) struct SidebarState {
    mode: bool,
    music: bool,
    harmony: bool,
    rhythm: bool,
    phrase: bool,
    velocity: bool,
    seed: bool,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            mode: true,
            music: true,
            harmony: false,
            rhythm: true,
            phrase: false,
            velocity: true,
            seed: false,
        }
    }
}

impl SidebarState {
    pub(super) fn is_open(&self, section: SidebarSection) -> bool {
        match section {
            SidebarSection::Mode => self.mode,
            SidebarSection::Music => self.music,
            SidebarSection::Harmony => self.harmony,
            SidebarSection::Rhythm => self.rhythm,
            SidebarSection::Phrase => self.phrase,
            SidebarSection::Velocity => self.velocity,
            SidebarSection::Seed => self.seed,
        }
    }

    pub(super) fn toggle(&mut self, section: SidebarSection) {
        let value = match section {
            SidebarSection::Mode => &mut self.mode,
            SidebarSection::Music => &mut self.music,
            SidebarSection::Harmony => &mut self.harmony,
            SidebarSection::Rhythm => &mut self.rhythm,
            SidebarSection::Phrase => &mut self.phrase,
            SidebarSection::Velocity => &mut self.velocity,
            SidebarSection::Seed => &mut self.seed,
        };
        *value = !*value;
    }
}

pub(super) fn expandable_group<'a>(
    title: &'static str,
    section: SidebarSection,
    open: bool,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let indicator = if open { "v" } else { ">" };
    let mut group = column![button(
        row![
            text(indicator).size(12).width(Length::Fixed(13.0)),
            text(title).size(14)
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .on_press(Message::ToggleSection(section))]
    .spacing(10);

    if open {
        group = group.push(content);
    }

    container(group)
        .padding(8)
        .style(group_style())
        .width(Length::Fill)
        .into()
}
