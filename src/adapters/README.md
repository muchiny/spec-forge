# 🔧 Adapters — Implémentations concrètes

> Les **adapters** implémentent les ports (traits) définis dans `ports/`.
> Ils gèrent toute l'I/O : réseau, fichiers, templates.

---

## 🏗️ Vue d'ensemble

```mermaid
graph TB
    subgraph "🔧 Adapters"
        subgraph "🧠 llm/"
            OA["OllamaAdapter<br/><i>Client HTTP Ollama</i>"]
            MA["MockAdapter<br/><i>Tests uniquement</i>"]
        end

        subgraph "📥 input/"
            MR["MarkdownReader<br/><i>.md</i>"]
            YR["YamlReader<br/><i>.yaml / .yml</i>"]
            PR["PdfReader<br/><i>.pdf</i>"]
            DR["DocxReader<br/><i>.docx</i>"]
            SP["StoryParser<br/><i>Parseur commun</i>"]
        end

        subgraph "📤 output/"
            MW["MarkdownWriter<br/><i>Spécifications .md</i>"]
            GW["GherkinWriter<br/><i>Fichiers .feature</i>"]
            TW["TraceabilityWriter<br/><i>Matrice .md</i>"]
        end

        subgraph "📝 templates/"
            FTE["FileTemplateEngine<br/><i>Handlebars</i>"]
        end
    end

    style OA fill:#2196F3,stroke:#333,color:#fff
    style MA fill:#9E9E9E,stroke:#333,color:#fff
    style MR fill:#4CAF50,stroke:#333,color:#fff
    style YR fill:#4CAF50,stroke:#333,color:#fff
    style PR fill:#4CAF50,stroke:#333,color:#fff
    style DR fill:#4CAF50,stroke:#333,color:#fff
    style MW fill:#FF9800,stroke:#333,color:#fff
    style GW fill:#FF9800,stroke:#333,color:#fff
    style TW fill:#FF9800,stroke:#333,color:#fff
    style FTE fill:#9C27B0,stroke:#333,color:#fff
```

---

## 📁 Structure

```
adapters/
├── 🧠 llm/
│   ├── ollama_adapter.rs    # Client HTTP reqwest → Ollama API
│   └── mock_adapter.rs      # Mock pour tests (réponses pré-définies)
├── 📥 input/
│   ├── markdown_reader.rs   # Parse User Stories depuis Markdown
│   ├── yaml_reader.rs       # Parse User Stories depuis YAML
│   ├── pdf_reader.rs        # Parse User Stories depuis PDF
│   ├── docx_reader.rs       # Parse User Stories depuis DOCX
│   ├── story_parser.rs      # Parseur commun (format "En tant que...")
│   └── mod.rs               # MAX_INPUT_FILE_SIZE = 10 Mo
├── 📤 output/
│   ├── markdown_writer.rs   # Écrit les spécifications en Markdown
│   ├── gherkin_writer.rs    # Écrit les fichiers .feature (FR/EN)
│   ├── traceability_writer.rs # Écrit la matrice de traçabilité
│   └── snapshots/           # Snapshots insta pour tests
└── 📝 templates/
    └── file_template_engine.rs # Charge et rend les templates Handlebars
```

---

## 🧠 LLM Adapter — OllamaAdapter

```mermaid
sequenceDiagram
    participant S as ⚙️ Service
    participant O as 🔧 OllamaAdapter
    participant API as 🤖 Ollama API

    S->>O: generate(system_prompt, user_prompt, config)
    O->>API: POST /api/generate
    Note right of API: model, prompt, temperature,<br/>num_predict, num_ctx
    API-->>O: JSON { response, done_reason }
    O->>O: Parse FinishReason
    O-->>S: Ok(LlmResponse)

    S->>O: check_connection()
    O->>API: GET /api/tags
    API-->>O: { models: [...] }
    O-->>S: Ok(()) ou Err(ModelNotFound)
```

### ⚙️ Configuration

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `model_name` | `qwen2.5:7b` | Modèle Ollama |
| `temperature` | `0.1` | Créativité (0.0 = déterministe) |
| `max_tokens` | `4096` | Tokens maximum en sortie |
| `context_size` | `8192` | Fenêtre de contexte |
| `api_base_url` | `http://localhost:11434` | URL de l'API Ollama |

---

## 📥 Input Adapters — Formats supportés

| Adapter | Format | Extension | Crate |
|---------|--------|-----------|-------|
| `MarkdownReader` | Markdown | `.md` | `pulldown-cmark` |
| `YamlReader` | YAML | `.yaml` / `.yml` | `serde_yaml` |
| `PdfReader` | PDF | `.pdf` | `pdf-extract` |
| `DocxReader` | DOCX | `.docx` | `zip` + XML parsing |

> 📏 **Limite** : 10 Mo par fichier (`MAX_INPUT_FILE_SIZE`)

### 🔄 Flux de parsing

```mermaid
graph LR
    F["📄 Fichier"] --> R["📥 Reader<br/><i>selon extension</i>"]
    R --> SP["🔍 StoryParser<br/><i>Extraction US</i>"]
    SP --> US["📝 Vec&lt;UserStory&gt;"]

    style F fill:#9E9E9E,stroke:#333,color:#fff
    style R fill:#4CAF50,stroke:#333,color:#fff
    style SP fill:#2196F3,stroke:#333,color:#fff
    style US fill:#FF9800,stroke:#333,color:#fff
```

---

## 📤 Output Adapters — Artefacts générés

| Adapter | Sortie | Contenu |
|---------|--------|---------|
| `MarkdownWriter` | `spec-*.md` | Spécification complète (FR, scénarios, entités, ...) |
| `GherkinWriter` | `*.feature` | Fichiers BDD avec tags `@US-xxx @FR-xxx` |
| `TraceabilityWriter` | `traceability.md` | Matrice FR → US → Scénarios + métriques |

---

## ➕ Ajouter un nouvel adapter

1. Créer `src/adapters/<groupe>/mon_adapter.rs`
2. Implémenter le trait du port correspondant
3. Ajouter `pub mod mon_adapter;` dans le `mod.rs` du groupe
4. Injecter via `Arc<dyn Trait>` dans l'application
5. Ajouter des tests unitaires inline

> 💡 **Astuce** : utiliser le skill `/add-adapter` pour le scaffolding automatique
