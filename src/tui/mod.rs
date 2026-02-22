//! Module TUI — Interface terminal interactive (ratatui + crossterm)

pub mod app;
pub mod event;
pub mod log_layer;
pub mod screens;
pub mod theme;
pub mod ui;
pub mod widgets;

use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio_util::sync::CancellationToken;

use anyhow::Result;
use crossterm::ExecutableCommand;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;

use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::adapters::llm::ollama_adapter::OllamaAdapter;
use crate::adapters::output::gherkin_writer::GherkinWriter;
use crate::adapters::output::traceability_writer::TraceabilityWriter;
use crate::adapters::templates::file_template_engine::FileTemplateEngine;
use crate::application::pipeline::Pipeline;
use crate::application::pipeline_events::{PipelineEvent, PipelineStage};
use crate::domain::user_story::Language;
use crate::infrastructure::config::Config;
use crate::ports::llm_service::LlmService;

use app::{App, LlmStatus, LogLevel, PipelineStatus, Screen};
use event::{AppEvent, EventHandler};
use log_layer::TuiLogLayer;

/// Point d'entree de la TUI
pub async fn run(config: Config) -> Result<()> {
    // Init terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    // Panic hook pour restaurer le terminal en cas de crash
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create pipeline
    let llm = Arc::new(OllamaAdapter::new(config.llm.clone())?);
    let templates = Arc::new(FileTemplateEngine::new(&config.templates.directory)?);
    let pipeline = Arc::new(Pipeline::new(llm.clone(), templates, config.clone()));

    let mut app = App::new(config.clone(), pipeline);

    // Load initial file picker entries
    app.file_picker.entries = screens::file_picker::load_directory(&app.file_picker.current_dir);

    // Event loop
    let mut events = EventHandler::new(Duration::from_millis(250));
    let pipeline_sender = events.pipeline_sender();

    // Initialiser tracing avec le bridge vers la TUI
    // Tous les info!, debug!, warn! des services apparaitront dans le panneau Logs
    let tui_layer = TuiLogLayer::new(pipeline_sender.clone());
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("spec_forge=debug"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tui_layer)
        .init();

    // Check LLM status en arriere-plan (non-bloquant)
    app.llm_status = LlmStatus::Checking;
    let llm_check = llm.clone();
    let llm_sender = pipeline_sender.clone();
    tokio::spawn(async move {
        let status = if llm_check.is_ready().await {
            LlmStatus::Ready
        } else {
            LlmStatus::Error("Non accessible".into())
        };
        let _ = llm_sender.send(AppEvent::LlmStatusUpdate(status));
    });

    loop {
        // Draw
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        // Handle events
        let Some(event) = events.next().await else {
            break;
        };

        match event {
            AppEvent::Key(key) => {
                // Help overlay takes priority
                if app.show_help {
                    if matches!(
                        key.code,
                        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q')
                    ) {
                        app.show_help = false;
                    }
                    continue;
                }

                // Ctrl+C annule le pipeline et quitte
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.pipeline_cancel.cancel();
                    if let Some(handle) = app.pipeline_handle.take() {
                        handle.abort();
                    }
                    app.should_quit = true;
                    continue;
                }

                // Si confirmation en attente
                if app.confirm_quit {
                    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                        app.should_quit = true;
                    } else {
                        app.confirm_quit = false;
                    }
                    continue;
                }

                match key.code {
                    // Quit (avec confirmation si pipeline actif)
                    KeyCode::Char('q') | KeyCode::Esc => {
                        if app.pipeline_state.status == PipelineStatus::Running {
                            app.confirm_quit = true;
                        } else {
                            app.should_quit = true;
                        }
                    }

                    // Help
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }

                    // Screen navigation by number
                    KeyCode::Char('1') => app.screen = Screen::Dashboard,
                    KeyCode::Char('2') => {
                        app.screen = Screen::FilePicker;
                        app.file_picker.entries =
                            screens::file_picker::load_directory(&app.file_picker.current_dir);
                    }
                    KeyCode::Char('3') => app.screen = Screen::Pipeline,
                    KeyCode::Char('4') => app.screen = Screen::SpecViewer,
                    KeyCode::Char('5') => app.screen = Screen::GherkinViewer,
                    KeyCode::Char('6') => app.screen = Screen::Traceability,
                    KeyCode::Char('7') => app.screen = Screen::Config,
                    KeyCode::Char('8') => app.screen = Screen::Logs,

                    // Tab navigation (sauf SpecViewer ou Tab change les onglets)
                    KeyCode::Tab if app.screen != Screen::SpecViewer => {
                        let screens = Screen::all();
                        let current = screens.iter().position(|s| *s == app.screen).unwrap_or(0);
                        app.screen = screens[(current + 1) % screens.len()];
                    }
                    KeyCode::BackTab if app.screen != Screen::SpecViewer => {
                        let screens = Screen::all();
                        let current = screens.iter().position(|s| *s == app.screen).unwrap_or(0);
                        app.screen = screens[(current + screens.len() - 1) % screens.len()];
                    }

                    // Screen-specific keys
                    _ => handle_screen_key(&mut app, key.code, key.modifiers, &pipeline_sender),
                }
            }
            AppEvent::Pipeline(pipeline_event) => {
                app.handle_pipeline_event(*pipeline_event);
            }
            AppEvent::LlmStatusUpdate(status) => {
                let log = match &status {
                    LlmStatus::Ready => (LogLevel::Info, "LLM connecte".into()),
                    LlmStatus::Error(msg) => (LogLevel::Warn, format!("LLM: {msg}")),
                    _ => (LogLevel::Debug, "LLM statut mis a jour".into()),
                };
                app.llm_status = status;
                app.add_log(log.0, log.1);
            }
            AppEvent::TracingLog { level, message } => {
                app.add_log(level, message);
            }
            AppEvent::Tick => {
                // Tick — nothing special needed, terminal redraws
            }
            AppEvent::Mouse(_) => {
                // Mouse events — not handled for now
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn handle_screen_key(
    app: &mut App,
    code: KeyCode,
    _modifiers: KeyModifiers,
    pipeline_sender: &tokio::sync::mpsc::UnboundedSender<AppEvent>,
) {
    match app.screen {
        Screen::Dashboard => handle_dashboard_key(app, code, pipeline_sender),
        Screen::FilePicker => handle_file_picker_key(app, code),
        Screen::Pipeline => handle_pipeline_key(app, code, pipeline_sender),
        Screen::SpecViewer => handle_spec_viewer_key(app, code),
        Screen::GherkinViewer => handle_gherkin_viewer_key(app, code),
        Screen::Config => handle_scroll_key(&mut app.config_scroll, code),
        Screen::Logs => handle_logs_key(app, code),
        Screen::Traceability => handle_scroll_key(&mut app.traceability_viewer.scroll_offset, code),
    }
}

fn handle_dashboard_key(
    app: &mut App,
    code: KeyCode,
    sender: &tokio::sync::mpsc::UnboundedSender<AppEvent>,
) {
    match code {
        KeyCode::Char('f') => {
            app.screen = Screen::FilePicker;
            app.file_picker.entries =
                screens::file_picker::load_directory(&app.file_picker.current_dir);
        }
        KeyCode::Char('c') => {
            app.screen = Screen::Config;
        }
        KeyCode::Enter => {
            if app.input_paths.is_empty() {
                app.add_log(
                    LogLevel::Warn,
                    "Aucun fichier selectionne — appuyez sur [f] pour choisir".into(),
                );
            } else if app.pipeline_state.status == PipelineStatus::Running {
                app.add_log(LogLevel::Warn, "Pipeline deja en cours d'execution".into());
            } else {
                launch_pipeline(app, sender);
            }
        }
        _ => {}
    }
}

fn handle_file_picker_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            if app.file_picker.selected_index > 0 {
                app.file_picker.selected_index -= 1;
                update_preview(app);
            }
        }
        KeyCode::Down => {
            if app.file_picker.selected_index + 1 < app.file_picker.entries.len() {
                app.file_picker.selected_index += 1;
                update_preview(app);
            }
        }
        // Space: toggle selection du fichier/dossier courant
        KeyCode::Char(' ') => {
            if let Some(entry) = app
                .file_picker
                .entries
                .get(app.file_picker.selected_index)
                .cloned()
                && !entry.is_dir
            {
                if app.file_picker.selected_paths.contains(&entry.path) {
                    app.file_picker.selected_paths.remove(&entry.path);
                } else {
                    app.file_picker.selected_paths.insert(entry.path);
                }
            }
        }
        KeyCode::Enter => {
            if let Some(entry) = app
                .file_picker
                .entries
                .get(app.file_picker.selected_index)
                .cloned()
            {
                if entry.is_dir {
                    // Navigate into directory
                    app.file_picker.current_dir = entry.path;
                    app.file_picker.entries =
                        screens::file_picker::load_directory(&app.file_picker.current_dir);
                    app.file_picker.selected_index = 0;
                    app.file_picker.preview = None;
                } else if app.file_picker.selected_paths.is_empty() {
                    // No multi-selection: select single file (legacy behavior)
                    app.input_paths = vec![entry.path];
                    app.add_log(
                        LogLevel::Info,
                        format!("Fichier selectionne: {}", app.input_paths[0].display()),
                    );
                    app.file_picker.selected_paths.clear();
                    app.screen = Screen::Dashboard;
                } else {
                    // Confirm multi-selection
                    app.input_paths = app.file_picker.selected_paths.iter().cloned().collect();
                    app.input_paths.sort();
                    app.add_log(
                        LogLevel::Info,
                        format!("{} fichier(s) selectionne(s)", app.input_paths.len()),
                    );
                    app.file_picker.selected_paths.clear();
                    app.screen = Screen::Dashboard;
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(parent) = app.file_picker.current_dir.parent() {
                app.file_picker.current_dir = parent.to_path_buf();
                app.file_picker.entries =
                    screens::file_picker::load_directory(&app.file_picker.current_dir);
                app.file_picker.selected_index = 0;
                app.file_picker.preview = None;
            }
        }
        _ => {}
    }
}

fn update_preview(app: &mut App) {
    if let Some(entry) = app.file_picker.entries.get(app.file_picker.selected_index) {
        app.file_picker.preview = screens::file_picker::load_preview(&entry.path);
    }
}

fn handle_pipeline_key(
    app: &mut App,
    code: KeyCode,
    sender: &tokio::sync::mpsc::UnboundedSender<AppEvent>,
) {
    match code {
        KeyCode::Enter => {
            if !app.input_paths.is_empty() && app.pipeline_state.status != PipelineStatus::Running {
                launch_pipeline(app, sender);
            }
        }
        KeyCode::Up => {
            app.pipeline_logs_auto_scroll = false;
            app.pipeline_logs_scroll = app.pipeline_logs_scroll.saturating_sub(1);
        }
        KeyCode::Down => {
            app.pipeline_logs_auto_scroll = false;
            app.pipeline_logs_scroll = app.pipeline_logs_scroll.saturating_add(1);
        }
        KeyCode::PageUp => {
            app.pipeline_logs_auto_scroll = false;
            app.pipeline_logs_scroll = app.pipeline_logs_scroll.saturating_sub(20);
        }
        KeyCode::PageDown => {
            app.pipeline_logs_auto_scroll = false;
            app.pipeline_logs_scroll += 20;
        }
        KeyCode::Home => {
            app.pipeline_logs_auto_scroll = false;
            app.pipeline_logs_scroll = 0;
        }
        KeyCode::End | KeyCode::Char('a') => {
            app.pipeline_logs_auto_scroll = true;
        }
        _ => {}
    }
}

fn handle_spec_viewer_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.spec_viewer.scroll_offset = app.spec_viewer.scroll_offset.saturating_sub(1);
        }
        KeyCode::Down => {
            app.spec_viewer.scroll_offset = app.spec_viewer.scroll_offset.saturating_add(1);
        }
        KeyCode::PageUp => {
            app.spec_viewer.scroll_offset = app.spec_viewer.scroll_offset.saturating_sub(20);
        }
        KeyCode::PageDown => {
            app.spec_viewer.scroll_offset += 20;
        }
        KeyCode::Home => {
            app.spec_viewer.scroll_offset = 0;
        }
        KeyCode::Tab => {
            app.spec_viewer.selected_tab = (app.spec_viewer.selected_tab + 1) % 4;
            app.spec_viewer.scroll_offset = 0;
        }
        KeyCode::BackTab => {
            app.spec_viewer.selected_tab = (app.spec_viewer.selected_tab + 3) % 4;
            app.spec_viewer.scroll_offset = 0;
        }
        _ => {}
    }
}

fn handle_gherkin_viewer_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.gherkin_viewer.scroll_offset = app.gherkin_viewer.scroll_offset.saturating_sub(1);
        }
        KeyCode::Down => {
            app.gherkin_viewer.scroll_offset = app.gherkin_viewer.scroll_offset.saturating_add(1);
        }
        KeyCode::PageUp => {
            app.gherkin_viewer.scroll_offset = app.gherkin_viewer.scroll_offset.saturating_sub(20);
        }
        KeyCode::PageDown => {
            app.gherkin_viewer.scroll_offset += 20;
        }
        KeyCode::Home => {
            app.gherkin_viewer.scroll_offset = 0;
        }
        KeyCode::Left => {
            if app.gherkin_viewer.selected_feature > 0 {
                app.gherkin_viewer.selected_feature -= 1;
                app.gherkin_viewer.scroll_offset = 0;
            }
        }
        KeyCode::Right => {
            let max = app.feature_contents.len().saturating_sub(1);
            if app.gherkin_viewer.selected_feature < max {
                app.gherkin_viewer.selected_feature += 1;
                app.gherkin_viewer.scroll_offset = 0;
            }
        }
        _ => {}
    }
}

fn handle_scroll_key(scroll: &mut usize, code: KeyCode) {
    match code {
        KeyCode::Up => {
            *scroll = scroll.saturating_sub(1);
        }
        KeyCode::Down => {
            *scroll = scroll.saturating_add(1);
        }
        KeyCode::PageUp => {
            *scroll = scroll.saturating_sub(20);
        }
        KeyCode::PageDown => {
            *scroll += 20;
        }
        KeyCode::Home => {
            *scroll = 0;
        }
        _ => {}
    }
}

fn handle_logs_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.logs_auto_scroll = false;
            // logs_scroll est clampe dans le render, donc saturating_sub fonctionne
            app.logs_scroll = app.logs_scroll.saturating_sub(1);
        }
        KeyCode::Down => {
            app.logs_auto_scroll = false;
            app.logs_scroll = app.logs_scroll.saturating_add(1);
            // sera clampe dans le render
        }
        KeyCode::PageUp => {
            app.logs_auto_scroll = false;
            app.logs_scroll = app.logs_scroll.saturating_sub(20);
        }
        KeyCode::PageDown => {
            app.logs_auto_scroll = false;
            app.logs_scroll += 20;
        }
        KeyCode::Home => {
            app.logs_auto_scroll = false;
            app.logs_scroll = 0;
        }
        KeyCode::End | KeyCode::Char('a') => {
            app.logs_auto_scroll = true;
        }
        _ => {}
    }
}

fn launch_pipeline(app: &mut App, sender: &tokio::sync::mpsc::UnboundedSender<AppEvent>) {
    if app.input_paths.is_empty() {
        return;
    }

    // Reset state
    app.pipeline_state.reset();
    app.pipeline_state.status = PipelineStatus::Running;
    app.pipeline_state.started_at = Some(Instant::now());
    app.pipeline_logs_scroll = 0;
    app.pipeline_logs_auto_scroll = true;
    app.screen = Screen::Pipeline;
    app.add_log(
        LogLevel::Info,
        format!("Pipeline demarre ({} fichier(s))", app.input_paths.len()),
    );

    let pipeline = Arc::clone(&app.pipeline);
    let input_paths = app.input_paths.clone();
    let output = app.output_dir.clone();
    let sender = sender.clone();
    let gherkin_lang = app.config.output.gherkin_language.clone();
    let traceability_enabled = app.config.output.traceability;

    // Token d'annulation pour arreter le pipeline proprement
    let cancel = CancellationToken::new();
    app.pipeline_cancel = cancel.clone();

    let cancel_sender = sender.clone();
    let handle = tokio::spawn(async move {
        tokio::select! {
            _ = cancel.cancelled() => {
                let _ = cancel_sender.send(AppEvent::Pipeline(Box::new(
                    PipelineEvent::Error("Pipeline annule par l'utilisateur".into())
                )));
            }
            _ = run_pipeline_async(pipeline, input_paths, output, gherkin_lang, traceability_enabled, sender) => {}
        }
    });
    app.pipeline_handle = Some(handle);
}

async fn run_pipeline_async(
    pipeline: Arc<Pipeline>,
    input_paths: Vec<std::path::PathBuf>,
    output: std::path::PathBuf,
    gherkin_lang: String,
    traceability_enabled: bool,
    sender: tokio::sync::mpsc::UnboundedSender<AppEvent>,
) {
    let send = |evt: PipelineEvent| {
        let _ = sender.send(AppEvent::Pipeline(Box::new(evt)));
    };

    // Stage 1: Read
    send(PipelineEvent::StageStarted(PipelineStage::ReadingInput));
    for path in &input_paths {
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        send(PipelineEvent::Progress {
            stage: PipelineStage::ReadingInput,
            message: format!("Lecture de {filename}..."),
        });
    }
    let story_set = match pipeline.read_stories_multi(&input_paths).await {
        Ok(s) => {
            let lang = if s
                .stories
                .first()
                .is_some_and(|st| st.raw_text.contains("En tant que"))
            {
                "FR"
            } else {
                "EN"
            };
            send(PipelineEvent::Progress {
                stage: PipelineStage::ReadingInput,
                message: format!(
                    "{} user stories trouvees depuis {} fichier(s) (langue: {lang})",
                    s.stories.len(),
                    input_paths.len()
                ),
            });
            send(PipelineEvent::StageCompleted(PipelineStage::ReadingInput));
            s
        }
        Err(e) => {
            send(PipelineEvent::Error(format!("Erreur lecture: {e}")));
            return;
        }
    };

    // Stage 2: Refine (use story_set directly to avoid double-read)
    send(PipelineEvent::StageStarted(PipelineStage::RefiningSpec));
    let prompt_tokens: usize = story_set.stories.iter().map(|s| s.raw_text.len() / 4).sum();
    send(PipelineEvent::LlmCallStarted { prompt_tokens });
    let model_name = pipeline.config().llm.model_name.clone();
    send(PipelineEvent::Progress {
        stage: PipelineStage::RefiningSpec,
        message: format!(
            "Envoi de {} stories au LLM ({model_name})...",
            story_set.stories.len()
        ),
    });

    let start = Instant::now();
    let specs_dir = output.join("specs");
    let spec = match pipeline.refine_stories(&story_set, &specs_dir, None).await {
        Ok(s) => {
            let elapsed = start.elapsed().as_millis() as u64;
            send(PipelineEvent::LlmCallCompleted {
                response_tokens: s.user_scenarios.len() * 200,
                elapsed_ms: elapsed,
            });
            send(PipelineEvent::Progress {
                stage: PipelineStage::RefiningSpec,
                message: format!(
                    "{} scenarios, {} exigences fonctionnelles",
                    s.user_scenarios.len(),
                    s.functional_requirements.len()
                ),
            });
            if !s.edge_cases.is_empty() {
                send(PipelineEvent::Progress {
                    stage: PipelineStage::RefiningSpec,
                    message: format!("{} cas limites identifies", s.edge_cases.len()),
                });
            }
            if !s.clarifications_needed.is_empty() {
                send(PipelineEvent::Progress {
                    stage: PipelineStage::RefiningSpec,
                    message: format!(
                        "{} clarifications en attente",
                        s.clarifications_needed.len()
                    ),
                });
            }
            if s.functional_requirements.is_empty() && !s.user_scenarios.is_empty() {
                send(PipelineEvent::Progress {
                    stage: PipelineStage::RefiningSpec,
                    message: "ATTENTION: 0 exigences — sortie LLM probablement tronquee (augmentez max_tokens)".into(),
                });
            }
            send(PipelineEvent::StageCompleted(PipelineStage::RefiningSpec));
            s
        }
        Err(e) => {
            send(PipelineEvent::Error(format!("Erreur raffinement: {e}")));
            return;
        }
    };

    // Stage 3: Generate tests
    send(PipelineEvent::StageStarted(PipelineStage::GeneratingTests));
    send(PipelineEvent::LlmCallStarted {
        prompt_tokens: spec.functional_requirements.len() * 100,
    });
    send(PipelineEvent::Progress {
        stage: PipelineStage::GeneratingTests,
        message: format!(
            "Generation Gherkin depuis {} exigences via LLM ({model_name})...",
            spec.functional_requirements.len()
        ),
    });

    let start = Instant::now();
    let features_dir = output.join("features");
    let suite = match pipeline.generate_tests(&spec, &features_dir).await {
        Ok(s) => {
            let elapsed = start.elapsed().as_millis() as u64;
            send(PipelineEvent::LlmCallCompleted {
                response_tokens: s.total_scenarios * 150,
                elapsed_ms: elapsed,
            });
            send(PipelineEvent::Progress {
                stage: PipelineStage::GeneratingTests,
                message: format!(
                    "{} features, {} scenarios, {:.0}% coverage",
                    s.features.len(),
                    s.total_scenarios,
                    s.coverage.coverage_percentage
                ),
            });
            for feature in &s.features {
                send(PipelineEvent::Progress {
                    stage: PipelineStage::GeneratingTests,
                    message: format!(
                        "  Feature '{}': {} scenarios",
                        feature.name,
                        feature.scenarios.len()
                    ),
                });
            }
            if s.coverage.requirements_covered.len() == s.coverage.requirements_total {
                send(PipelineEvent::Progress {
                    stage: PipelineStage::GeneratingTests,
                    message: "Toutes les exigences sont couvertes".into(),
                });
            } else {
                let uncovered =
                    s.coverage.requirements_total - s.coverage.requirements_covered.len();
                send(PipelineEvent::Progress {
                    stage: PipelineStage::GeneratingTests,
                    message: format!(
                        "ATTENTION: {uncovered} exigences non couvertes sur {}",
                        s.coverage.requirements_total
                    ),
                });
            }
            send(PipelineEvent::StageCompleted(
                PipelineStage::GeneratingTests,
            ));
            s
        }
        Err(e) => {
            send(PipelineEvent::Error(format!("Erreur generation: {e}")));
            return;
        }
    };

    // Stage 4: Write outputs
    send(PipelineEvent::StageStarted(PipelineStage::WritingOutput));

    send(PipelineEvent::Progress {
        stage: PipelineStage::WritingOutput,
        message: format!("Ecriture de {} features Gherkin...", suite.features.len()),
    });

    // Render feature contents for TUI display
    let language = Language::from_code(&gherkin_lang);
    let gherkin_writer = GherkinWriter::new(language);
    let feature_contents: Vec<String> = suite
        .features
        .iter()
        .map(|f| gherkin_writer.render_feature(f))
        .collect();

    // Render traceability
    let traceability_content = if traceability_enabled {
        send(PipelineEvent::Progress {
            stage: PipelineStage::WritingOutput,
            message: "Ecriture matrice de tracabilite...".into(),
        });
        let trace_writer = TraceabilityWriter::new();
        let content = trace_writer.render(&spec, &suite);
        if let Ok(path) = trace_writer.write(&spec, &suite, &output).await {
            send(PipelineEvent::FileWritten { path });
        }
        Some(content)
    } else {
        None
    };

    send(PipelineEvent::StageCompleted(PipelineStage::WritingOutput));

    // Send final result
    send(PipelineEvent::Completed {
        spec: Box::new(spec),
        test_suite: Box::new(suite),
        feature_contents,
        traceability_content,
    });
}
