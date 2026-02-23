# ⚙️ Application — Orchestration du pipeline

> La couche **application** orchestre les cas d'usage en combinant les ports et le domaine.
> Elle gère le retry LLM, le parsing JSON, et les événements du pipeline.

---

## 🏗️ Vue d'ensemble

```mermaid
graph TB
    subgraph "⚙️ Application Layer"
        PIP["🔄 Pipeline<br/><i>pipeline.rs</i><br/>Orchestrateur principal"]
        RS["📋 RefineService<br/><i>refine_service.rs</i><br/>US → Spécification"]
        GTS["🧪 GenerateTestsService<br/><i>generate_tests_service.rs</i><br/>Spec → Gherkin"]
        RET["🔁 LlmRetry<br/><i>llm_retry.rs</i><br/>Stratégie de retry"]
        JU["🔧 JsonUtils<br/><i>json_utils.rs</i><br/>Parsing JSON robuste"]
        PE["📡 PipelineEvents<br/><i>pipeline_events.rs</i><br/>Événements temps réel"]
    end

    PIP --> RS
    PIP --> GTS
    RS --> RET
    GTS --> RET
    RS --> JU
    GTS --> JU
    PIP --> PE

    style PIP fill:#FF9800,stroke:#333,color:#fff
    style RS fill:#2196F3,stroke:#333,color:#fff
    style GTS fill:#4CAF50,stroke:#333,color:#fff
    style RET fill:#9C27B0,stroke:#333,color:#fff
    style JU fill:#00BCD4,stroke:#333,color:#fff
    style PE fill:#F44336,stroke:#333,color:#fff
```

---

## 📁 Fichiers

| Fichier | Rôle | Taille |
|---------|------|--------|
| 🔄 `pipeline.rs` | Orchestrateur : lecture → raffinement → génération → écriture | ~13 Ko |
| 📋 `refine_service.rs` | Raffinement US → Spécification via LLM + parsing JSON | ~56 Ko |
| 🧪 `generate_tests_service.rs` | Génération Spec → Gherkin/BDD via LLM + parsing JSON | ~45 Ko |
| 🔁 `llm_retry.rs` | Retry exponentiel avec backoff configurable | ~6 Ko |
| 🔧 `json_utils.rs` | Nettoyage et extraction JSON depuis les réponses LLM | ~7 Ko |
| 📡 `pipeline_events.rs` | Types `PipelineStage` et `PipelineEvent` pour le suivi | ~2 Ko |

---

## 🔄 Pipeline — Flux complet

```mermaid
sequenceDiagram
    participant CLI as 🖥️ CLI / TUI
    participant PIP as 🔄 Pipeline
    participant IR as 📥 InputReader
    participant RS as 📋 RefineService
    participant GTS as 🧪 GenerateTestsService
    participant OW as 📤 OutputWriter
    participant LLM as 🤖 Ollama

    CLI->>PIP: run(input, output)
    PIP->>IR: read(input_path)
    IR-->>PIP: Vec<UserStory>

    loop Pour chaque UserStory
        PIP->>RS: refine(user_story)
        RS->>LLM: generate(refine_prompt)
        LLM-->>RS: JSON Specification
        RS-->>PIP: Specification
    end

    PIP->>GTS: generate(specifications)
    GTS->>LLM: generate(test_prompt)
    LLM-->>GTS: JSON TestSuite
    GTS-->>PIP: TestSuite

    PIP->>OW: write_specs(specifications)
    PIP->>OW: write_features(test_suite)
    PIP->>OW: write_traceability(matrix)
    PIP-->>CLI: ✅ Terminé
```

---

## 📋 RefineService — US → Spécification

Le service de raffinement transforme chaque User Story en une spécification conforme ISO 29148 :

```mermaid
graph LR
    US["📝 UserStory"] --> TPL["📝 Template<br/><i>refine_system.md<br/>refine_user.md</i>"]
    TPL --> LLM["🤖 LLM"]
    LLM --> JSON["📦 JSON brut"]
    JSON --> PARSE["🔧 JsonUtils<br/><i>nettoyage + extraction</i>"]
    PARSE --> SPEC["📋 Specification"]
    SPEC --> VAL["✅ Validation<br/><i>9 critères ISO 29148</i>"]

    style US fill:#4CAF50,stroke:#333,color:#fff
    style LLM fill:#2196F3,stroke:#333,color:#fff
    style SPEC fill:#FF9800,stroke:#333,color:#fff
    style VAL fill:#9C27B0,stroke:#333,color:#fff
```

---

## 🧪 GenerateTestsService — Spec → Gherkin

Le service de génération produit des tests BDD conformes ISO 29119 :

```mermaid
graph LR
    SPEC["📋 Specification"] --> TPL["📝 Template<br/><i>generate_tests_system.md<br/>generate_tests_user.md</i>"]
    TPL --> LLM["🤖 LLM"]
    LLM --> JSON["📦 JSON brut"]
    JSON --> PARSE["🔧 JsonUtils"]
    PARSE --> TS["🧪 TestSuite"]
    TS --> GH["📄 .feature files"]

    style SPEC fill:#2196F3,stroke:#333,color:#fff
    style LLM fill:#2196F3,stroke:#333,color:#fff
    style TS fill:#FF9800,stroke:#333,color:#fff
    style GH fill:#4CAF50,stroke:#333,color:#fff
```

---

## 🔁 Stratégie de retry LLM

| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| `max_retries` | 3 (config) | Nombre maximum de tentatives |
| Backoff | Exponentiel | Délai croissant entre les tentatives |
| Retry sur | `OutputTruncated`, `OutputParseFailed` | Erreurs récupérables |
| Pas de retry | `ConnectionFailed`, `ModelNotFound` | Erreurs fatales |

---

## 📡 Événements du pipeline

Les événements permettent le suivi en temps réel (TUI et CLI) :

| Stage | Description |
|-------|-------------|
| `Reading` | 📥 Lecture des entrées |
| `Refining` | 📋 Raffinement en cours |
| `Generating` | 🧪 Génération des tests |
| `Writing` | 📤 Écriture des sorties |
| `Completed` | ✅ Pipeline terminé |
| `Failed` | ❌ Erreur survenue |
