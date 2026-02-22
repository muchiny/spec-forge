//! Boucle d'evenements pour la TUI (crossterm + pipeline events)

use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind, MouseEvent};
use tokio::sync::mpsc;

use crate::application::pipeline_events::PipelineEvent;
use crate::tui::app::{LlmStatus, LogLevel};

/// Evenement applicatif
#[derive(Debug)]
pub enum AppEvent {
    /// Evenement clavier
    Key(KeyEvent),
    /// Evenement souris
    Mouse(MouseEvent),
    /// Tick de rafraichissement
    Tick,
    /// Evenement du pipeline
    Pipeline(Box<PipelineEvent>),
    /// Mise a jour du statut LLM (check asynchrone)
    LlmStatusUpdate(LlmStatus),
    /// Log provenant de tracing (bridge tracing -> TUI)
    TracingLog { level: LogLevel, message: String },
}

/// Gestionnaire d'evenements
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<AppEvent>,
    _tx: mpsc::UnboundedSender<AppEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Thread crossterm events
        let crossterm_tx = tx.clone();
        std::thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap_or(false) {
                    match event::read() {
                        Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                            if crossterm_tx.send(AppEvent::Key(key)).is_err() {
                                break;
                            }
                        }
                        Ok(Event::Mouse(mouse)) => {
                            if crossterm_tx.send(AppEvent::Mouse(mouse)).is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                } else if crossterm_tx.send(AppEvent::Tick).is_err() {
                    break;
                }
            }
        });

        Self { rx, _tx: tx }
    }

    /// Retourne le sender pour envoyer des events pipeline
    pub fn pipeline_sender(&self) -> mpsc::UnboundedSender<AppEvent> {
        self._tx.clone()
    }

    /// Attend le prochain evenement
    pub async fn next(&mut self) -> Option<AppEvent> {
        self.rx.recv().await
    }
}
