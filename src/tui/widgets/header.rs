//! Widget Header — titre + navigation tabs

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs};

use crate::tui::app::{App, LlmStatus, Screen};
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(area);

    // Title line
    let llm_indicator = match &app.llm_status {
        LlmStatus::Ready => Span::styled(" ● ", theme::status_ok()),
        LlmStatus::Error(_) => Span::styled(" ● ", theme::status_error()),
        LlmStatus::Checking => Span::styled(" ◌ ", theme::status_warning()),
        LlmStatus::Unknown => Span::styled(" ○ ", theme::muted()),
    };

    let title_line = Line::from(vec![
        Span::styled(" spec-forge ", theme::title()),
        llm_indicator,
        Span::styled(format!("{} ", app.config.llm.model_name), theme::muted()),
    ]);
    frame.render_widget(Paragraph::new(title_line), chunks[0]);

    // Tab bar
    let titles: Vec<Line> = Screen::all()
        .iter()
        .map(|s| Line::from(format!("[{}]{}", s.key(), s.label())))
        .collect();

    let selected = Screen::all()
        .iter()
        .position(|s| *s == app.screen)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .select(selected)
        .style(theme::tab_inactive())
        .highlight_style(theme::tab_active())
        .divider(Span::styled(" ", Style::default()));

    frame.render_widget(tabs, chunks[1]);
}
