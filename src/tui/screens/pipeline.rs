//! Ecran Pipeline — execution en temps reel avec progression

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Wrap};

use crate::tui::app::{App, PipelineStageState, PipelineStatus};
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(3), // Input/Output info
        Constraint::Length(8), // Stages
        Constraint::Length(3), // Progress bar
        Constraint::Min(5),    // Logs
    ])
    .split(area);

    render_info(frame, app, chunks[0]);
    render_stages(frame, app, chunks[1]);
    render_progress_bar(frame, app, chunks[2]);
    render_logs(frame, app, chunks[3]);
}

fn render_info(frame: &mut Frame, app: &App, area: Rect) {
    let input = if app.input_paths.is_empty() {
        "Aucun fichier selectionne".to_string()
    } else if app.input_paths.len() == 1 {
        app.input_paths[0].display().to_string()
    } else {
        format!("{} fichier(s)", app.input_paths.len())
    };

    let line = Line::from(vec![
        Span::styled("  Input: ", theme::muted()),
        Span::styled(input, Style::default().fg(theme::SUCCESS)),
        Span::styled("    Output: ", theme::muted()),
        Span::styled(
            app.output_dir.display().to_string(),
            Style::default().fg(theme::TEXT),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(line).block(block);
    frame.render_widget(paragraph, area);
}

fn render_stages(frame: &mut Frame, app: &App, area: Rect) {
    let ps = &app.pipeline_state;

    let stages = [
        ("Lecture du fichier", &ps.reading),
        ("Raffinement LLM", &ps.refining),
        ("Generation des tests", &ps.generating),
        ("Ecriture des sorties", &ps.writing),
    ];

    let mut lines = Vec::new();
    for (name, state) in &stages {
        let (icon, style) = match state {
            PipelineStageState::Pending => ("○", theme::muted()),
            PipelineStageState::Running => (
                "⟳",
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            PipelineStageState::Done => ("✓", theme::status_ok()),
            PipelineStageState::Failed(_) => ("✗", theme::status_error()),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {icon} "), style),
            Span::styled(name.to_string(), style),
        ]));

        // Show extra info for running stage
        if matches!(state, PipelineStageState::Running) {
            if !ps.progress_message.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("      ", Style::default()),
                    Span::styled(format!("└ {}", ps.progress_message), theme::muted()),
                ]));
            }
            let elapsed = ps.elapsed_secs();
            let elapsed_str = if ps.tokens_used > 0 {
                format!("  Tokens: ~{}  Duree: {}s", ps.tokens_used, elapsed)
            } else {
                format!("  Duree: {}s ...", elapsed)
            };
            lines.push(Line::from(vec![
                Span::styled("      ", Style::default()),
                Span::styled(elapsed_str, theme::muted()),
            ]));
        }
    }

    let block = Block::default()
        .title(" Etapes ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn render_progress_bar(frame: &mut Frame, app: &App, area: Rect) {
    let progress = app.pipeline_state.progress_percent();
    let label = match app.pipeline_state.status {
        PipelineStatus::Idle => "En attente...".to_string(),
        PipelineStatus::Running => format!("{progress}%"),
        PipelineStatus::Done => "Termine!".to_string(),
        PipelineStatus::Error => "Erreur!".to_string(),
    };

    let color = match app.pipeline_state.status {
        PipelineStatus::Done => theme::SUCCESS,
        PipelineStatus::Error => theme::ERROR,
        _ => theme::PRIMARY,
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::border()),
        )
        .gauge_style(Style::default().fg(color))
        .percent(progress)
        .label(label);
    frame.render_widget(gauge, area);
}

fn render_logs(frame: &mut Frame, app: &mut App, area: Rect) {
    let logs = &app.pipeline_state.logs;
    let content_width = area.width.saturating_sub(4) as usize;
    let visible_height = area.height.saturating_sub(2) as usize;

    let lines: Vec<Line> = logs
        .iter()
        .map(|log| Line::from(Span::styled(log.clone(), theme::muted())))
        .collect();

    // Estimer les lignes visuelles (avec wrapping)
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

    let scroll_y = if app.pipeline_logs_auto_scroll {
        max_scroll
    } else {
        app.pipeline_logs_scroll = app.pipeline_logs_scroll.min(max_scroll);
        app.pipeline_logs_scroll
    };

    let block = Block::default()
        .title(" Logs [↑↓ scroll] ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll_y as u16, 0));
    frame.render_widget(paragraph, area);
}
