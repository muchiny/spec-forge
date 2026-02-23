# 🎯 Domain — Logique métier pure

> Cœur du système spec-forge. **Aucune dépendance externe** (pas d'I/O, pas de réseau, pas de framework).
> Seules les crates `serde`, `thiserror`, `uuid` et `chrono` sont autorisées.

---

## 🏗️ Vue d'ensemble

```mermaid
graph TB
    subgraph "🎯 Domain Layer"
        US["📝 UserStory<br/><i>user_story.rs</i>"]
        SP["📋 Specification<br/><i>specification.rs</i>"]
        TC["🧪 Feature / Scenario<br/><i>test_case.rs</i>"]
        VA["✅ Validation<br/><i>validation.rs</i>"]
        TR["📊 TraceabilityMatrix<br/><i>traceability.rs</i>"]
        ER["❌ DomainError<br/><i>errors.rs</i>"]
    end

    US -->|"raffinement LLM"| SP
    SP -->|"génération LLM"| TC
    SP --> VA
    TC --> VA
    SP --> TR
    TC --> TR

    style US fill:#4CAF50,stroke:#333,color:#fff
    style SP fill:#2196F3,stroke:#333,color:#fff
    style TC fill:#FF9800,stroke:#333,color:#fff
    style VA fill:#9C27B0,stroke:#333,color:#fff
    style TR fill:#00BCD4,stroke:#333,color:#fff
    style ER fill:#F44336,stroke:#333,color:#fff
```

---

## 📁 Fichiers

| Fichier | Rôle | Norme ISO |
|---------|------|-----------|
| 📝 `user_story.rs` | Modèle `UserStory`, `Priority` (MoSCoW P1/P2/P3), `Language` (FR/EN) | — |
| 📋 `specification.rs` | `Specification`, `FunctionalRequirement`, `QualityCharacteristic`, `ComplianceProfile` | ISO 29148, ISO 25010 |
| 🧪 `test_case.rs` | `Feature`, `Scenario`, `Step`, `TestLevel`, `CoverageTechnique` | ISO 29119 |
| ✅ `validation.rs` | 9 critères de bien-formation, mots ambigus interdits, métriques de couverture | ISO 29148, ISO 25023 |
| 📊 `traceability.rs` | `TraceabilityMatrix`, `TraceabilityEntry`, `ComplianceNote` | ISO 29148 §6.6 |
| ❌ `errors.rs` | `DomainError`, `InputError`, `RefinementError`, `GenerationError`, `ValidationError` | — |

---

## 📐 Normes ISO implémentées

### 📋 ISO/IEC/IEEE 29148:2018 — Ingénierie des exigences

9 critères de bien-formation dans `validation.rs` :

```
✅ Necessary    — Pas de doublon
🎯 Unambiguous  — Pas de mots ambigus (environ, parfois, ...)
📝 Complete     — Tous les champs remplis
1️⃣ Singular     — Une exigence par statement
🏗️ Feasible     — Réalisable
🔬 Verifiable   — Testable
✏️ Correct      — Mots normatifs (MUST/SHALL/SHOULD/COULD)
📏 Conforming   — Format conforme
🔗 Traceable    — Source identifiable
```

### ⭐ ISO/IEC 25010:2023 — Qualité produit

9 caractéristiques dans `specification.rs` → `QualityCharacteristic` :

| Caractéristique | Description |
|----------------|-------------|
| `FunctionalSuitability` | Adéquation fonctionnelle |
| `PerformanceEfficiency` | Efficacité performance |
| `Compatibility` | Compatibilité |
| `InteractionCapability` | Capacité d'interaction |
| `Reliability` | Fiabilité |
| `Security` | Sécurité |
| `Maintainability` | Maintenabilité |
| `Flexibility` | Flexibilité |
| `Safety` | Sûreté |

### 🧪 ISO/IEC/IEEE 29119 — Tests logiciel

Niveaux de test (`TestLevel`) : `Unit` | `Integration` | `System` | `Acceptance`

Techniques de couverture (`CoverageTechnique`) :

| Code | Technique |
|------|-----------|
| `EP` | Equivalence Partitioning |
| `BVA` | Boundary Value Analysis |
| `DT` | Decision Table |
| `ST` | State Transition |
| `EG` | Error Guessing |

---

## 🏭 Profils de conformité réglementaire

```mermaid
graph LR
    CP["🏭 ComplianceProfile"]
    CP --> G["🌐 General<br/><i>ISO 29148</i>"]
    CP --> AV["✈️ Aviation<br/><i>DO-178C</i>"]
    CP --> MD["🏥 Medical<br/><i>IEC 62304</i>"]
    CP --> AU["🚗 Automotive<br/><i>ISO 26262</i>"]
    CP --> RW["🚄 Railway<br/><i>EN 50716</i>"]
    CP --> SF["🔒 Safety<br/><i>IEC 61508</i>"]

    AV --> DAL["DAL A/B/C/D/E"]
    MD --> SW["SwClass A/B/C"]
    AU --> ASIL["ASIL A/B/C/D"]
    RW --> SSIL["SSIL 1/2/3/4"]
    SF --> SIL["SIL 1/2/3/4"]

    style CP fill:#FF9800,stroke:#333,color:#fff
    style G fill:#4CAF50,stroke:#333,color:#fff
    style AV fill:#2196F3,stroke:#333,color:#fff
    style MD fill:#F44336,stroke:#333,color:#fff
    style AU fill:#9C27B0,stroke:#333,color:#fff
```

---

## 🔒 Hiérarchie des erreurs

```mermaid
graph TD
    DE["❌ DomainError"]
    DE --> IE["📥 InputError<br/><i>FileNotFound, InvalidFormat,<br/>NoStoriesFound, ParseError</i>"]
    DE --> RE["🔧 RefinementError<br/><i>LlmFailed, OutputParseFailed,<br/>IncompleteSpec, OutputTruncated</i>"]
    DE --> GE["🧪 GenerationError<br/><i>GherkinFailed, InvalidSyntax,<br/>SpecNotRefined, OutputTruncated</i>"]
    DE --> VE["✅ ValidationError<br/><i>GherkinSyntax, TraceabilityGap,<br/>CompletenessBelow</i>"]

    style DE fill:#F44336,stroke:#333,color:#fff
    style IE fill:#FF9800,stroke:#333,color:#fff
    style RE fill:#FF9800,stroke:#333,color:#fff
    style GE fill:#FF9800,stroke:#333,color:#fff
    style VE fill:#FF9800,stroke:#333,color:#fff
```

---

## 🧪 Tests

Chaque fichier du domaine contient ses tests unitaires inline `#[cfg(test)]`.

```bash
# Lancer uniquement les tests du domaine
cargo test domain::
```
