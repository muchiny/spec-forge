# 🔌 Ports — Interfaces abstraites

> Les **ports** définissent les contrats (traits) entre le domaine/application et le monde extérieur.
> Chaque port est un trait Rust avec `#[async_trait]` + `Send + Sync`.

---

## 🏗️ Vue d'ensemble

```mermaid
graph LR
    subgraph "🔌 Ports (Traits)"
        P1["🧠 LlmService"]
        P2["📥 InputReader"]
        P3["📤 OutputWriter"]
        P4["📝 TemplateEngine"]
    end

    subgraph "🔧 Adapters (Implémentations)"
        A1["OllamaAdapter<br/>MockAdapter"]
        A2["MarkdownReader<br/>YamlReader<br/>PdfReader<br/>DocxReader"]
        A3["MarkdownWriter<br/>GherkinWriter<br/>TraceabilityWriter"]
        A4["FileTemplateEngine"]
    end

    P1 -.->|"impl"| A1
    P2 -.->|"impl"| A2
    P3 -.->|"impl"| A3
    P4 -.->|"impl"| A4

    style P1 fill:#2196F3,stroke:#333,color:#fff
    style P2 fill:#2196F3,stroke:#333,color:#fff
    style P3 fill:#2196F3,stroke:#333,color:#fff
    style P4 fill:#2196F3,stroke:#333,color:#fff
    style A1 fill:#FF9800,stroke:#333,color:#fff
    style A2 fill:#FF9800,stroke:#333,color:#fff
    style A3 fill:#FF9800,stroke:#333,color:#fff
    style A4 fill:#FF9800,stroke:#333,color:#fff
```

---

## 📁 Fichiers

| Fichier | Trait | Responsabilité |
|---------|-------|----------------|
| 🧠 `llm_service.rs` | `LlmService` | Communication avec le LLM (generate, check_connection) |
| 📥 `input_reader.rs` | `InputReader` | Lecture des User Stories depuis un fichier |
| 📤 `output_writer.rs` | `OutputWriter` | Écriture des artefacts générés |
| 📝 `template_engine.rs` | `TemplateEngine` | Chargement et rendu des templates de prompts |

---

## 🧠 LlmService — Le port principal

```mermaid
sequenceDiagram
    participant App as ⚙️ Application
    participant Port as 🔌 LlmService
    participant Adapter as 🔧 OllamaAdapter
    participant LLM as 🤖 Ollama

    App->>Port: generate(prompt, config)
    Port->>Adapter: HTTP POST /api/generate
    Adapter->>LLM: Requête JSON
    LLM-->>Adapter: Réponse JSON
    Adapter-->>Port: LlmResponse
    Port-->>App: Result<LlmResponse>
```

### 📦 Types associés

| Type | Description |
|------|-------------|
| `LlmConfig` | Configuration (model, temperature, max_tokens, context_size) |
| `LlmResponse` | Réponse LLM (response, model, finish_reason) |
| `FinishReason` | Raison d'arrêt (Stop, Length, Error) |
| `LlmError` | Erreurs LLM (ConnectionFailed, ModelNotFound, GenerationFailed) |

---

## 🔑 Principe d'injection de dépendances

```rust
// ✅ Injection via Arc<dyn Trait>
let llm: Arc<dyn LlmService> = Arc::new(OllamaAdapter::new(config));
let pipeline = Pipeline::new(llm, reader, writer, template_engine);
```

> 💡 **Avantage** : les tests utilisent `MockAdapter` sans modifier le code applicatif.

---

## ➕ Ajouter un nouveau port

1. Créer `src/ports/mon_port.rs` avec un trait `#[async_trait]`
2. Ajouter `pub mod mon_port;` dans `src/ports/mod.rs`
3. Créer l'adapter correspondant dans `src/adapters/`
4. Injecter via `Arc<dyn MonPort>` dans l'application
