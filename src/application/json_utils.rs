//! Utilitaires de nettoyage JSON pour les reponses LLM

use tracing::debug;

/// Nettoie la reponse JSON du LLM (retire les blocs markdown, extrait le JSON)
///
/// Gere les cas suivants :
/// - Blocs `<think>...</think>` (mode thinking Qwen3/DeepSeek-R1)
/// - Blocs ```json ... ```
/// - Blocs ``` ... ```
/// - JSON entoure de texte (extraction premier { au dernier })
/// - Texte brut (retourne tel quel)
pub fn clean_json_response(response: &str) -> String {
    // Etape 0 : Retirer les blocs <think>...</think> (modeles avec mode thinking)
    let cleaned = strip_think_blocks(response);
    let trimmed = cleaned.trim();

    // Retirer les blocs ```json ... ```
    if let Some(start) = trimmed.find("```json") {
        let json_start = start + 7;
        if let Some(end) = trimmed[json_start..].find("```") {
            return trimmed[json_start..json_start + end].trim().to_string();
        }
    }

    // Retirer les blocs ``` ... ```
    if let Some(start) = trimmed.find("```") {
        let json_start = start + 3;
        // Skip to newline after ```
        let actual_start = trimmed[json_start..]
            .find('\n')
            .map(|n| json_start + n + 1)
            .unwrap_or(json_start);
        if let Some(end) = trimmed[actual_start..].find("```") {
            return trimmed[actual_start..actual_start + end].trim().to_string();
        }
    }

    // Trouver le premier { et le dernier }
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}'))
        && start <= end
    {
        return trimmed[start..=end].to_string();
    }

    trimmed.to_string()
}

/// Retire tous les blocs `<think>...</think>` d'une reponse LLM.
///
/// Gere les cas :
/// - Bloc complet : `<think>contenu</think>` -> supprime
/// - Bloc non ferme (reponse tronquee) : `<think>contenu...` -> supprime tout apres `<think>`
/// - Pas de bloc : retourne le texte tel quel
fn strip_think_blocks(input: &str) -> String {
    if !input.contains("<think>") {
        return input.to_string();
    }

    debug!("Bloc <think> detecte et retire de la reponse LLM");

    let mut result = String::with_capacity(input.len());
    let mut remaining = input;

    while let Some(start_pos) = remaining.find("<think>") {
        // Ajouter le texte avant le bloc <think>
        result.push_str(&remaining[..start_pos]);

        // Chercher le </think> correspondant
        let after_tag = &remaining[start_pos + 7..]; // 7 = "<think>".len()
        if let Some(end_pos) = after_tag.find("</think>") {
            // Bloc complet : sauter tout le contenu entre <think> et </think>
            remaining = &after_tag[end_pos + 8..]; // 8 = "</think>".len()
        } else {
            // Bloc non ferme (reponse tronquee) : ignorer tout le reste
            return result.trim().to_string();
        }
    }

    // Ajouter le texte restant apres le dernier bloc
    result.push_str(remaining);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_json_code_block() {
        let input = "```json\n{\"key\": \"value\"}\n```";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_bare_code_block() {
        let input = "```\n{\"key\": \"value\"}\n```";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_plain_json() {
        let input = "{\"key\": \"value\"}";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_json_with_surrounding_text() {
        let input = "Here is the JSON:\n{\"key\": \"value\"}\nThat's all.";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_nested_braces() {
        let input = "Some text {\"a\": {\"b\": \"c\"}} more text";
        assert_eq!(clean_json_response(input), "{\"a\": {\"b\": \"c\"}}");
    }

    #[test]
    fn test_empty() {
        assert_eq!(clean_json_response(""), "");
        assert_eq!(clean_json_response("   "), "");
    }

    #[test]
    fn test_no_json() {
        assert_eq!(clean_json_response("just text"), "just text");
    }

    #[test]
    fn test_brace_before_open_no_panic() {
        // Regression: } before { should not panic
        assert_eq!(clean_json_response("}={R"), "}={R");
    }

    #[test]
    fn test_closing_brace_only() {
        assert_eq!(clean_json_response("}"), "}");
    }

    #[test]
    fn test_opening_brace_only() {
        assert_eq!(clean_json_response("{"), "{");
    }

    // Tests pour le mode thinking (Qwen3, DeepSeek-R1)

    #[test]
    fn test_strip_think_block_before_json() {
        let input = "<think>\nLe user veut un JSON...\n</think>\n{\"key\": \"value\"}";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_strip_think_block_with_braces_inside() {
        let input = "<think>\nAnalysons {\"partial\": true} ...\n</think>\n{\"real\": \"data\"}";
        assert_eq!(clean_json_response(input), "{\"real\": \"data\"}");
    }

    #[test]
    fn test_strip_unclosed_think_block() {
        let input = "<think>\nLe modele reflechit encore...";
        assert_eq!(clean_json_response(input), "");
    }

    #[test]
    fn test_strip_think_block_unclosed_with_json_before() {
        let input = "{\"key\": \"value\"}\n<think>\nEncore en train de reflechir...";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_strip_multiple_think_blocks() {
        let input = "<think>Premier</think>text<think>Deuxieme</think>{\"ok\": true}";
        assert_eq!(clean_json_response(input), "{\"ok\": true}");
    }

    #[test]
    fn test_no_think_block_unchanged() {
        let input = "{\"key\": \"value\"}";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_think_block_with_markdown_json() {
        let input = "<think>\nRaisonnement...\n</think>\n```json\n{\"key\": \"value\"}\n```";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    mod proptest_suite {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn clean_json_never_panics(input in "\\PC*") {
                let _ = clean_json_response(&input);
            }

            #[test]
            fn clean_json_with_think_never_panics(
                think_content in "\\PC{0,200}",
                after_content in "\\PC{0,200}"
            ) {
                let input = format!("<think>{}</think>{}", think_content, after_content);
                let _ = clean_json_response(&input);
            }
        }
    }
}
