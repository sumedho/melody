use iced::theme;
use iced::widget::container::Appearance as ContainerAppearance;
use iced::widget::{button, column, pick_list, row, slider, text};
use iced::{Background, Border, Color, Element, Length, Theme};
use std::fmt::Display;

use crate::constants::VELOCITY_SHAPING_POWER;
use crate::ui::GridLine;

use super::Message;

pub(super) fn labeled_pick<'a, T, F>(
    label: &'a str,
    options: Vec<T>,
    selected: T,
    on_select: F,
) -> Element<'a, Message>
where
    T: Display + Eq + Clone + 'a,
    F: Fn(T) -> Message + 'a,
{
    column![
        text(label).size(14),
        pick_list(options, Some(selected), on_select).width(Length::Fill),
    ]
    .spacing(4)
    .into()
}

pub(super) fn segmented_control<'a, T, F>(
    options: &'a [T],
    selected: T,
    on_select: F,
    per_row: usize,
) -> Element<'a, Message>
where
    T: Display + Eq + Copy + 'a,
    F: Fn(T) -> Message + Copy + 'a,
{
    let mut rows = column![].spacing(4).width(Length::Fill);
    for chunk in options.chunks(per_row.max(1)) {
        let mut controls = row![].spacing(4).width(Length::Fill);
        for option in chunk {
            let active = *option == selected;
            let button = button(text(option.to_string()).size(12))
                .padding([6, 8])
                .width(Length::FillPortion(1))
                .style(if active {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                })
                .on_press(on_select(*option));
            controls = controls.push(button);
        }
        rows = rows.push(controls);
    }
    rows.into()
}

pub(super) fn toolbar_button(
    label: &'static str,
    message: Message,
    primary: bool,
) -> Element<'static, Message> {
    button(text(label).size(13))
        .padding([8, 11])
        .style(if primary {
            theme::Button::Primary
        } else {
            theme::Button::Secondary
        })
        .on_press(message)
        .into()
}

pub(super) fn section_label(label: &str) -> Element<'_, Message> {
    text(label).size(16).into()
}

pub(super) fn labeled_slider_u16<'a, F>(
    label: &'a str,
    value: u16,
    range: std::ops::RangeInclusive<u16>,
    on_change: F,
) -> Element<'a, Message>
where
    F: Fn(u16) -> Message + 'a,
{
    column![
        row![text(label).size(14), text(value.to_string()).size(14)]
            .spacing(8)
            .align_items(iced::Alignment::Center),
        slider(range, value, on_change),
    ]
    .spacing(4)
    .into()
}

pub(super) fn panel_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: None,
        background: Some(Background::Color(Color::from_rgb8(24, 28, 42))),
        border: Border {
            color: Color::from_rgb8(50, 58, 82),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
}

pub(super) fn chord_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(224, 231, 255)),
        background: Some(Background::Color(Color::from_rgb8(38, 48, 79))),
        border: Border {
            color: Color::from_rgb8(79, 91, 128),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}

pub(super) fn summary_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(226, 232, 240)),
        background: Some(Background::Color(Color::from_rgb8(29, 36, 53))),
        border: Border {
            color: Color::from_rgb8(62, 74, 101),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
}

pub(super) fn field_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(203, 213, 225)),
        background: Some(Background::Color(Color::from_rgb8(20, 25, 38))),
        border: Border {
            color: Color::from_rgb8(45, 55, 78),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}

pub(super) fn group_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(226, 232, 240)),
        background: Some(Background::Color(Color::from_rgb8(28, 34, 50))),
        border: Border {
            color: Color::from_rgb8(48, 57, 80),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
}

pub(super) fn timeline_cell_style(velocity: Option<u8>, grid_line: GridLine) -> theme::Container {
    theme::Container::from(move |_theme: &Theme| {
        let note_colors = velocity.map(active_note_colors);
        let background = match velocity {
            Some(_) => note_colors.expect("note colors exist for active note").0,
            None if grid_line == GridLine::Bar => Color::from_rgb8(47, 55, 76),
            None if grid_line == GridLine::Beat => Color::from_rgb8(31, 37, 52),
            None => Color::from_rgb8(22, 27, 39),
        };

        ContainerAppearance {
            text_color: None,
            background: Some(Background::Color(background)),
            border: Border {
                color: if let Some((_, border)) = note_colors {
                    border
                } else {
                    match grid_line {
                        GridLine::Bar => Color::from_rgb8(96, 111, 145),
                        GridLine::Beat => Color::from_rgb8(62, 73, 98),
                        GridLine::Step => Color::from_rgb8(35, 41, 56),
                    }
                },
                width: if velocity.is_some() || grid_line != GridLine::Step {
                    1.0
                } else {
                    0.5
                },
                radius: 2.0.into(),
            },
            ..Default::default()
        }
    })
}

pub(crate) fn active_note_colors(velocity: u8) -> (Color, Color) {
    let intensity = (velocity as f32 / 127.0).clamp(0.0, 1.0);
    let shaped = intensity.powf(VELOCITY_SHAPING_POWER);
    let background = Color {
        r: 0.08 + shaped * 0.40,
        g: 0.28 + shaped * 0.55,
        b: 0.42 + shaped * 0.50,
        a: 1.0,
    };
    let border = Color {
        r: 0.18 + shaped * 0.55,
        g: 0.44 + shaped * 0.48,
        b: 0.62 + shaped * 0.36,
        a: 1.0,
    };

    (background, border)
}

pub(super) fn labeled_slider_u8<'a, F>(
    label: &'a str,
    value: u8,
    range: std::ops::RangeInclusive<u8>,
    on_change: F,
) -> Element<'a, Message>
where
    F: Fn(u8) -> Message + 'a,
{
    column![
        row![text(label).size(14), text(value.to_string()).size(14)]
            .spacing(8)
            .align_items(iced::Alignment::Center),
        slider(range, value, on_change),
    ]
    .spacing(4)
    .into()
}
