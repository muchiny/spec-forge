//! Ecran Config — affichage de la configuration actuelle

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let c = &app.config;
    let mut lines = Vec::new();

    // LLM Section
    section_header(&mut lines, "LLM");
    config_line(&mut lines, "Provider", &c.llm.provider);
    config_line(&mut lines, "Model", &c.llm.model_name);
    config_line(&mut lines, "URL", &c.llm.api_base_url);
    config_line(&mut lines, "Max tokens", &c.llm.max_tokens.to_string());
    config_line(&mut lines, "Temperature", &c.llm.temperature.to_string());
    config_line(&mut lines, "Timeout", &format!("{}s", c.llm.timeout_secs));
    config_line(
        &mut lines,
        "Enabled",
        if c.llm.enabled { "oui" } else { "non" },
    );
    lines.push(Line::from(""));

    // Pipeline Section
    section_header(&mut lines, "Pipeline");
    config_line(
        &mut lines,
        "Max retries",
        &c.pipeline.max_retries.to_string(),
    );
    config_line(&mut lines, "Langue", &c.pipeline.default_language);
    lines.push(Line::from(""));

    // Output Section
    section_header(&mut lines, "Output");
    config_line(&mut lines, "Format spec", &c.output.spec_format);
    config_line(&mut lines, "Gherkin lang", &c.output.gherkin_language);
    config_line(
        &mut lines,
        "Tracabilite",
        if c.output.traceability { "oui" } else { "non" },
    );
    lines.push(Line::from(""));

    // Validation Section
    section_header(&mut lines, "Validation");
    config_line(
        &mut lines,
        "Min coverage",
        &format!("{}%", c.validation.min_coverage_percent),
    );
    config_line(
        &mut lines,
        "Gherkin syntax",
        if c.validation.validate_gherkin_syntax {
            "oui"
        } else {
            "non"
        },
    );
    config_line(
        &mut lines,
        "Max clarifications",
        &c.validation.max_clarifications.to_string(),
    );
    lines.push(Line::from(""));

    // Paths Section
    section_header(&mut lines, "Paths");
    config_line(
        &mut lines,
        "Input",
        &c.paths.input_dir.display().to_string(),
    );
    config_line(
        &mut lines,
        "Output",
        &c.paths.output_dir.display().to_string(),
    );
    config_line(
        &mut lines,
        "Specs",
        &c.paths.specs_dir.display().to_string(),
    );
    config_line(
        &mut lines,
        "Features",
        &c.paths.features_dir.display().to_string(),
    );

    let scroll = app.config_scroll as u16;
    let block = Block::default()
        .title(" Configuration (lecture seule) ")
        .borders(Borders::ALL)
        .border_style(theme::border_active());
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn section_header(lines: &mut Vec<Line<'static>>, title: &str) {
    lines.push(Line::from(Span::styled(
        format!("  [{title}]"),
        theme::title(),
    )));
}

fn config_line(lines: &mut Vec<Line<'static>>, key: &str, value: &str) {
    lines.push(Line::from(vec![
        Span::styled(format!("    {key}: "), theme::muted()),
        Span::styled(value.to_string(), Style::default().fg(theme::TEXT)),
    ]));
}
