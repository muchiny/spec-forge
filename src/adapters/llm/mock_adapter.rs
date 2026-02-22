//! Mock LLM Adapter pour les tests

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::ports::llm_service::{
    FinishReason, LlmConfig, LlmError, LlmModelInfo, LlmResponse, LlmService,
};

/// Mock adapter qui retourne des reponses pre-enregistrees
pub struct MockLlmAdapter {
    config: LlmConfig,
    responses: Arc<Mutex<Vec<LlmResponse>>>,
    call_count: Arc<Mutex<usize>>,
}

impl MockLlmAdapter {
    /// Cree un mock avec des reponses texte pre-enregistrees (finish_reason=Stop)
    pub fn new(responses: Vec<String>) -> Self {
        let llm_responses = responses
            .into_iter()
            .map(|content| LlmResponse {
                content,
                tokens_used: 100,
                finish_reason: FinishReason::Stop,
            })
            .collect();
        Self {
            config: LlmConfig {
                enabled: true,
                provider: "mock".to_string(),
                model_name: "mock-model".to_string(),
                ..Default::default()
            },
            responses: Arc::new(Mutex::new(llm_responses)),
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Cree un mock avec des LlmResponse completes (permet de controler finish_reason)
    pub fn with_responses(responses: Vec<LlmResponse>) -> Self {
        Self {
            config: LlmConfig {
                enabled: true,
                provider: "mock".to_string(),
                model_name: "mock-model".to_string(),
                ..Default::default()
            },
            responses: Arc::new(Mutex::new(responses)),
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Retourne le nombre d'appels effectues
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    fn next_response(&self) -> LlmResponse {
        let mut count = self.call_count.lock().unwrap();
        let responses = self.responses.lock().unwrap();
        let idx = *count % responses.len();
        *count += 1;
        responses[idx].clone()
    }
}

#[async_trait]
impl LlmService for MockLlmAdapter {
    async fn generate(&self, _prompt: &str) -> Result<LlmResponse, LlmError> {
        Ok(self.next_response())
    }

    async fn generate_with_system(
        &self,
        _system_prompt: &str,
        _user_prompt: &str,
    ) -> Result<LlmResponse, LlmError> {
        Ok(self.next_response())
    }

    async fn generate_json(
        &self,
        _system_prompt: &str,
        _user_prompt: &str,
    ) -> Result<LlmResponse, LlmError> {
        Ok(self.next_response())
    }

    async fn is_ready(&self) -> bool {
        true
    }

    fn config(&self) -> &LlmConfig {
        &self.config
    }

    fn model_info(&self) -> LlmModelInfo {
        LlmModelInfo {
            name: "mock-model".to_string(),
            provider: "mock".to_string(),
            context_size: Some(8192),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_adapter_returns_responses() {
        let mock = MockLlmAdapter::new(vec!["response1".into(), "response2".into()]);
        let r1 = mock.generate("test").await.unwrap();
        let r2 = mock.generate("test").await.unwrap();
        assert_eq!(r1.content, "response1");
        assert_eq!(r2.content, "response2");
        assert_eq!(mock.call_count(), 2);
    }

    #[tokio::test]
    async fn test_mock_adapter_cycles_responses() {
        let mock = MockLlmAdapter::new(vec!["only".into()]);
        let r1 = mock.generate("a").await.unwrap();
        let r2 = mock.generate("b").await.unwrap();
        assert_eq!(r1.content, "only");
        assert_eq!(r2.content, "only");
    }

    #[tokio::test]
    async fn test_mock_adapter_is_ready() {
        let mock = MockLlmAdapter::new(vec!["test".into()]);
        assert!(mock.is_ready().await);
    }
}
