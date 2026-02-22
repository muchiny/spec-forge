//! Ecran File Picker — selecteur de fichier avec apercu

use std::path::Path;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::tui::app::{App, FileEntry};
use crate::tui::theme;

const SUPPORTED_EXTENSIONS: &[&str] = &["md", "yaml", "yml", "pdf", "docx"];

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

    let main_chunks = Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[0]);

    render_file_list(frame, app, main_chunks[0]);
    render_preview(frame, app, main_chunks[1]);
    render_status_bar(frame, app, chunks[1]);
}

fn render_file_list(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(" {} ", app.file_picker.current_dir.display());

    let items: Vec<ListItem> = app
        .file_picker
        .entries
        .iter()
        .map(|entry| {
            let is_selected = app.file_picker.selected_paths.contains(&entry.path);
            let checkbox = if entry.is_dir {
                "   "
            } else if is_selected {
                "[x]"
            } else {
                "[ ]"
            };

            let icon = if entry.is_dir {
                " 📁 "
            } else {
                match entry.extension.as_str() {
                    "md" => " 📝 ",
                    "yaml" | "yml" => " 📋 ",
                    "pdf" => " 📄 ",
                    "docx" => " 📃 ",
                    _ => "    ",
                }
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme::SUCCESS)
                    .add_modifier(Modifier::BOLD)
            } else if entry.is_dir {
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else if SUPPORTED_EXTENSIONS.contains(&entry.extension.as_str()) {
                Style::default().fg(theme::TEXT)
            } else {
                theme::muted()
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    checkbox.to_string(),
                    if is_selected {
                        Style::default().fg(theme::SUCCESS)
                    } else {
                        theme::muted()
                    },
                ),
                Span::raw(icon.to_string()),
                Span::styled(entry.name.clone(), style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme::border_active()),
        )
        .highlight_style(theme::selected())
        .highlight_symbol("▸ ");

    let mut state = ListState::default();
    state.select(Some(app.file_picker.selected_index));
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_preview(frame: &mut Frame, app: &App, area: Rect) {
    let content = app
        .file_picker
        .preview
        .as_deref()
        .unwrap_or("Selectionnez un fichier pour voir l'apercu");

    let block = Block::default()
        .title(" Apercu ")
        .borders(Borders::ALL)
        .border_style(theme::border());
    let paragraph = Paragraph::new(content.to_string())
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let count = app.file_picker.selected_paths.len();
    let text = if count == 0 {
        " Space: selectionner | Enter: ouvrir/confirmer | Backspace: remonter".to_string()
    } else {
        format!(
            " {} fichier(s) selectionne(s) | Space: (de)selectionner | Enter: confirmer",
            count
        )
    };
    let style = if count > 0 {
        Style::default().fg(theme::SUCCESS)
    } else {
        theme::muted()
    };
    let paragraph = Paragraph::new(Span::styled(text, style));
    frame.render_widget(paragraph, area);
}

/// Charge la liste des fichiers du repertoire courant
pub fn load_directory(dir: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    // Entree parent (..)
    if dir.parent().is_some() {
        entries.push(FileEntry {
            name: "..".into(),
            path: dir.parent().unwrap().to_path_buf(),
            is_dir: true,
            extension: String::new(),
        });
    }

    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return entries;
    };

    let mut dir_entries: Vec<FileEntry> = read_dir
        .filter_map(|e| e.ok())
        .map(|e| {
            let path = e.path();
            let is_dir = path.is_dir();
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            FileEntry {
                name,
                path,
                is_dir,
                extension,
            }
        })
        .filter(|e| !e.name.starts_with('.')) // Masquer fichiers caches
        .collect();

    // Trier : dossiers d'abord, puis fichiers
    dir_entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    entries.extend(dir_entries);
    entries
}

/// Charge un apercu du fichier (premieres lignes)
pub fn load_preview(path: &Path) -> Option<String> {
    if path.is_dir() {
        return None;
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    if !SUPPORTED_EXTENSIONS.contains(&ext) {
        return Some("Format non supporte".into());
    }

    // Les fichiers binaires ne peuvent pas etre lus comme texte
    match ext {
        "pdf" => Some("Apercu PDF non disponible\nSelectionnez pour traiter".into()),
        "docx" => Some("Apercu DOCX non disponible\nSelectionnez pour traiter".into()),
        _ => std::fs::read_to_string(path).ok().map(|content| {
            let lines: Vec<&str> = content.lines().take(30).collect();
            lines.join("\n")
        }),
    }
}
