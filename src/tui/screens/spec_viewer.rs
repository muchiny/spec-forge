//! Ecran Spec Viewer — affichage de la specification raffinee

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};

use crate::domain::user_story::Priority;
use crate::tui::app::App;
use crate::tui::theme;

const TAB_TITLES: &[&str] = &["Scenarios", "Exigences", "Entites", "Cas limites"];

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let Some(ref spec) = app.spec else {
        let msg = Paragraph::new("Aucune specification disponible. Lancez le pipeline d'abord.")
            .block(
                Block::default()
                    .title(" Specification ")
                    .borders(Borders::ALL)
                    .border_style(theme::border()),
            );
        frame.render_widget(msg, area);
        return;
    };

    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(5)]).split(area);

    // Tabs
    let titles: Vec<Line> = TAB_TITLES.iter().map(|t| Line::from(*t)).collect();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title(format!(" {} ", spec.title))
                .borders(Borders::ALL)
                .border_style(theme::border_active()),
        )
        .select(app.spec_viewer.selected_tab)
        .style(theme::tab_inactive())
        .highlight_style(theme::tab_active());
    frame.render_widget(tabs, chunks[0]);

    // Content based on tab
    match app.spec_viewer.selected_tab {
        0 => render_scenarios(frame, app, spec, chunks[1]),
        1 => render_requirements(frame, app, spec, chunks[1]),
        2 => render_entities(frame, app, spec, chunks[1]),
        3 => render_edge_cases(frame, app, spec, chunks[1]),
        _ => {}
    }
}

fn priority_style(priority: &Priority) -> Style {
    match priority {
        Priority::P1 => Style::default()
            .fg(theme::P1_COLOR)
            .add_modifier(Modifier::BOLD),
        Priority::P2 => Style::default().fg(theme::P2_COLOR),
        Priority::P3 => Style::default().fg(theme::P3_COLOR),
    }
}

fn render_scenarios(
    frame: &mut Frame,
    app: &App,
    spec: &crate::domain::specification::Specification,
    area: Rect,
) {
    let mut lines = Vec::new();
    for scenario in &spec.user_scenarios {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {} ", scenario.id),
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&scenario.title, theme::header()),
            Span::raw("  "),
            Span::styled(
                format!("{}", scenario.priority),
                priority_style(&scenario.priority),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(&scenario.description, Style::default().fg(theme::TEXT)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("    Priorite: ", theme::muted()),
            Span::styled(&scenario.why_priority, Style::default().fg(theme::TEXT)),
        ]));

        for (i, ac) in scenario.acceptance_scenarios.iter().enumerate() {
            lines.push(Line::from(vec![
                Span::styled(format!("    {}. ", i + 1), theme::muted()),
                Span::styled(
                    "Given ",
                    Style::default()
                        .fg(theme::GHERKIN_KEYWORD)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&ac.given),
            ]));
            lines.push(Line::from(vec![
                Span::styled("       ", Style::default()),
                Span::styled(
                    "When ",
                    Style::default()
                        .fg(theme::GHERKIN_KEYWORD)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&ac.when),
            ]));
            lines.push(Line::from(vec![
                Span::styled("       ", Style::default()),
                Span::styled(
                    "Then ",
                    Style::default()
                        .fg(theme::GHERKIN_KEYWORD)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&ac.then),
            ]));
        }
        lines.push(Line::from(""));
    }

    let scroll = app.spec_viewer.scroll_offset as u16;
    let block = Block::default()
        .title(format!(" {} scenario(s) ", spec.user_scenarios.len()))
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn render_requirements(
    frame: &mut Frame,
    app: &App,
    spec: &crate::domain::specification::Specification,
    area: Rect,
) {
    let mut lines = Vec::new();
    for fr in &spec.functional_requirements {
        // First line: ID, category, priority
        let mut first_line = vec![
            Span::styled(
                format!("  {} ", fr.id),
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("[{:?}] ", fr.category), theme::muted()),
            Span::styled(format!("{}", fr.priority), priority_style(&fr.priority)),
        ];

        // Risk level if present
        if let Some(ref risk) = fr.risk_level {
            let risk_style = match risk {
                crate::domain::specification::RiskLevel::High => theme::status_error(),
                crate::domain::specification::RiskLevel::Medium => theme::status_warning(),
                crate::domain::specification::RiskLevel::Low => Style::default().fg(theme::SUCCESS),
            };
            first_line.push(Span::styled(format!("  {risk}"), risk_style));
        }

        lines.push(Line::from(first_line));

        // Statement
        lines.push(Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(&fr.statement, Style::default().fg(theme::TEXT)),
        ]));

        // Verification method + quality characteristic on same line
        let mut meta_spans = vec![
            Span::styled("    Verification: ", theme::muted()),
            Span::styled(
                format!("{}", fr.verification_method),
                Style::default().fg(theme::TEXT),
            ),
        ];
        if let Some(ref qc) = fr.quality_characteristic {
            meta_spans.push(Span::styled("  ISO 25010: ", theme::muted()));
            meta_spans.push(Span::styled(
                format!("{qc}"),
                Style::default().fg(theme::SECONDARY),
            ));
        }
        lines.push(Line::from(meta_spans));

        lines.push(Line::from(""));
    }

    let scroll = app.spec_viewer.scroll_offset as u16;
    let block = Block::default()
        .title(format!(
            " {} exigence(s) ",
            spec.functional_requirements.len()
        ))
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn render_entities(
    frame: &mut Frame,
    app: &App,
    spec: &crate::domain::specification::Specification,
    area: Rect,
) {
    let mut lines = Vec::new();
    if spec.key_entities.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Aucune entite identifiee",
            theme::muted(),
        )));
    }
    for entity in &spec.key_entities {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {} ", entity.name),
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&entity.description, Style::default().fg(theme::TEXT)),
        ]));
        for attr in &entity.attributes {
            lines.push(Line::from(vec![
                Span::styled("    - ", theme::muted()),
                Span::raw(attr),
            ]));
        }
        lines.push(Line::from(""));
    }

    let scroll = app.spec_viewer.scroll_offset as u16;
    let block = Block::default()
        .title(format!(" {} entite(s) ", spec.key_entities.len()))
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn render_edge_cases(
    frame: &mut Frame,
    app: &App,
    spec: &crate::domain::specification::Specification,
    area: Rect,
) {
    let mut lines = Vec::new();
    if spec.edge_cases.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Aucun cas limite identifie",
            theme::muted(),
        )));
    }
    for ec in &spec.edge_cases {
        lines.push(Line::from(vec![
            Span::styled("  ● ", priority_style(&ec.severity)),
            Span::styled(&ec.description, Style::default().fg(theme::TEXT)),
        ]));
        if let Some(ref related) = ec.related_scenario {
            lines.push(Line::from(vec![
                Span::styled("    Lie a: ", theme::muted()),
                Span::styled(related, Style::default().fg(theme::PRIMARY)),
            ]));
        }
    }

    let scroll = app.spec_viewer.scroll_offset as u16;
    let block = Block::default()
        .title(format!(" {} cas limite(s) ", spec.edge_cases.len()))
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}
