//! Widget Help — popup modale avec raccourcis clavier

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    // Center the popup (60% width, 70% height)
    let popup_area = centered_rect(60, 70, area);

    // Clear background
    frame.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from(Span::styled("  Raccourcis globaux", theme::title())),
        Line::from(""),
        help_line("q / Esc", "Quitter (ou fermer cette aide)"),
        help_line("Ctrl+C", "Quitter immediatement"),
        help_line("?", "Afficher/masquer cette aide"),
        help_line("1-8", "Naviguer vers un ecran"),
        help_line("Tab", "Ecran suivant"),
        help_line("Shift+Tab", "Ecran precedent"),
        Line::from(""),
        Line::from(Span::styled("  Accueil", theme::title())),
        Line::from(""),
        help_line("Enter", "Lancer le pipeline"),
        help_line("f", "Ouvrir le selecteur de fichier"),
        help_line("c", "Ouvrir la configuration"),
        Line::from(""),
        Line::from(Span::styled("  Selecteur de fichier", theme::title())),
        Line::from(""),
        help_line("↑/↓", "Naviguer dans la liste"),
        help_line("Enter", "Ouvrir dossier / selectionner fichier"),
        help_line("Backspace", "Remonter au dossier parent"),
        Line::from(""),
        Line::from(Span::styled("  Viewers (Spec/Gherkin)", theme::title())),
        Line::from(""),
        help_line("↑/↓", "Faire defiler le contenu"),
        help_line("PgUp/PgDn", "Defiler rapidement"),
        help_line("Tab", "Onglet suivant (Spec Viewer)"),
        help_line("←/→", "Feature precedente/suivante (Gherkin)"),
        Line::from(""),
        Line::from(Span::styled("  Journaux", theme::title())),
        Line::from(""),
        help_line("a", "Activer/desactiver l'auto-scroll"),
        help_line("↑/↓", "Faire defiler les logs"),
    ];

    let block = Block::default()
        .title(" Aide - Raccourcis clavier ")
        .borders(Borders::ALL)
        .border_style(theme::border_active());

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().bg(ratatui::style::Color::Black));

    frame.render_widget(paragraph, popup_area);
}

fn help_line(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("    {key:>15}  "), theme::key_hint()),
        Span::styled(desc.to_string(), Style::default().fg(theme::TEXT)),
    ])
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1]);

    horizontal[1]
}
