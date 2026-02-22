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

    /// Applique un PipelineEvent a l'etat
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
