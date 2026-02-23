//! Logique partagée de parsing des User Stories depuis du texte brut
//!
//! Réutilisé par MarkdownReader, PdfReader et DocxReader.

use std::sync::LazyLock;

use regex::Regex;

use crate::domain::errors::InputError;
use crate::domain::user_story::{Language, UserStory};

/// Pattern "En tant que ... je veux ... afin de ..."
static FR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)en\s+tant\s+que\s+(.+?),?\s+je\s+veux\s+(.+?),?\s+afin\s+de\s+(.+?)(?:\.|$)")
        .expect("Regex FR_PATTERN invalide (bug interne)")
});

/// Pattern "As a ... I want ... so that ..."
static EN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)as\s+an?\s+(.+?),?\s+I\s+want\s+(?:to\s+)?(.+?),?\s+so\s+that\s+(.+?)(?:\.|$)")
        .expect("Regex EN_PATTERN invalide (bug interne)")
});

/// Parse les User Stories depuis du contenu texte brut
pub fn parse_stories(content: &str, language: Language) -> Result<Vec<UserStory>, InputError> {
    let mut stories = Vec::new();

    // Decoupe par sections (headers H2/H3 ou lignes vides multiples)
    let sections: Vec<&str> = content.split("\n## ").collect();

    for (idx, section) in sections.iter().enumerate() {
        let title = section
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .trim_start_matches('#')
            .trim();

        let pattern = match language {
            Language::French => &*FR_PATTERN,
            Language::English => &*EN_PATTERN,
        };

        if let Some(captures) = pattern.captures(section) {
            let actor = captures
                .get(1)
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();
            let action = captures
                .get(2)
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();
            let benefit = captures
                .get(3)
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();

            // Extraire les criteres d'acceptation (lignes commencant par - ou *)
            let acceptance_criteria: Vec<String> = section
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    (trimmed.starts_with("- ") || trimmed.starts_with("* "))
                        && !trimmed.contains("En tant que")
                        && !trimmed.contains("As a")
                })
                .map(|line| {
                    line.trim()
                        .trim_start_matches("- ")
                        .trim_start_matches("* ")
                        .to_string()
                })
                .collect();

            let us_title = if title.is_empty() {
                format!("US-{:03}", idx + 1)
            } else {
                title.to_string()
            };

            let mut us = UserStory::new(us_title, actor, action, benefit);
            us.external_id = Some(format!("US-{:03}", idx + 1));
            us.acceptance_criteria = acceptance_criteria;
            us.raw_text = section.to_string();

            stories.push(us);
        }
    }

    // Si aucune section avec header, essayer le contenu brut
    if stories.is_empty() {
        let pattern = match language {
            Language::French => &*FR_PATTERN,
            Language::English => &*EN_PATTERN,
        };

        for (idx, captures) in pattern.captures_iter(content).enumerate() {
            let actor = captures
                .get(1)
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();
            let action = captures
                .get(2)
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();
            let benefit = captures
                .get(3)
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();

            let mut us = UserStory::new(format!("US-{:03}", idx + 1), actor, action, benefit);
            us.external_id = Some(format!("US-{:03}", idx + 1));
            us.raw_text = captures.get(0).map_or("", |m| m.as_str()).to_string();
            stories.push(us);
        }
    }

    if stories.is_empty() {
        return Err(InputError::NoStoriesFound);
    }

    Ok(stories)
}

/// Detecte la langue du contenu texte
pub fn detect_language(content: &str) -> Language {
    let fr_indicators = ["en tant que", "je veux", "afin de", "critères", "scénario"];
    let en_indicators = ["as a", "i want", "so that", "criteria", "scenario"];

    let lower = content.to_lowercase();
    let fr_count = fr_indicators
        .iter()
        .filter(|ind| lower.contains(*ind))
        .count();
    let en_count = en_indicators
        .iter()
        .filter(|ind| lower.contains(*ind))
        .count();

    if fr_count >= en_count {
        Language::French
    } else {
        Language::English
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_detect_language_french() {
        let content = "En tant que bibliothécaire, je veux rechercher un livre afin de le prêter.";
        assert_eq!(detect_language(content), Language::French);
    }

    #[test]
    fn test_detect_language_english() {
        let content = "As a user, I want to search for a book so that I can borrow it.";
        assert_eq!(detect_language(content), Language::English);
    }

    #[test]
    fn test_parse_french_stories() {
        let content = r#"## Recherche ISBN

En tant que bibliothécaire, je veux rechercher un livre par ISBN afin de trouver rapidement un ouvrage.

- Le champ accepte ISBN-10 et ISBN-13
- Les résultats s'affichent en moins de 2 secondes

## Inscription membre

En tant que membre, je veux m'inscrire en ligne afin de pouvoir emprunter des livres.

- Le formulaire demande nom, prénom, email
- Un email de confirmation est envoyé
"#;

        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 2);
        assert_eq!(stories[0].actor, "bibliothécaire");
        assert_eq!(stories[0].acceptance_criteria.len(), 2);
        assert_eq!(stories[1].actor, "membre");
    }

    #[test]
    fn test_parse_no_stories() {
        let content = "# Just a title\n\nSome random content without user stories.";
        let result = parse_stories(content, Language::French);
        assert!(matches!(result, Err(InputError::NoStoriesFound)));
    }

    #[test]
    fn test_parse_english_stories() {
        let content = r#"## Search by ISBN

As a librarian, I want to search for a book by ISBN so that I can find it quickly.

- The input accepts ISBN-10 and ISBN-13 formats
- Results display in under 2 seconds

## Online Registration

As a member, I want to register online so that I can borrow books without visiting.

- The form asks for name, email and address
"#;

        let stories = parse_stories(content, Language::English).unwrap();
        assert_eq!(stories.len(), 2);
        assert_eq!(stories[0].actor, "librarian");
        assert_eq!(stories[0].acceptance_criteria.len(), 2);
        assert_eq!(stories[1].actor, "member");
    }

    #[test]
    fn test_parse_without_headers() {
        let content = "En tant que admin, je veux gerer les comptes afin de controler les acces.";
        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].actor, "admin");
    }

    #[test]
    fn test_parse_without_acceptance_criteria() {
        let content = r#"## Feature

En tant que user, je veux une feature afin de gagner du temps.

"#;
        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 1);
        assert!(stories[0].acceptance_criteria.is_empty());
    }

    #[test]
    fn test_detect_language_empty() {
        assert_eq!(detect_language(""), Language::French); // Default FR
    }

    #[test]
    fn test_detect_language_mixed() {
        let content =
            "En tant que user, I want something so that it works. Je veux afin de scenario";
        let lang = detect_language(content);
        // FR indicators: "en tant que", "je veux", "afin de" = 3
        // EN indicators: "i want", "so that", "scenario" = 3
        // Tie goes to French
        assert_eq!(lang, Language::French);
    }

    #[test]
    fn test_parse_with_crlf() {
        let content = "## Feature\r\n\r\nEn tant que admin, je veux gerer afin de controler.\r\n\r\n- Critere 1\r\n- Critere 2\r\n";
        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].actor, "admin");
    }

    #[test]
    fn test_parse_multiple_empty_lines() {
        let content = "## Feature\n\n\n\n\nEn tant que user, je veux tester afin de valider.\n\n\n\n- Critere\n\n\n";
        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].acceptance_criteria.len(), 1);
    }

    #[test]
    fn test_parse_actor_with_comma() {
        let content =
            "En tant que administrateur systeme, je veux superviser afin de maintenir la qualite.";
        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].actor, "administrateur systeme");
    }

    #[test]
    fn test_parse_stories_multiple_same_title() {
        let content = r#"## Feature
En tant que user, je veux A afin de B.

## Feature
En tant que admin, je veux C afin de D.
"#;
        let stories = parse_stories(content, Language::French).unwrap();
        assert_eq!(stories.len(), 2, "Meme titre ne doit pas fusionner");
    }

    mod proptest_suite {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn parse_stories_never_panics(input in "\\PC*") {
                let _ = parse_stories(&input, Language::French);
                let _ = parse_stories(&input, Language::English);
            }

            #[test]
            fn detect_language_never_panics(input in "\\PC*") {
                let _ = detect_language(&input);
            }

            #[test]
            fn detect_language_returns_valid_enum(input in "\\PC*") {
                let lang = detect_language(&input);
                assert!(lang == Language::French || lang == Language::English);
            }
        }
    }
}
