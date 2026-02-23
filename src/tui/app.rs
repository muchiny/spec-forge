//! Etat applicatif de la TUI

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use tokio_util::sync::CancellationToken;

use crate::application::pipeline::Pipeline;
use crate::application::pipeline_events::PipelineEvent;
use crate::domain::specification::Specification;
use crate::domain::test_case::TestSuite;
use crate::domain::traceability::TraceabilityMatrix;
use crate::infrastructure::config::Config;

/// Ecran actif
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    FilePicker,
    Pipeline,
    SpecViewer,
    GherkinViewer,
    Traceability,
    Config,
    Logs,
}

impl Screen {
    pub fn label(&self) -> &'static str {
        match self {
            Screen::Dashboard => "Accueil",
            Screen::FilePicker => "Fichier",
            Screen::Pipeline => "Pipeline",
            Screen::SpecViewer => "Spec",
            Screen::GherkinViewer => "Gherkin",
            Screen::Traceability => "Tracabilite",
            Screen::Config => "Config",
            Screen::Logs => "Journaux",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Screen::Dashboard => "1",
            Screen::FilePicker => "2",
            Screen::Pipeline => "3",
            Screen::SpecViewer => "4",
            Screen::GherkinViewer => "5",
            Screen::Traceability => "6",
            Screen::Config => "7",
            Screen::Logs => "8",
        }
    }

    pub fn all() -> &'static [Screen] {
        &[
            Screen::Dashboard,
            Screen::FilePicker,
            Screen::Pipeline,
            Screen::SpecViewer,
            Screen::GherkinViewer,
            Screen::Traceability,
            Screen::Config,
            Screen::Logs,
        ]
    }
}

/// Statut de la connexion LLM
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmStatus {
    Unknown,
    Checking,
    Ready,
    Error(String),
}

/// Etape du pipeline en cours
#[derive(Debug, Clone)]
pub enum PipelineStageState {
    Pending,
    Running,
    Done,
    Failed(String),
}

/// Etat du pipeline
#[derive(Debug, Clone)]
pub struct PipelineState {
    pub status: PipelineStatus,
    pub reading: PipelineStageState,
    pub refining: PipelineStageState,
    pub generating: PipelineStageState,
    pub writing: PipelineStageState,
    pub logs: Vec<String>,
    pub started_at: Option<Instant>,
    pub tokens_used: usize,
    pub progress_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineStatus {
    Idle,
    Running,
    Done,
    Error,
}

impl Default for PipelineState {
    fn default() -> Self {
        Self {
            status: PipelineStatus::Idle,
            reading: PipelineStageState::Pending,
            refining: PipelineStageState::Pending,
            generating: PipelineStageState::Pending,
            writing: PipelineStageState::Pending,
            logs: Vec::new(),
            started_at: None,
            tokens_used: 0,
            progress_message: String::new(),
        }
    }
}

impl PipelineState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn progress_percent(&self) -> u16 {
        let mut done = 0u16;
        if matches!(self.reading, PipelineStageState::Done) {
            done += 25;
        }
        if matches!(self.refining, PipelineStageState::Done) {
            done += 25;
        }
        if matches!(self.generating, PipelineStageState::Done) {
            done += 25;
        }
        if matches!(self.writing, PipelineStageState::Done) {
            done += 25;
        }
        done
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.started_at.map(|s| s.elapsed().as_secs()).unwrap_or(0)
    }
}

/// Entree de log pour la TUI
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

/// Etat du file picker
#[derive(Debug, Clone)]
pub struct FilePickerState {
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub preview: Option<String>,
    pub selected_paths: HashSet<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub extension: String,
}

impl Default for FilePickerState {
    fn default() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            entries: Vec::new(),
            selected_index: 0,
            preview: None,
            selected_paths: HashSet::new(),
        }
    }
}

/// Etat des viewers (scroll)
#[derive(Debug, Clone, Default)]
pub struct ViewerState {
    pub scroll_offset: usize,
    pub selected_tab: usize,
    pub selected_feature: usize,
}

/// Etat principal de l'application TUI
pub struct App {
    pub screen: Screen,
    pub show_help: bool,
    pub config: Config,
    pub pipeline: Arc<Pipeline>,
    pub llm_status: LlmStatus,
    pub pipeline_state: PipelineState,
    pub spec: Option<Specification>,
    pub test_suite: Option<TestSuite>,
    pub feature_contents: Vec<String>,
    pub traceability_content: Option<String>,
    pub cached_traceability: Option<TraceabilityMatrix>,
    pub logs: Vec<LogEntry>,
    pub input_paths: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub should_quit: bool,
    pub file_picker: FilePickerState,
    pub spec_viewer: ViewerState,
    pub gherkin_viewer: ViewerState,
    pub traceability_viewer: ViewerState,
    pub config_scroll: usize,
    pub logs_scroll: usize,
    pub logs_auto_scroll: bool,
    pub pipeline_logs_scroll: usize,
    pub pipeline_logs_auto_scroll: bool,
    pub confirm_quit: bool,
    pub pipeline_cancel: CancellationToken,
    pub pipeline_handle: Option<tokio::task::JoinHandle<()>>,
}

impl App {
    pub fn new(config: Config, pipeline: Arc<Pipeline>) -> Self {
        let output_dir = config.paths.output_dir.clone();
        Self {
            screen: Screen::Dashboard,
            show_help: false,
            config,
            pipeline,
            llm_status: LlmStatus::Unknown,
            pipeline_state: PipelineState::default(),
            spec: None,
            test_suite: None,
            feature_contents: Vec::new(),
            traceability_content: None,
            cached_traceability: None,
            logs: Vec::new(),
            input_paths: Vec::new(),
            output_dir,
            should_quit: false,
            file_picker: FilePickerState::default(),
            spec_viewer: ViewerState::default(),
            gherkin_viewer: ViewerState::default(),
            traceability_viewer: ViewerState::default(),
            config_scroll: 0,
            logs_scroll: 0,
            logs_auto_scroll: true,
            pipeline_logs_scroll: 0,
            pipeline_logs_auto_scroll: true,
            confirm_quit: false,
            pipeline_cancel: CancellationToken::new(),
            pipeline_handle: None,
        }
    }

    /// Nombre maximum de logs conserves en memoire
    const MAX_LOGS: usize = 5000;

    pub fn add_log(&mut self, level: LogLevel, message: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.logs.push(LogEntry {
            timestamp,
            level,
            message,
        });
        // Limiter la taille des logs
        if self.logs.len() > Self::MAX_LOGS {
            let drain_count = self.logs.len() - Self::MAX_LOGS;
            self.logs.drain(..drain_count);
        }
        // Also add to pipeline logs if running
        if self.pipeline_state.status == PipelineStatus::Running
            && let Some(log) = self.logs.last()
        {
            self.pipeline_state
                .logs
                .push(format!("{} {}", log.timestamp, log.message));
            if self.pipeline_state.logs.len() > Self::MAX_LOGS {
                let drain_count = self.pipeline_state.logs.len() - Self::MAX_LOGS;
                self.pipeline_state.logs.drain(..drain_count);
            }
        }
    }

    /// Applique un PipelineEvent a l'etat de l'application
    pub fn handle_pipeline_event(&mut self, event: PipelineEvent) {
        use crate::application::pipeline_events::PipelineStage;

        match event {
            PipelineEvent::StageStarted(stage) => {
                let state = match stage {
                    PipelineStage::ReadingInput => &mut self.pipeline_state.reading,
                    PipelineStage::RefiningSpec => &mut self.pipeline_state.refining,
                    PipelineStage::GeneratingTests => &mut self.pipeline_state.generating,
                    PipelineStage::WritingOutput => &mut self.pipeline_state.writing,
                };
                *state = PipelineStageState::Running;
                self.add_log(LogLevel::Info, format!("{} demarre", stage.label()));
            }
            PipelineEvent::StageCompleted(stage) => {
                let state = match stage {
                    PipelineStage::ReadingInput => &mut self.pipeline_state.reading,
                    PipelineStage::RefiningSpec => &mut self.pipeline_state.refining,
                    PipelineStage::GeneratingTests => &mut self.pipeline_state.generating,
                    PipelineStage::WritingOutput => &mut self.pipeline_state.writing,
                };
                *state = PipelineStageState::Done;
                self.add_log(LogLevel::Info, format!("{} termine", stage.label()));
            }
            PipelineEvent::Progress { message, .. } => {
                self.pipeline_state.progress_message = message.clone();
                self.add_log(LogLevel::Info, message);
            }
            PipelineEvent::LlmCallStarted { prompt_tokens } => {
                self.add_log(
                    LogLevel::Info,
                    format!("Appel LLM ({prompt_tokens} tokens prompt)"),
                );
            }
            PipelineEvent::LlmCallCompleted {
                response_tokens,
                elapsed_ms,
            } => {
                self.pipeline_state.tokens_used += response_tokens;
                self.add_log(
                    LogLevel::Info,
                    format!(
                        "LLM repondu ({response_tokens} tokens, {:.1}s)",
                        elapsed_ms as f64 / 1000.0
                    ),
                );
            }
            PipelineEvent::FileWritten { path } => {
                self.add_log(LogLevel::Info, format!("Fichier ecrit: {}", path.display()));
            }
            PipelineEvent::Error(msg) => {
                self.pipeline_state.status = PipelineStatus::Error;
                self.add_log(LogLevel::Error, msg);
            }
            PipelineEvent::Completed {
                spec,
                test_suite,
                feature_contents,
                traceability_content,
            } => {
                self.spec = Some(*spec);
                self.test_suite = Some(*test_suite);
                self.feature_contents = feature_contents;
                self.traceability_content = traceability_content;
                // Calculer et cacher la matrice de tracabilite
                if let (Some(s), Some(t)) = (&self.spec, &self.test_suite) {
                    self.cached_traceability =
                        Some(crate::domain::traceability::build_traceability_matrix(s, t));
                }
                self.pipeline_state.status = PipelineStatus::Done;
                self.add_log(LogLevel::Info, "Pipeline termine avec succes!".into());
                // Auto-switch to spec viewer
                self.screen = Screen::SpecViewer;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::llm::mock_adapter::MockLlmAdapter;
    use crate::adapters::templates::file_template_engine::FileTemplateEngine;
    use crate::application::pipeline_events::{PipelineEvent, PipelineStage};
    use crate::domain::specification::Specification;
    use crate::domain::test_case::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    fn make_test_app() -> App {
        let config = Config::default();
        let llm: Arc<dyn crate::ports::llm_service::LlmService> =
            Arc::new(MockLlmAdapter::new(vec!["{}".into()]));
        let templates: Arc<dyn crate::ports::template_engine::TemplateEngine> = Arc::new(
            FileTemplateEngine::new(std::path::Path::new("templates")).expect("templates dir"),
        );
        let pipeline = Arc::new(Pipeline::new(llm, templates, config.clone()));
        App::new(config, pipeline)
    }

    #[test]
    fn test_screen_labels() {
        assert_eq!(Screen::Dashboard.label(), "Accueil");
        assert_eq!(Screen::FilePicker.label(), "Fichier");
        assert_eq!(Screen::Pipeline.label(), "Pipeline");
        assert_eq!(Screen::SpecViewer.label(), "Spec");
        assert_eq!(Screen::GherkinViewer.label(), "Gherkin");
        assert_eq!(Screen::Traceability.label(), "Tracabilite");
        assert_eq!(Screen::Config.label(), "Config");
        assert_eq!(Screen::Logs.label(), "Journaux");
    }

    #[test]
    fn test_screen_keys() {
        assert_eq!(Screen::Dashboard.key(), "1");
        assert_eq!(Screen::Logs.key(), "8");
    }

    #[test]
    fn test_screen_all() {
        let all = Screen::all();
        assert_eq!(all.len(), 8);
        assert_eq!(all[0], Screen::Dashboard);
        assert_eq!(all[7], Screen::Logs);
    }

    #[test]
    fn test_pipeline_state_default() {
        let state = PipelineState::default();
        assert_eq!(state.status, PipelineStatus::Idle);
        assert!(matches!(state.reading, PipelineStageState::Pending));
        assert!(matches!(state.refining, PipelineStageState::Pending));
        assert!(matches!(state.generating, PipelineStageState::Pending));
        assert!(matches!(state.writing, PipelineStageState::Pending));
        assert_eq!(state.tokens_used, 0);
        assert!(state.logs.is_empty());
    }

    #[test]
    fn test_pipeline_state_reset() {
        let mut state = PipelineState {
            status: PipelineStatus::Running,
            reading: PipelineStageState::Done,
            tokens_used: 500,
            ..Default::default()
        };
        state.logs.push("test".into());

        state.reset();

        assert_eq!(state.status, PipelineStatus::Idle);
        assert!(matches!(state.reading, PipelineStageState::Pending));
        assert_eq!(state.tokens_used, 0);
        assert!(state.logs.is_empty());
    }

    #[test]
    fn test_progress_percent() {
        let mut state = PipelineState::default();
        assert_eq!(state.progress_percent(), 0);

        state.reading = PipelineStageState::Done;
        assert_eq!(state.progress_percent(), 25);

        state.refining = PipelineStageState::Done;
        assert_eq!(state.progress_percent(), 50);

        state.generating = PipelineStageState::Done;
        assert_eq!(state.progress_percent(), 75);

        state.writing = PipelineStageState::Done;
        assert_eq!(state.progress_percent(), 100);
    }

    #[test]
    fn test_progress_running_not_counted() {
        let state = PipelineState {
            reading: PipelineStageState::Running,
            ..Default::default()
        };
        assert_eq!(
            state.progress_percent(),
            0,
            "Running ne compte pas comme Done"
        );
    }

    #[test]
    fn test_elapsed_secs_without_start() {
        let state = PipelineState::default();
        assert_eq!(state.elapsed_secs(), 0);
    }

    #[test]
    fn test_app_initial_state() {
        let app = make_test_app();
        assert_eq!(app.screen, Screen::Dashboard);
        assert!(!app.show_help);
        assert!(!app.should_quit);
        assert!(app.spec.is_none());
        assert!(app.test_suite.is_none());
        assert!(app.logs.is_empty());
        assert_eq!(app.llm_status, LlmStatus::Unknown);
        assert_eq!(app.pipeline_state.status, PipelineStatus::Idle);
    }

    #[test]
    fn test_add_log() {
        let mut app = make_test_app();
        app.add_log(LogLevel::Info, "Test message".into());

        assert_eq!(app.logs.len(), 1);
        assert_eq!(app.logs[0].level, LogLevel::Info);
        assert_eq!(app.logs[0].message, "Test message");
        assert!(!app.logs[0].timestamp.is_empty());
    }

    #[test]
    fn test_add_log_cap() {
        let mut app = make_test_app();
        for i in 0..5010 {
            app.add_log(LogLevel::Info, format!("Log {i}"));
        }
        assert!(
            app.logs.len() <= App::MAX_LOGS,
            "Logs ne doivent pas depasser MAX_LOGS"
        );
    }

    #[test]
    fn test_handle_stage_started() {
        let mut app = make_test_app();
        app.pipeline_state.status = PipelineStatus::Running;

        app.handle_pipeline_event(PipelineEvent::StageStarted(PipelineStage::ReadingInput));
        assert!(matches!(
            app.pipeline_state.reading,
            PipelineStageState::Running
        ));
    }

    #[test]
    fn test_handle_stage_completed() {
        let mut app = make_test_app();
        app.pipeline_state.status = PipelineStatus::Running;

        app.handle_pipeline_event(PipelineEvent::StageCompleted(PipelineStage::RefiningSpec));
        assert!(matches!(
            app.pipeline_state.refining,
            PipelineStageState::Done
        ));
    }

    #[test]
    fn test_handle_progress_message() {
        let mut app = make_test_app();
        app.handle_pipeline_event(PipelineEvent::Progress {
            stage: PipelineStage::GeneratingTests,
            message: "Batch 2/3".into(),
        });
        assert_eq!(app.pipeline_state.progress_message, "Batch 2/3");
    }

    #[test]
    fn test_handle_llm_tokens() {
        let mut app = make_test_app();
        app.pipeline_state.status = PipelineStatus::Running;

        app.handle_pipeline_event(PipelineEvent::LlmCallCompleted {
            response_tokens: 500,
            elapsed_ms: 2000,
        });
        assert_eq!(app.pipeline_state.tokens_used, 500);

        app.handle_pipeline_event(PipelineEvent::LlmCallCompleted {
            response_tokens: 300,
            elapsed_ms: 1500,
        });
        assert_eq!(app.pipeline_state.tokens_used, 800);
    }

    #[test]
    fn test_handle_error_sets_status() {
        let mut app = make_test_app();
        app.handle_pipeline_event(PipelineEvent::Error("Echec LLM".into()));
        assert_eq!(app.pipeline_state.status, PipelineStatus::Error);
    }

    #[test]
    fn test_handle_file_written() {
        let mut app = make_test_app();
        app.pipeline_state.status = PipelineStatus::Running;
        app.handle_pipeline_event(PipelineEvent::FileWritten {
            path: PathBuf::from("/tmp/test.md"),
        });
        assert!(app.logs.iter().any(|l| l.message.contains("/tmp/test.md")));
    }

    #[test]
    fn test_handle_completed_switches_screen() {
        let mut app = make_test_app();
        let spec = Box::new(Specification::new("Test".into()));
        let suite = Box::new(TestSuite {
            features: vec![],
            source_spec_id: spec.id,
            total_scenarios: 0,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        });

        app.handle_pipeline_event(PipelineEvent::Completed {
            spec,
            test_suite: suite,
            feature_contents: vec!["Feature 1".into()],
            traceability_content: Some("Trace".into()),
        });

        assert_eq!(app.pipeline_state.status, PipelineStatus::Done);
        assert_eq!(app.screen, Screen::SpecViewer);
        assert!(app.spec.is_some());
        assert!(app.test_suite.is_some());
        assert_eq!(app.feature_contents.len(), 1);
    }

    #[test]
    fn test_viewer_state_default() {
        let state = ViewerState::default();
        assert_eq!(state.scroll_offset, 0);
        assert_eq!(state.selected_tab, 0);
        assert_eq!(state.selected_feature, 0);
    }

    #[test]
    fn test_llm_status_variants() {
        assert_eq!(LlmStatus::Unknown, LlmStatus::Unknown);
        assert_eq!(LlmStatus::Ready, LlmStatus::Ready);
        assert_ne!(LlmStatus::Unknown, LlmStatus::Ready);
        assert_eq!(
            LlmStatus::Error("test".into()),
            LlmStatus::Error("test".into())
        );
    }
}
