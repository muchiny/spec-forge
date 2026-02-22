# Skill: Verification conformite ISO

Analyse la couverture des normes ISO dans spec-forge.

## Instructions

### 1. ISO 29148 - Criteres de bien-formation
Lis `src/domain/validation.rs` et verifie que les 9 criteres sont implementes :
1. Necessary (pas de doublon)
2. Unambiguous (pas de mots vagues)
3. Complete (tous champs remplis)
4. Singular (une exigence par statement)
5. Feasible
6. Verifiable (methode de verification)
7. Correct (syntaxe normative MUST/SHALL/SHOULD/COULD)
8. Conforming (format respecte)
9. Traceable (source identifiee)

Pour chaque critere, indique : implemente / partiellement / manquant.

### 2. ISO 25010 - Caracteristiques qualite
Verifie que les 9 caracteristiques sont definies dans `src/domain/specification.rs` :
FunctionalSuitability, PerformanceEfficiency, Compatibility, InteractionCapability, Reliability, Security, Maintainability, Flexibility, Safety.

### 3. ISO 29119 - Couverture de test
Verifie dans `src/domain/test_case.rs` :
- Niveaux de test (Unit, Integration, System, Acceptance)
- Techniques de couverture (EquivalencePartitioning, BoundaryValueAnalysis, DecisionTable, StateTransition, ErrorGuessing)
- Types de scenario (HappyPath, EdgeCase, ErrorScenario)

### 4. Templates LLM
Verifie que les templates dans `templates/` instruisent correctement le LLM sur les normes :
- `refine_system.md` : mentionne ISO 29148 et ISO 25010
- `generate_tests_system.md` : mentionne ISO 29119

### 5. Rapport
Produis un tableau de conformite :

| Norme | Critere | Statut | Fichier |
|-------|---------|--------|---------|
| ISO 29148 | ... | ... | ... |

Avec un score global de couverture et des recommandations d'amelioration.
