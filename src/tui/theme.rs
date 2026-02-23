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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_color_constants() {
        assert_eq!(PRIMARY, Color::Cyan);
        assert_eq!(SUCCESS, Color::Green);
        assert_eq!(ERROR, Color::Red);
        assert_eq!(WARNING, Color::Yellow);
        assert_eq!(MUTED, Color::DarkGray);
        assert_eq!(TEXT, Color::White);
    }

    #[test]
    fn test_priority_colors() {
        assert_eq!(P1_COLOR, Color::Red);
        assert_eq!(P2_COLOR, Color::Yellow);
        assert_eq!(P3_COLOR, Color::Green);
    }

    #[test]
    fn test_gherkin_syntax_colors() {
        assert_eq!(GHERKIN_KEYWORD, Color::Blue);
        assert_eq!(GHERKIN_TAG, Color::Cyan);
        assert_eq!(GHERKIN_STRING, Color::Green);
        assert_eq!(GHERKIN_COMMENT, Color::DarkGray);
    }

    #[test]
    fn test_title_style() {
        let style = title();
        assert_eq!(style.fg, Some(PRIMARY));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_selected_style() {
        let style = selected();
        assert_eq!(style.fg, Some(Color::Black));
        assert_eq!(style.bg, Some(PRIMARY));
    }

    #[test]
    fn test_status_styles() {
        assert_eq!(status_ok().fg, Some(SUCCESS));
        assert_eq!(status_error().fg, Some(ERROR));
        assert_eq!(status_warning().fg, Some(WARNING));
    }

    #[test]
    fn test_tab_styles() {
        let active = tab_active();
        assert_eq!(active.fg, Some(Color::Black));
        assert_eq!(active.bg, Some(PRIMARY));
        assert!(active.add_modifier.contains(Modifier::BOLD));

        let inactive = tab_inactive();
        assert_eq!(inactive.fg, Some(Color::Gray));
    }

    #[test]
    fn test_border_styles() {
        assert_eq!(border().fg, Some(Color::Gray));
        assert_eq!(border_active().fg, Some(PRIMARY));
    }
}
