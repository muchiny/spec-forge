//! Widget Progress — barre de progression du pipeline (utilise dans le header si running)

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Gauge};

use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let progress = app.pipeline_state.progress_percent();
    let label = format!("Pipeline: {}%", progress);

    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(theme::PRIMARY))
        .percent(progress)
        .label(label);
    frame.render_widget(gauge, area);
}
