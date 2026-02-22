//! Fonction de rendu principal — dispatch vers les ecrans

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::tui::app::{App, Screen};
use crate::tui::screens;
use crate::tui::theme;
use crate::tui::widgets;

const MIN_WIDTH: u16 = 80;
const MIN_HEIGHT: u16 = 20;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Verification taille minimale
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let msg = Line::from(vec![Span::styled(
            format!(
                "Terminal trop petit ({} x {}). Minimum: {} x {}",
                area.width, area.height, MIN_WIDTH, MIN_HEIGHT
            ),
            Style::default().fg(theme::WARNING),
        )]);
        frame.render_widget(Paragraph::new(msg), area);
        return;
    }

    let chunks = Layout::vertical([
        Constraint::Length(2), // Header + tabs
        Constraint::Min(10),   // Content
        Constraint::Length(1), // Status bar
    ])
    .split(area);

    // Header
    widgets::header::render(frame, app, chunks[0]);

    // Main content
    render_screen(frame, app, chunks[1]);

    // Status bar
    widgets::status_bar::render(frame, app, chunks[2]);

    // Help overlay
    if app.show_help {
        widgets::help::render(frame, area);
    }
}

fn render_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.screen {
        Screen::Dashboard => screens::dashboard::render(frame, app, area),
        Screen::FilePicker => screens::file_picker::render(frame, app, area),
        Screen::Pipeline => screens::pipeline::render(frame, app, area),
        Screen::SpecViewer => screens::spec_viewer::render(frame, app, area),
        Screen::GherkinViewer => screens::gherkin_viewer::render(frame, app, area),
        Screen::Traceability => screens::traceability::render(frame, app, area),
        Screen::Config => screens::config::render(frame, app, area),
        Screen::Logs => screens::logs::render(frame, app, area),
    }
}
