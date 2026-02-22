//! Layer tracing personnalise pour la TUI
//!
//! Capture les evenements tracing (`info!`, `debug!`, `warn!`, etc.)
//! et les envoie vers le channel de la TUI pour affichage dans le panneau Logs.

use tokio::sync::mpsc;
use tracing::Subscriber;
use tracing::field::{Field, Visit};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

use super::app::LogLevel;
use super::event::AppEvent;

/// Layer tracing qui envoie les logs vers le channel TUI
pub struct TuiLogLayer {
    sender: mpsc::UnboundedSender<AppEvent>,
}

impl TuiLogLayer {
    pub fn new(sender: mpsc::UnboundedSender<AppEvent>) -> Self {
        Self { sender }
    }
}

impl<S: Subscriber> Layer<S> for TuiLogLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let level = match *event.metadata().level() {
            tracing::Level::ERROR => LogLevel::Error,
            tracing::Level::WARN => LogLevel::Warn,
            tracing::Level::INFO => LogLevel::Info,
            tracing::Level::DEBUG | tracing::Level::TRACE => LogLevel::Debug,
        };

        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);

        // Construire le message final : "message (champ1=val1, champ2=val2)"
        let message = if visitor.fields.is_empty() {
            visitor.message
        } else if visitor.message.is_empty() {
            visitor.fields.join(", ")
        } else {
            format!("{} ({})", visitor.message, visitor.fields.join(", "))
        };

        if !message.is_empty() {
            let _ = self.sender.send(AppEvent::TracingLog { level, message });
        }
    }
}

/// Visiteur qui extrait le message et les champs structures d'un evenement tracing
#[derive(Default)]
struct MessageVisitor {
    message: String,
    fields: Vec<String>,
}

impl Visit for MessageVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            let raw = format!("{:?}", value);
            // Retirer les guillemets si la valeur est une chaine
            if raw.starts_with('"') && raw.ends_with('"') {
                self.message = raw[1..raw.len() - 1].to_string();
            } else {
                self.message = raw;
            }
        } else {
            self.fields.push(format!("{}={:?}", field.name(), value));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        } else {
            self.fields.push(format!("{}={}", field.name(), value));
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields.push(format!("{}={}", field.name(), value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields.push(format!("{}={}", field.name(), value));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.fields.push(format!("{}={:.2}", field.name(), value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields.push(format!("{}={}", field.name(), value));
    }
}
