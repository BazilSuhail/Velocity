use std::sync::OnceLock;

use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::border;

use crate::platform::{color_enabled, truecolor_supported, unicode_supported};

/// Named styles used by the TUI. Every color in the UI comes from here.
pub struct Theme {
    pub accent: Style,
    pub accent_dim: Style,
    pub text: Style,
    pub muted: Style,
    pub border: Style,
    pub selected: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub key_label: Style,
}

impl Theme {
    fn truecolor() -> Self {
        let rgb = Color::Rgb;
        Self {
            accent: Style::default().fg(rgb(0xFF, 0x8A, 0x3D)),
            accent_dim: Style::default().fg(rgb(0xB3, 0x5A, 0x24)),
            text: Style::default().fg(rgb(0xE6, 0xE6, 0xE6)),
            muted: Style::default().fg(rgb(0x80, 0x80, 0x80)),
            border: Style::default().fg(rgb(0x5A, 0x4A, 0x40)),
            selected: Style::default()
                .fg(rgb(0xFF, 0xB2, 0x7A))
                .bg(rgb(0x3A, 0x2A, 0x20))
                .add_modifier(Modifier::BOLD),
            success: Style::default().fg(rgb(0x6B, 0xD9, 0x68)),
            warning: Style::default().fg(rgb(0xFF, 0xC8, 0x57)),
            error: Style::default().fg(rgb(0xFF, 0x5A, 0x5A)),
            key_label: Style::default()
                .fg(rgb(0xFF, 0x8A, 0x3D))
                .add_modifier(Modifier::BOLD),
        }
    }

    fn ansi() -> Self {
        Self {
            accent: Style::default().fg(Color::Yellow),
            accent_dim: Style::default().fg(Color::DarkGray),
            text: Style::default(),
            muted: Style::default().fg(Color::DarkGray),
            border: Style::default().fg(Color::DarkGray),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            success: Style::default().fg(Color::Green),
            warning: Style::default().fg(Color::Yellow),
            error: Style::default().fg(Color::Red),
            key_label: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        }
    }

    /// NO_COLOR: no colors at all; the selection stays visible via reverse video.
    fn plain() -> Self {
        Self {
            accent: Style::default().add_modifier(Modifier::BOLD),
            accent_dim: Style::default(),
            text: Style::default(),
            muted: Style::default().add_modifier(Modifier::DIM),
            border: Style::default(),
            selected: Style::default().add_modifier(Modifier::REVERSED),
            success: Style::default(),
            warning: Style::default(),
            error: Style::default().add_modifier(Modifier::BOLD),
            key_label: Style::default().add_modifier(Modifier::BOLD),
        }
    }
}

/// Picks the palette from terminal capabilities.
pub fn detect() -> Theme {
    if !color_enabled() {
        Theme::plain()
    } else if truecolor_supported() {
        Theme::truecolor()
    } else {
        Theme::ansi()
    }
}

/// The theme for this process, detected once on first use.
pub fn current() -> &'static Theme {
    static THEME: OnceLock<Theme> = OnceLock::new();
    THEME.get_or_init(detect)
}

/// Box-drawing borders when Unicode is supported, otherwise plain ASCII `+ | -`.
pub fn border_set() -> border::Set {
    if unicode_supported() {
        border::ROUNDED
    } else {
        border::Set {
            top_left: "+",
            top_right: "+",
            bottom_left: "+",
            bottom_right: "+",
            vertical_left: "|",
            vertical_right: "|",
            horizontal_top: "-",
            horizontal_bottom: "-",
        }
    }
}
