//! Ecran Dashboard — accueil avec status LLM et actions rapides

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::{App, LlmStatus, PipelineStatus};
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let has_results = app.spec.is_some();
    let chunks = if has_results {
        Layout::vertical([
            Constraint::Length(6), // LLM Status + Compliance
            Constraint::Length(9), // Actions rapides
            Constraint::Min(5),    // Derniere execution + couverture
        ])
        .split(area)
    } else {
        Layout::vertical([
            Constraint::Length(5), // LLM Status
            Constraint::Length(9), // Actions rapides
            Constraint::Min(5),    // Derniere execution
        ])
        .split(area)
    };

    render_llm_status(frame, app, chunks[0]);
    render_actions(frame, app, chunks[1]);
    render_last_run(frame, app, chunks[2]);
}

fn render_llm_status(frame: &mut Frame, app: &App, area: Rect) {
    let (indicator, status_text, style) = match &app.llm_status {
        LlmStatus::Ready => ("●", "Connecte", theme::status_ok()),
        LlmStatus::Checking => ("◌", "Verification...", theme::status_warning()),
        LlmStatus::Error(msg) => ("●", msg.as_str(), theme::status_error()),
        LlmStatus::Unknown => ("○", "Non verifie", theme::muted()),
    };

    let model = &app.config.llm.model_name;
    let provider = &app.config.llm.provider;
    let lang = &app.config.output.gherkin_language;

    let mut text = vec![
        Line::from(vec![
            Span::styled(format!("  LLM: {indicator} "), style),
            Span::styled(
                format!("{provider} ({model})"),
                Style::default().fg(theme::TEXT),
            ),
            Span::styled(format!(" - {status_text}"), style),
        ]),
        Line::from(vec![
            Span::styled("  Langue: ", theme::muted()),
            Span::styled(lang.to_uppercase(), Style::default().fg(theme::TEXT)),
            Span::styled("    Sortie: ", theme::muted()),
            Span::styled(
                app.output_dir.display().to_string(),
                Style::default().fg(theme::TEXT),
            ),
        ]),
    ];

    // Compliance profile indicator
    let profile = &app.config.compliance.profile;
    let compliance_text = format!("  Conformite: {}", profile.to_uppercase());
    if let Some(ref matrix) = app.cached_traceability {
        let pct = matrix.summary.forward_coverage_pct;
        let cov_style = if pct >= 80.0 {
            theme::status_ok()
        } else if pct >= 50.0 {
            theme::status_warning()
        } else {
            theme::status_error()
        };
        text.push(Line::from(vec![
            Span::styled(compliance_text, Style::default().fg(theme::PRIMARY)),
            Span::styled(format!("    Couverture: {:.0}%", pct), cov_style),
        ]));
    } else {
        text.push(Line::from(vec![Span::styled(
            compliance_text,
            Style::default().fg(theme::PRIMARY),
        )]));
    }

    let block = Block::default()
        .title(" Statut ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_actions(frame: &mut Frame, app: &App, area: Rect) {
    let can_run =
        !app.input_paths.is_empty() && app.pipeline_state.status != PipelineStatus::Running;

    let mut lines = vec![
        Line::from(""),
        make_action_line("Enter", "Lancer le pipeline", can_run),
        make_action_line("f", "Choisir un/des fichier(s)", true),
        make_action_line("c", "Configuration", true),
        make_action_line("?", "Aide", true),
    ];

    if !app.input_paths.is_empty() {
        lines.push(Line::from(""));
        if app.input_paths.len() == 1 {
            lines.push(Line::from(vec![
                Span::styled("  Fichier: ", theme::muted()),
                Span::styled(
                    app.input_paths[0].display().to_string(),
                    Style::default()
                        .fg(theme::SUCCESS)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("  Fichiers: ", theme::muted()),
                Span::styled(
                    format!("{} selectionne(s)", app.input_paths.len()),
                    Style::default()
                        .fg(theme::SUCCESS)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            for path in &app.input_paths {
                lines.push(Line::from(vec![
                    Span::styled("    - ", theme::muted()),
                    Span::styled(
                        path.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default(),
                        Style::default().fg(theme::SUCCESS),
                    ),
                ]));
            }
        }
    }

    let block = Block::default()
        .title(" Actions ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn render_last_run(frame: &mut Frame, app: &App, area: Rect) {
    let lines = if let Some(ref spec) = app.spec {
        let suite_info = app
            .test_suite
            .as_ref()
            .map(|s| {
                format!(
                    "{} features, {} scenarios, {:.0}% coverage",
                    s.features.len(),
                    s.total_scenarios,
                    s.coverage.coverage_percentage
                )
            })
            .unwrap_or_default();

        let mut result = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Specification: ", theme::muted()),
                Span::styled(&spec.title, Style::default().fg(theme::TEXT)),
            ]),
            Line::from(vec![
                Span::styled("  Scenarios: ", theme::muted()),
                Span::styled(
                    format!("{}", spec.user_scenarios.len()),
                    Style::default().fg(theme::TEXT),
                ),
                Span::styled("  Exigences: ", theme::muted()),
                Span::styled(
                    format!("{}", spec.functional_requirements.len()),
                    Style::default().fg(theme::TEXT),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Tests: ", theme::muted()),
                Span::styled(suite_info, Style::default().fg(theme::TEXT)),
            ]),
        ];

        // Coverage by priority
        if let Some(suite) = &app.test_suite {
            use crate::domain::user_story::Priority;
            let covered_ids: std::collections::HashSet<_> = suite
                .features
                .iter()
                .flat_map(|f| f.covered_requirements.iter())
                .collect();

            let mut p1_total = 0u32;
            let mut p1_covered = 0u32;
            let mut p2_total = 0u32;
            let mut p2_covered = 0u32;
            let mut p3_total = 0u32;
            let mut p3_covered = 0u32;

            for fr in &spec.functional_requirements {
                let is_covered = covered_ids.contains(&fr.id);
                match fr.priority {
                    Priority::P1 => {
                        p1_total += 1;
                        if is_covered {
                            p1_covered += 1;
                        }
                    }
                    Priority::P2 => {
                        p2_total += 1;
                        if is_covered {
                            p2_covered += 1;
                        }
                    }
                    Priority::P3 => {
                        p3_total += 1;
                        if is_covered {
                            p3_covered += 1;
                        }
                    }
                }
            }

            let fmt_pct = |covered: u32, total: u32| -> (String, Style) {
                if total == 0 {
                    return ("-".into(), theme::muted());
                }
                let pct = (covered as f32 / total as f32) * 100.0;
                let style = if pct >= 80.0 {
                    theme::status_ok()
                } else if pct >= 50.0 {
                    theme::status_warning()
                } else {
                    theme::status_error()
                };
                (format!("{:.0}%", pct), style)
            };

            let (p1_str, p1_style) = fmt_pct(p1_covered, p1_total);
            let (p2_str, p2_style) = fmt_pct(p2_covered, p2_total);
            let (p3_str, p3_style) = fmt_pct(p3_covered, p3_total);

            result.push(Line::from(vec![
                Span::styled("  Couverture: ", theme::muted()),
                Span::styled("P1:", Style::default().fg(theme::P1_COLOR)),
                Span::styled(format!("{p1_str} "), p1_style),
                Span::styled("P2:", Style::default().fg(theme::P2_COLOR)),
                Span::styled(format!("{p2_str} "), p2_style),
                Span::styled("P3:", Style::default().fg(theme::P3_COLOR)),
                Span::styled(p3_str, p3_style),
            ]));
        }

        result
    } else {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Aucune execution precedente",
                theme::muted(),
            )),
            Line::from(Span::styled(
                "  Selectionnez un fichier (f) puis lancez le pipeline (Enter)",
                theme::muted(),
            )),
        ]
    };

    let block = Block::default()
        .title(" Derniere execution ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn make_action_line(key: &str, desc: &str, enabled: bool) -> Line<'static> {
    if enabled {
        Line::from(vec![
            Span::styled(format!("  [{key}] "), theme::key_hint()),
            Span::styled(desc.to_string(), Style::default().fg(theme::TEXT)),
        ])
    } else {
        Line::from(vec![
            Span::styled(format!("  [{key}] "), theme::muted()),
            Span::styled(desc.to_string(), theme::muted()),
        ])
    }
}
