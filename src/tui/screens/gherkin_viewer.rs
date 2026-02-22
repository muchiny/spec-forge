//! Ecran Gherkin Viewer — affichage des .feature avec coloration syntaxique

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let Some(ref suite) = app.test_suite else {
        let msg = Paragraph::new("Aucun test genere. Lancez le pipeline d'abord.").block(
            Block::default()
                .title(" Gherkin ")
                .borders(Borders::ALL)
                .border_style(theme::border()),
        );
        frame.render_widget(msg, area);
        return;
    };

    if suite.features.is_empty() {
        let msg = Paragraph::new("Aucune feature generee.").block(
            Block::default()
                .title(" Gherkin ")
                .borders(Borders::ALL)
                .border_style(theme::border()),
        );
        frame.render_widget(msg, area);
        return;
    }

    let chunks =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area);

    // Feature list (left pane)
    let items: Vec<ListItem> = suite
        .features
        .iter()
        .map(|f| {
            ListItem::new(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(&f.name, Style::default().fg(theme::TEXT)),
                Span::styled(format!(" ({})", f.scenarios.len()), theme::muted()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Features ")
                .borders(Borders::ALL)
                .border_style(theme::border()),
        )
        .highlight_style(theme::selected())
        .highlight_symbol("▸ ");

    let mut state = ListState::default();
    state.select(Some(app.gherkin_viewer.selected_feature));
    frame.render_stateful_widget(list, chunks[0], &mut state);

    // Feature content with syntax highlighting (right pane)
    let content = app
        .feature_contents
        .get(app.gherkin_viewer.selected_feature)
        .cloned()
        .unwrap_or_default();

    let lines: Vec<Line> = content.lines().map(highlight_gherkin_line).collect();

    let scroll = app.gherkin_viewer.scroll_offset as u16;
    let block = Block::default()
        .title(" .feature ")
        .borders(Borders::ALL)
        .border_style(theme::border_active());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, chunks[1]);
}

fn highlight_gherkin_line(line: &str) -> Line<'static> {
    let trimmed = line.trim();
    let owned = line.to_string();

    // Comments
    if trimmed.starts_with('#') {
        return Line::from(Span::styled(
            owned,
            Style::default().fg(theme::GHERKIN_COMMENT),
        ));
    }

    // Tags
    if trimmed.starts_with('@') {
        return Line::from(Span::styled(owned, Style::default().fg(theme::GHERKIN_TAG)));
    }

    // Keywords FR + EN
    let keywords = [
        "Fonctionnalite:",
        "Feature:",
        "Contexte:",
        "Background:",
        "Scenario:",
        "Plan du Scenario:",
        "Scenario Outline:",
        "Exemples:",
        "Examples:",
        "Soit ",
        "Quand ",
        "Alors ",
        "Et ",
        "Mais ",
        "Given ",
        "When ",
        "Then ",
        "And ",
        "But ",
    ];

    for kw in &keywords {
        if let Some(rest) = trimmed.strip_prefix(kw) {
            let indent = owned.len() - trimmed.len();
            let indent_str = &owned[..indent];
            return Line::from(vec![
                Span::raw(indent_str.to_string()),
                Span::styled(
                    kw.to_string(),
                    Style::default()
                        .fg(theme::GHERKIN_KEYWORD)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(rest.to_string(), Style::default().fg(theme::TEXT)),
            ]);
        }
    }

    Line::from(Span::styled(owned, Style::default().fg(theme::TEXT)))
}
