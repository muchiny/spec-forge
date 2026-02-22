//! Adapter Ollama - Implementation du LlmService pour Ollama local
//!
//! Porte depuis mcp-doc-rag avec ajout du support format:"json".

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::ports::llm_service::{
    FinishReason, LlmConfig, LlmError, LlmModelInfo, LlmResponse, LlmService,
};

/// Requete a l'API Ollama /api/generate
#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    /// Desactiver le mode thinking (Qwen3, DeepSeek-R1, etc.)
    /// `false` = pas de bloc `<think>...</think>` en sortie
    #[serde(skip_serializing_if = "Option::is_none")]
    think: Option<bool>,
    options: OllamaOptions,
}

/// Options de generation Ollama
#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: usize,
    num_ctx: usize,
}

/// Reponse de l'API Ollama /api/generate
#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    /// Contenu de reflexion du modele (si think=true)
    #[serde(default)]
    thinking: Option<String>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    eval_count: Option<usize>,
    /// Duree de generation en nanosecondes
    #[serde(default)]
    eval_duration: Option<u64>,
    #[serde(default)]
    done_reason: Option<String>,
}

/// Reponse de l'API Ollama /api/tags
#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

/// Modele Ollama
#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

/// Adapter pour Ollama (LLM local)
pub struct OllamaAdapter {
    client: Client,
    config: LlmConfig,
}

impl OllamaAdapter {
    /// Cree un nouvel adapter Ollama
    pub fn new(config: LlmConfig) -> Result<Self, LlmError> {
        if !config.enabled {
            return Err(LlmError::Disabled);
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| LlmError::ConnectionError(e.to_string()))?;

        info!(
            model = %config.model_name,
            url = %config.api_base_url,
            "OllamaAdapter initialise"
        );

        Ok(Self { client, config })
    }

    /// Verifie si le modele est disponible
    pub async fn check_model(&self) -> Result<bool, LlmError> {
        let url = format!("{}/api/tags", self.config.api_base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::ConnectionError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError {
                status_code: response.status().as_u16(),
                message: "Impossible de lister les modeles".to_string(),
            });
        }

        let tags: OllamaTagsResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let model_name = self
            .config
            .model_name
            .split(':')
            .next()
            .unwrap_or(&self.config.model_name);
        Ok(tags.models.iter().any(|m| m.name.starts_with(model_name)))
    }

    /// Appel interne a l'API generate
    async fn call_generate(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        json_format: bool,
    ) -> Result<LlmResponse, LlmError> {
        let url = format!("{}/api/generate", self.config.api_base_url);

        // Prepender /no_think au system prompt pour desactiver le mode thinking
        // (Qwen3, DeepSeek-R1). Le parametre think=false seul n'est pas fiable
        // (bug connu Ollama, cf. issues #11032, #12610, #12917).
        let effective_system = if system_prompt.is_empty() {
            if json_format {
                Some("/no_think".to_string())
            } else {
                None
            }
        } else if json_format {
            Some(format!("/no_think\n{system_prompt}"))
        } else {
            Some(system_prompt.to_string())
        };

        debug!(
            model = %self.config.model_name,
            prompt_len = user_prompt.len(),
            system_len = system_prompt.len(),
            json_format,
            temperature = self.config.temperature,
            max_tokens = self.config.max_tokens,
            num_ctx = self.config.context_size,
            "Envoi requete a Ollama"
        );

        // Log complet des prompts pour debug
        if let Some(sys) = &effective_system {
            debug!(
                system_prompt = %truncate_for_log(sys, 2000),
                "System prompt envoye"
            );
        }
        debug!(
            user_prompt = %truncate_for_log(user_prompt, 3000),
            "User prompt envoye"
        );

        let request = OllamaGenerateRequest {
            model: self.config.model_name.clone(),
            prompt: user_prompt.to_string(),
            stream: false,
            system: effective_system,
            format: if json_format {
                Some("json".to_string())
            } else {
                None
            },
            // Desactiver le mode thinking pour les requetes JSON structurees
            // Belt-and-suspenders : on utilise AUSSI think=false en plus de /no_think
            think: if json_format { Some(false) } else { None },
            options: OllamaOptions {
                temperature: self.config.temperature,
                num_predict: self.config.max_tokens,
                num_ctx: self.config.context_size,
            },
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    warn!(
                        timeout_secs = self.config.timeout_secs,
                        "Timeout Ollama — augmentez llm.timeout_secs dans config.yaml"
                    );
                    LlmError::Timeout(self.config.timeout_secs)
                } else {
                    warn!(
                        error = %e,
                        url = %url,
                        "Echec de connexion a Ollama"
                    );
                    LlmError::ConnectionError(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let mut error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Tronquer le corps d'erreur pour eviter les allocations excessives
            const MAX_ERROR_BODY: usize = 4096;
            if error_body.len() > MAX_ERROR_BODY {
                error_body.truncate(MAX_ERROR_BODY);
                error_body.push_str("... (tronque)");
            }

            warn!(status, error = %error_body, "Erreur API Ollama");

            return Err(LlmError::ApiError {
                status_code: status,
                message: error_body,
            });
        }

        let ollama_response: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let finish_reason = match ollama_response.done_reason.as_deref() {
            Some("stop") | None if ollama_response.done => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            _ => FinishReason::Stop,
        };

        // Diagnostics : detecter si le thinking est encore actif malgre think=false
        if let Some(ref thinking) = ollama_response.thinking
            && !thinking.is_empty()
        {
            warn!(
                thinking_len = thinking.len(),
                "Mode thinking encore actif malgre think=false — \
                 les tokens de reflexion ralentissent la generation"
            );
        }
        if ollama_response.response.contains("<think>") {
            warn!("Balises <think> detectees dans la reponse malgre think=false");
        }

        // Diagnostics : vitesse de generation
        let eval_count = ollama_response.eval_count.unwrap_or(0);
        if let Some(eval_duration) = ollama_response.eval_duration {
            if eval_duration > 0 {
                let tok_per_sec = eval_count as f64 / (eval_duration as f64 / 1e9);
                info!(
                    tokens = eval_count,
                    response_len = ollama_response.response.len(),
                    tok_per_sec = format!("{tok_per_sec:.1}"),
                    "Reponse Ollama recue"
                );
            } else {
                debug!(
                    tokens = eval_count,
                    response_len = ollama_response.response.len(),
                    "Reponse Ollama recue"
                );
            }
        } else {
            debug!(
                tokens = eval_count,
                response_len = ollama_response.response.len(),
                "Reponse Ollama recue"
            );
        }

        // Log du contenu complet de la reponse pour debug
        debug!(
            response_content = %truncate_for_log(&ollama_response.response, 5000),
            "Contenu reponse LLM"
        );

        Ok(LlmResponse {
            content: ollama_response.response,
            tokens_used: ollama_response.eval_count.unwrap_or(0),
            finish_reason,
        })
    }
}

/// Tronque un texte pour le logging (evite de noyer les logs)
fn truncate_for_log(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        text.to_string()
    } else {
        format!(
            "{}... [tronque, {} chars total]",
            &text[..max_chars],
            text.len()
        )
    }
}

#[async_trait]
impl LlmService for OllamaAdapter {
    async fn generate(&self, prompt: &str) -> Result<LlmResponse, LlmError> {
        self.call_generate("", prompt, false).await
    }

    async fn generate_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmResponse, LlmError> {
        self.call_generate(system_prompt, user_prompt, false).await
    }

    async fn generate_json(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmResponse, LlmError> {
        self.call_generate(system_prompt, user_prompt, true).await
    }

    async fn is_ready(&self) -> bool {
        let url = format!("{}/api/tags", self.config.api_base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    fn config(&self) -> &LlmConfig {
        &self.config
    }

    fn model_info(&self) -> LlmModelInfo {
        LlmModelInfo {
            name: self.config.model_name.clone(),
            provider: "ollama".to_string(),
            context_size: Some(self.config.context_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_adapter_disabled() {
        let config = LlmConfig {
            enabled: false,
            ..Default::default()
        };
        let result = OllamaAdapter::new(config);
        assert!(matches!(result, Err(LlmError::Disabled)));
    }

    #[test]
    fn test_ollama_adapter_creation() {
        let config = LlmConfig::default();
        let result = OllamaAdapter::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_request_json_desactive_thinking() {
        let request = OllamaGenerateRequest {
            model: "qwen3:8b".to_string(),
            prompt: "test".to_string(),
            stream: false,
            system: None,
            format: Some("json".to_string()),
            think: Some(false),
            options: OllamaOptions {
                temperature: 0.1,
                num_predict: 4096,
                num_ctx: 8192,
            },
        };
        let json = serde_json::to_value(&request).expect("Serialisation requete");
        assert_eq!(json["think"], serde_json::json!(false));
        assert_eq!(json["format"], serde_json::json!("json"));
        assert_eq!(json["options"]["num_ctx"], serde_json::json!(8192));
    }

    #[test]
    fn test_generate_request_texte_omet_thinking() {
        let request = OllamaGenerateRequest {
            model: "qwen3:8b".to_string(),
            prompt: "test".to_string(),
            stream: false,
            system: None,
            format: None,
            think: None,
            options: OllamaOptions {
                temperature: 0.1,
                num_predict: 4096,
                num_ctx: 8192,
            },
        };
        let json = serde_json::to_value(&request).expect("Serialisation requete");
        assert!(
            json.get("think").is_none(),
            "Le champ think ne doit pas etre present pour les requetes texte"
        );
        assert!(json.get("format").is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn test_ollama_is_ready() {
        let config = LlmConfig::default();
        let adapter = OllamaAdapter::new(config).unwrap();
        let ready = adapter.is_ready().await;
        println!("Ollama ready: {}", ready);
    }

    #[tokio::test]
    #[ignore]
    async fn test_ollama_generate_json() {
        let config = LlmConfig::default();
        let adapter = OllamaAdapter::new(config).unwrap();
        let result = adapter
            .generate_json(
                "Tu es un assistant. Reponds en JSON.",
                "Donne-moi un objet JSON avec un champ 'status' valant 'ok'.",
            )
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&response.content).unwrap();
        assert_eq!(parsed["status"], "ok");
    }

    // -----------------------------------------------------------------------
    // Tests wiremock — mock HTTP pour OllamaAdapter
    // -----------------------------------------------------------------------

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn make_config_for_mock(server_uri: &str) -> LlmConfig {
        LlmConfig {
            enabled: true,
            api_base_url: server_uri.to_string(),
            timeout_secs: 5,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_wiremock_generate_json_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "response": "{\"status\": \"ok\"}",
                "done": true,
                "eval_count": 42,
                "done_reason": "stop"
            })))
            .mount(&server)
            .await;

        let config = make_config_for_mock(&server.uri());
        let adapter = OllamaAdapter::new(config).unwrap();
        let result = adapter.generate_json("system", "user").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.content.contains("ok"));
        assert_eq!(response.tokens_used, 42);
        assert_eq!(response.finish_reason, FinishReason::Stop);
    }

    #[tokio::test]
    async fn test_wiremock_generate_500_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&server)
            .await;

        let config = make_config_for_mock(&server.uri());
        let adapter = OllamaAdapter::new(config).unwrap();
        let result = adapter.generate("test prompt").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            LlmError::ApiError { status_code, .. } => assert_eq!(status_code, 500),
            other => panic!("Attendu ApiError, obtenu: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_wiremock_generate_malformed_json() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(200).set_body_string("ceci n'est pas du JSON"))
            .mount(&server)
            .await;

        let config = make_config_for_mock(&server.uri());
        let adapter = OllamaAdapter::new(config).unwrap();
        let result = adapter.generate("test").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), LlmError::ParseError(_)));
    }

    #[tokio::test]
    async fn test_wiremock_check_model_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "models": [
                    {"name": "qwen3:8b"},
                    {"name": "llama3:8b"}
                ]
            })))
            .mount(&server)
            .await;

        let config = make_config_for_mock(&server.uri());
        let adapter = OllamaAdapter::new(config).unwrap();
        let found = adapter.check_model().await.unwrap();
        assert!(found, "Le modele qwen3:8b devrait etre trouve");
    }

    #[tokio::test]
    async fn test_wiremock_check_model_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "models": [
                    {"name": "llama3:8b"}
                ]
            })))
            .mount(&server)
            .await;

        let config = make_config_for_mock(&server.uri());
        let adapter = OllamaAdapter::new(config).unwrap();
        let found = adapter.check_model().await.unwrap();
        assert!(!found, "Le modele qwen3:8b ne devrait pas etre trouve");
    }

    #[tokio::test]
    async fn test_wiremock_is_ready_unreachable() {
        // Adresse qui ne repond pas (port ferme)
        let config = LlmConfig {
            enabled: true,
            api_base_url: "http://127.0.0.1:1".to_string(),
            timeout_secs: 1,
            ..Default::default()
        };
        let adapter = OllamaAdapter::new(config).unwrap();
        let ready = adapter.is_ready().await;
        assert!(
            !ready,
            "is_ready() doit retourner false si le serveur est injoignable"
        );
    }
}
