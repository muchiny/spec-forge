//! Ecran Logs — affichage des logs en temps reel

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::{App, LogLevel};
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let content_width = area.width.saturating_sub(4) as usize; // bordures + marge
    let visible_height = area.height.saturating_sub(2) as usize;

    let lines: Vec<Line> = app
        .logs
        .iter()
        .map(|entry| {
            let level_style = match entry.level {
                LogLevel::Info => Style::default().fg(theme::SUCCESS),
                LogLevel::Warn => Style::default().fg(theme::WARNING),
                LogLevel::Error => theme::status_error(),
                LogLevel::Debug => theme::muted(),
            };
            let level_label = match entry.level {
                LogLevel::Info => "INFO ",
                LogLevel::Warn => "WARN ",
                LogLevel::Error => "ERROR",
                LogLevel::Debug => "DEBUG",
            };

            Line::from(vec![
                Span::styled(format!("  {} ", entry.timestamp), theme::muted()),
                Span::styled(format!("{level_label} "), level_style),
                Span::styled(entry.message.clone(), Style::default().fg(theme::TEXT)),
            ])
        })
        .collect();

    // Estimer le nombre total de lignes visuelles (avec wrapping)
    let total_visual: usize = lines
        .iter()
        .map(|line| {
            let w: usize = line.spans.iter().map(|s| s.content.len()).sum();
            if content_width > 0 {
                (w.max(1).saturating_sub(1)) / content_width + 1
            } else {
                1
            }
        })
        .sum();

    let max_scroll = total_visual.saturating_sub(visible_height);

    let scroll_y = if app.logs_auto_scroll {
        max_scroll
    } else {
        app.logs_scroll = app.logs_scroll.min(max_scroll);
        app.logs_scroll
    };

    let total = app.logs.len();
    let title = format!(" Logs ({total}) [↑↓ scroll, a=auto-scroll, Home=debut] ");
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(theme::border_active());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll_y as u16, 0));
    frame.render_widget(paragraph, area);
}
