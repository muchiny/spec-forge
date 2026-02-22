//! Theme et styles constants pour la TUI

use ratatui::style::{Color, Modifier, Style};

// Couleurs principales
pub const PRIMARY: Color = Color::Cyan;
pub const SECONDARY: Color = Color::Yellow;
pub const SUCCESS: Color = Color::Green;
pub const ERROR: Color = Color::Red;
pub const WARNING: Color = Color::Yellow;
pub const MUTED: Color = Color::DarkGray;
pub const TEXT: Color = Color::White;
pub const BG: Color = Color::Reset;

// Priorites
pub const P1_COLOR: Color = Color::Red;
pub const P2_COLOR: Color = Color::Yellow;
pub const P3_COLOR: Color = Color::Green;

// Gherkin syntax highlighting
pub const GHERKIN_KEYWORD: Color = Color::Blue;
pub const GHERKIN_TAG: Color = Color::Cyan;
pub const GHERKIN_STRING: Color = Color::Green;
pub const GHERKIN_COMMENT: Color = Color::DarkGray;

// Styles predefinis
pub fn title() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn header() -> Style {
    Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
}

pub fn selected() -> Style {
    Style::default().fg(Color::Black).bg(PRIMARY)
}

pub fn status_ok() -> Style {
    Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD)
}

pub fn status_error() -> Style {
    Style::default().fg(ERROR).add_modifier(Modifier::BOLD)
}

pub fn status_warning() -> Style {
    Style::default().fg(WARNING)
}

pub fn muted() -> Style {
    Style::default().fg(MUTED)
}

pub fn border() -> Style {
    Style::default().fg(Color::Gray)
}

pub fn border_active() -> Style {
    Style::default().fg(PRIMARY)
}

pub fn tab_active() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(PRIMARY)
        .add_modifier(Modifier::BOLD)
}

pub fn tab_inactive() -> Style {
    Style::default().fg(Color::Gray)
}

pub fn key_hint() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn key_desc() -> Style {
    Style::default().fg(Color::Gray)
}
