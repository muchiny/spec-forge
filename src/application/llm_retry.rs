//! Logique commune de retry avec backoff pour les appels LLM
//!
//! Factorise le pattern retry + nettoyage JSON + parsing
//! utilise par `RefineService` et `GenerateTestsService`.

use serde::de::DeserializeOwned;
use tracing::{debug, info, warn};

use crate::application::json_utils;
use crate::ports::llm_service::{FinishReason, LlmService};

/// Erreur generique d'un appel LLM avec retry
#[derive(Debug, thiserror::Error)]
pub enum LlmRetryError {
    #[error("Sortie LLM tronquee (max_tokens atteint): {details}")]
    Truncated { details: String },

    #[error("Echec apres retries: {details}")]
    Failed { details: String },
}

/// Callback de validation post-parsing
pub type ValidateFn<T> = dyn Fn(&T, usize, usize) -> Option<String> + Send + Sync;

/// Appelle le LLM avec retry + backoff exponentiel, nettoyage JSON et parsing.
///
/// - `validate_fn` : callback optionnel pour valider la sortie parsee.
///   Retourne `Some(raison)` pour forcer un retry, `None` pour accepter.
pub async fn call_with_retry<T: DeserializeOwned>(
    llm: &dyn LlmService,
    system_prompt: &str,
    user_prompt: &str,
    max_retries: usize,
    validate_fn: Option<&ValidateFn<T>>,
) -> Result<T, LlmRetryError> {
    let mut last_error = String::new();

    for attempt in 0..=max_retries {
        if attempt > 0 {
            let delay = std::time::Duration::from_secs(2u64.pow(attempt as u32).min(30));
            info!(
                attempt,
                delay_secs = delay.as_secs(),
                "Retry LLM apres backoff"
            );
            tokio::time::sleep(delay).await;
        }

        match llm.generate_json(system_prompt, user_prompt).await {
            Ok(response) => {
                info!(
                    tokens = response.tokens_used,
                    finish = ?response.finish_reason,
                    response_len = response.content.len(),
                    "Reponse LLM recue"
                );

                // Detecter troncature — ne pas gaspiller les retries
                if response.finish_reason == FinishReason::Length {
                    warn!(
                        tokens = response.tokens_used,
                        response_len = response.content.len(),
                        "Sortie LLM tronquee (finish_reason=Length)"
                    );
                    return Err(LlmRetryError::Truncated {
                        details: format!(
                            "Reponse tronquee apres {} tokens ({} chars)",
                            response.tokens_used,
                            response.content.len()
                        ),
                    });
                }

                let cleaned = json_utils::clean_json_response(&response.content);
                debug!(
                    raw_len = response.content.len(),
                    cleaned_len = cleaned.len(),
                    "JSON nettoye"
                );

                match serde_json::from_str::<T>(&cleaned) {
                    Ok(output) => {
                        // Validation optionnelle post-parsing
                        if let Some(vf) = validate_fn
                            && let Some(reason) = vf(&output, attempt, max_retries)
                        {
                            last_error = reason.clone();
                            warn!(attempt, reason = %reason, "Validation post-parsing echouee — retry");
                            continue;
                        }
                        return Ok(output);
                    }
                    Err(e) => {
                        let preview = &cleaned[..cleaned.len().min(500)];
                        last_error = format!("Parsing JSON: {} | Reponse: {}", e, preview);
                        warn!(
                            error = %e,
                            attempt,
                            json_preview = %preview,
                            "Parsing JSON echoue"
                        );
                    }
                }
            }
            Err(e) => {
                last_error = format!("Appel LLM: {}", e);
                warn!(error = %e, attempt, "Appel LLM echoue");
            }
        }
    }

    Err(LlmRetryError::Failed {
        details: last_error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::llm::mock_adapter::MockLlmAdapter;
    use crate::ports::llm_service::LlmResponse;

    #[tokio::test]
    async fn test_call_with_retry_succes() {
        let json = r#"{"value": 42}"#;
        let adapter = MockLlmAdapter::with_responses(vec![LlmResponse {
            content: json.into(),
            tokens_used: 10,
            finish_reason: FinishReason::Stop,
        }]);

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 2, None).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["value"], 42);
    }

    #[tokio::test]
    async fn test_call_with_retry_troncature() {
        let adapter = MockLlmAdapter::with_responses(vec![LlmResponse {
            content: "{}".into(),
            tokens_used: 100,
            finish_reason: FinishReason::Length,
        }]);

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 2, None).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, LlmRetryError::Truncated { .. }));
    }

    #[tokio::test]
    async fn test_call_with_retry_parse_echoue() {
        let adapter = MockLlmAdapter::with_responses(vec![
            LlmResponse {
                content: "pas du json".into(),
                tokens_used: 10,
                finish_reason: FinishReason::Stop,
            },
            LlmResponse {
                content: "toujours pas".into(),
                tokens_used: 10,
                finish_reason: FinishReason::Stop,
            },
        ]);

        #[derive(Debug, serde::Deserialize)]
        struct Strict {
            #[allow(dead_code)]
            required: String,
        }

        let result: Result<Strict, _> = call_with_retry(&adapter, "sys", "usr", 1, None).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), LlmRetryError::Failed { .. }));
    }
}
