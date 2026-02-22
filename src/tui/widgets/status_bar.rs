//! Widget Status Bar — raccourcis contextuels en bas

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::tui::app::{App, Screen};
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Message de confirmation prioritaire
    if app.confirm_quit {
        let line = Line::from(vec![Span::styled(
            " Pipeline actif! Appuyez a nouveau sur q/Esc pour quitter ",
            theme::status_warning(),
        )]);
        frame.render_widget(Paragraph::new(line), area);
        return;
    }

    let common = [("q", "quitter"), ("?", "aide"), ("1-8", "naviguer")];

    let contextual: Vec<(&str, &str)> = match app.screen {
        Screen::Dashboard => vec![("Enter", "lancer"), ("f", "fichier")],
        Screen::FilePicker => vec![("Enter", "selectionner"), ("Bksp", "parent")],
        Screen::Pipeline => vec![("Enter", "lancer")],
        Screen::SpecViewer => vec![("Tab", "onglet"), ("↑↓", "scroll")],
        Screen::GherkinViewer => vec![("←→", "feature"), ("↑↓", "scroll")],
        Screen::Traceability => vec![("↑↓", "scroll")],
        Screen::Config => vec![("↑↓", "scroll")],
        Screen::Logs => vec![("↑↓", "scroll"), ("a", "auto-scroll")],
    };

    let mut spans = Vec::new();
    spans.push(Span::styled(" ", theme::muted()));

    for (key, desc) in contextual.iter().chain(common.iter()) {
        spans.push(Span::styled(format!(" {key}"), theme::key_hint()));
        spans.push(Span::styled(format!(":{desc} "), theme::key_desc()));
    }

    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);
}
