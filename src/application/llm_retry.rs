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
    use pretty_assertions::assert_eq;

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

        assert!(
            result.is_err(),
            "Aurait du echouer apres epuisement des retries"
        );
        assert!(matches!(result.unwrap_err(), LlmRetryError::Failed { .. }));
    }

    #[tokio::test]
    async fn test_retry_succes_deuxieme_tentative() {
        // Premiere reponse : JSON invalide, deuxieme : valide
        let adapter = MockLlmAdapter::with_responses(vec![
            LlmResponse {
                content: "pas du json".into(),
                tokens_used: 10,
                finish_reason: FinishReason::Stop,
            },
            LlmResponse {
                content: r#"{"value": 42}"#.into(),
                tokens_used: 15,
                finish_reason: FinishReason::Stop,
            },
        ]);

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 1, None).await;

        assert!(
            result.is_ok(),
            "Aurait du reussir au 2e essai: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap()["value"], 42);
        assert_eq!(adapter.call_count(), 2);
    }

    #[tokio::test]
    async fn test_retry_epuise_max_tentatives() {
        // Toutes les reponses sont invalides
        let adapter = MockLlmAdapter::with_responses(vec![LlmResponse {
            content: "invalide".into(),
            tokens_used: 10,
            finish_reason: FinishReason::Stop,
        }]);

        #[derive(Debug, serde::Deserialize)]
        struct Strict {
            #[allow(dead_code)]
            field: String,
        }

        let result: Result<Strict, _> = call_with_retry(&adapter, "sys", "usr", 2, None).await;

        assert!(result.is_err());
        // 1 tentative initiale + 2 retries = 3 appels
        assert_eq!(adapter.call_count(), 3);
    }

    #[tokio::test]
    async fn test_retry_troncature_arrete_immediatement() {
        // FinishReason::Length ne doit PAS declencher de retry
        let adapter = MockLlmAdapter::with_responses(vec![
            LlmResponse {
                content: r#"{"value": 1}"#.into(),
                tokens_used: 100,
                finish_reason: FinishReason::Length,
            },
            LlmResponse {
                content: r#"{"value": 2}"#.into(),
                tokens_used: 50,
                finish_reason: FinishReason::Stop,
            },
        ]);

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 3, None).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LlmRetryError::Truncated { .. }
        ));
        // Un seul appel — pas de retry apres troncature
        assert_eq!(adapter.call_count(), 1);
    }

    #[tokio::test]
    async fn test_retry_validation_fn_rejecte_puis_accepte() {
        let adapter = MockLlmAdapter::with_responses(vec![
            LlmResponse {
                content: r#"{"count": 0}"#.into(),
                tokens_used: 10,
                finish_reason: FinishReason::Stop,
            },
            LlmResponse {
                content: r#"{"count": 5}"#.into(),
                tokens_used: 15,
                finish_reason: FinishReason::Stop,
            },
        ]);

        // Validate_fn rejette si count < 1
        let validate: &ValidateFn<serde_json::Value> = &|val, _attempt, _max| {
            if val["count"].as_i64().unwrap_or(0) < 1 {
                Some("count doit etre >= 1".into())
            } else {
                None
            }
        };

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 2, Some(validate)).await;

        assert!(
            result.is_ok(),
            "Aurait du reussir au 2e essai: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap()["count"], 5);
        assert_eq!(adapter.call_count(), 2);
    }

    #[tokio::test]
    async fn test_retry_validation_fn_rejette_toujours() {
        let adapter = MockLlmAdapter::with_responses(vec![LlmResponse {
            content: r#"{"count": 0}"#.into(),
            tokens_used: 10,
            finish_reason: FinishReason::Stop,
        }]);

        let validate: &ValidateFn<serde_json::Value> =
            &|_val, _attempt, _max| Some("toujours rejete".into());

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 1, Some(validate)).await;

        assert!(result.is_err());
        if let Err(LlmRetryError::Failed { details }) = result {
            assert!(
                details.contains("toujours rejete"),
                "Le message devrait contenir la raison du rejet: {details}"
            );
        }
    }

    #[tokio::test]
    async fn test_retry_json_invalide_puis_valide() {
        // Code block wrapping puis JSON propre
        let adapter = MockLlmAdapter::with_responses(vec![
            LlmResponse {
                content: "```json\nbroken json here\n```".into(),
                tokens_used: 10,
                finish_reason: FinishReason::Stop,
            },
            LlmResponse {
                content: r#"{"status": "ok"}"#.into(),
                tokens_used: 12,
                finish_reason: FinishReason::Stop,
            },
        ]);

        let result: Result<serde_json::Value, _> =
            call_with_retry(&adapter, "sys", "usr", 2, None).await;

        assert!(result.is_ok(), "Aurait du reussir: {:?}", result.err());
        assert_eq!(result.unwrap()["status"], "ok");
    }

    #[tokio::test]
    async fn test_retry_zero_max_retries() {
        // max_retries = 0 signifie une seule tentative
        let adapter = MockLlmAdapter::with_responses(vec![LlmResponse {
            content: "pas json".into(),
            tokens_used: 10,
            finish_reason: FinishReason::Stop,
        }]);

        #[derive(Debug, serde::Deserialize)]
        struct S {
            #[allow(dead_code)]
            x: i32,
        }

        let result: Result<S, _> = call_with_retry(&adapter, "sys", "usr", 0, None).await;

        assert!(result.is_err());
        assert_eq!(adapter.call_count(), 1);
    }
}
