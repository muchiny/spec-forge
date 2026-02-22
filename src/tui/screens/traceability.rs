//! Ecran Traceability — matrice de tracabilite

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};

use crate::domain::traceability::TraceabilityStatus;
use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let (Some(matrix), Some(suite)) = (&app.cached_traceability, &app.test_suite) else {
        let msg = Paragraph::new("Aucune donnee de tracabilite. Lancez le pipeline d'abord.")
            .block(
                Block::default()
                    .title(" Tracabilite ")
                    .borders(Borders::ALL)
                    .border_style(theme::border()),
            );
        frame.render_widget(msg, area);
        return;
    };

    let chunks = Layout::vertical([Constraint::Min(10), Constraint::Length(9)]).split(area);

    // Table with enriched columns
    let header = Row::new(vec![
        Cell::from("Exigence").style(theme::header()),
        Cell::from("Priorite").style(theme::header()),
        Cell::from("Risque").style(theme::header()),
        Cell::from("Verification").style(theme::header()),
        Cell::from("Feature").style(theme::header()),
        Cell::from("Scen.").style(theme::header()),
        Cell::from("Statut").style(theme::header()),
    ])
    .height(1);

    let mut rows = Vec::new();
    for entry in &matrix.entries {
        let risk = entry
            .risk_level
            .as_ref()
            .map(|r| r.to_string())
            .unwrap_or_else(|| "-".into());

        let features = if entry.covering_features.is_empty() {
            "-".into()
        } else {
            entry.covering_features.join(", ")
        };

        let status_cell = match entry.status {
            TraceabilityStatus::NotCovered => Cell::from("GAP").style(
                Style::default()
                    .fg(theme::ERROR)
                    .add_modifier(Modifier::BOLD),
            ),
            TraceabilityStatus::FullyCovered => Cell::from("Couvert").style(theme::status_ok()),
            TraceabilityStatus::PartiallyCovered => {
                Cell::from("Partiel").style(theme::status_warning())
            }
            _ => Cell::from(entry.status.to_string()).style(Style::default().fg(theme::TEXT)),
        };

        rows.push(Row::new(vec![
            Cell::from(entry.requirement_id.clone()),
            Cell::from(entry.priority.to_string()),
            Cell::from(risk),
            Cell::from(entry.verification_method.to_string()),
            Cell::from(features),
            Cell::from(format!("{}", entry.covering_scenarios.len())),
            status_cell,
        ]));
    }

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(5),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Percentage(30),
            Constraint::Length(6),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Matrice de Tracabilite (ISO 29148) ")
            .borders(Borders::ALL)
            .border_style(theme::border_active()),
    )
    .row_highlight_style(theme::selected());

    let mut table_state = TableState::default();
    let offset = app.traceability_viewer.scroll_offset;
    if offset > 0 {
        table_state.select(Some(offset.min(matrix.entries.len().saturating_sub(1))));
    }
    frame.render_stateful_widget(table, chunks[0], &mut table_state);

    // Summary with traceability matrix data
    let summary = &matrix.summary;
    let fwd_pct = summary.forward_coverage_pct;
    let cov_style = if fwd_pct >= 80.0 {
        theme::status_ok()
    } else if fwd_pct >= 50.0 {
        theme::status_warning()
    } else {
        theme::status_error()
    };

    let cov = &suite.coverage;
    let orphan_info = if summary.orphan_tests.is_empty() {
        String::new()
    } else {
        format!("  Orphelins: {}", summary.orphan_tests.len())
    };

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Couverture forward: ", theme::muted()),
            Span::styled(format!("{:.0}%", fwd_pct), cov_style),
            Span::styled(
                format!(
                    "  ({}/{} exigences)",
                    summary.covered + summary.partially_covered + summary.verified_other,
                    summary.total_requirements,
                ),
                Style::default().fg(theme::TEXT),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Couvert: ", theme::muted()),
            Span::styled(format!("{}", summary.covered), theme::status_ok()),
            Span::styled("  Partiel: ", theme::muted()),
            Span::styled(
                format!("{}", summary.partially_covered),
                theme::status_warning(),
            ),
            Span::styled("  GAP: ", theme::muted()),
            Span::styled(
                format!("{}", summary.not_covered),
                if summary.not_covered > 0 {
                    theme::status_error()
                } else {
                    theme::status_ok()
                },
            ),
            Span::styled("  Autre: ", theme::muted()),
            Span::styled(
                format!("{}", summary.verified_other),
                Style::default().fg(theme::TEXT),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Chemin nominal: ", theme::muted()),
            Span::styled(
                format!("{}", cov.scenarios_by_type.happy_path),
                Style::default().fg(theme::TEXT),
            ),
            Span::styled("  Cas limite: ", theme::muted()),
            Span::styled(
                format!("{}", cov.scenarios_by_type.edge_case),
                Style::default().fg(theme::TEXT),
            ),
            Span::styled("  Erreur: ", theme::muted()),
            Span::styled(
                format!("{}", cov.scenarios_by_type.error_scenario),
                Style::default().fg(theme::TEXT),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Total scenarios: ", theme::muted()),
            Span::styled(
                format!("{}", suite.total_scenarios),
                Style::default().fg(theme::TEXT),
            ),
            Span::styled(orphan_info, theme::status_warning()),
        ]),
    ];

    let block = Block::default()
        .title(" Resume ISO 29148 ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, chunks[1]);
}
