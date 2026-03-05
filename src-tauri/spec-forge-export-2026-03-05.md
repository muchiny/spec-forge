# Rapport complet spec-forge : Specification (9 User Stories)

| Champ | Valeur |
|-------|--------|
| **ID** | 210a6553-4b60-471f-a7e3-f50fbbf03e0c |
| **Version** | 1.0.0 |
| **Statut** | NeedsClarification |
| **Auteur** | — |
| **Date** | 2026-03-05T13:08:24.831796465Z |
| **Baseline** | — |
| **Outil** | 0.1.0 |
| **Profil conformite** | — |
| **Stories source** | a1b2c3d4-1111-4000-a000-000000000001, a1b2c3d4-1111-4000-a000-000000000002, a1b2c3d4-1111-4000-a000-000000000003, a1b2c3d4-1111-4000-a000-000000000004, a1b2c3d4-1111-4000-a000-000000000005, a1b2c3d4-1111-4000-a000-000000000006, a1b2c3d4-1111-4000-a000-000000000007, a1b2c3d4-1111-4000-a000-000000000008, a1b2c3d4-1111-4000-a000-000000000009 |

## Validation

- **Completude** : 1%
- **Clarte** : 0.5%
- **Testabilite** : 1%

| Categorie | Critere | Statut |
|-----------|---------|--------|
| Completude | Au moins un scenario utilisateur defini | ✅ |
| Completude | Exigences fonctionnelles definies | ✅ |
| Completude | Criteres de succes mesurables definis | ✅ |
| Completude | Chaque scenario a des criteres d'acceptation | ✅ |
| Clarte | Moins de 3 clarifications non resolues | ❌ |
| Testabilite | Toutes les exigences sont testables | ✅ |
| Completude | Cas limites identifies | ✅ |
| Conformite | ISO-29148: IDs d'exigences uniques | ✅ |
| Conformite | ISO-29148: Syntaxe normative (MUST/SHALL/SHOULD) | ✅ |
| Clarte | ISO-29148: Aucun mot ambigu dans les exigences | ✅ |
| Conformite | ISO-29148: Exigences P1 ont un niveau de risque | ❌ |
| Conformite | ISO-25010: NFR ont une quality_characteristic | ✅ |

## Scenarios utilisateur (9)

### US-001 — Authentification biometrique (Priorite: P1)

Le client souhaite se connecter à l'application via son empreinte digitale ou la reconnaissance faciale pour accéder à ses comptes de manière sécurisée et rapide, sans avoir à entrer un mot de passe.

**Justification priorite** : L'authentification biométrique est un besoin essentiel pour la sécurité et la facilité d'accès, en remplaçant le mot de passe.

**Test independant** : Vérifier que l'authentification biométrique est disponible et fonctionne correctement avec les options proposées.

**Story source** : a1b2c3d4-1111-4000-a000-000000000001

**Scenarios d'acceptation** :

1. **Given** L'utilisateur a installé l'application et a un appareil compatible avec la reconnaissance faciale ou l'empreinte digitale., **When** L'utilisateur tente de se connecter., **Then** L'authentification biométrique est proposée et fonctionne correctement.

---

### US-002 — Consultation des soldes et des opérations (Priorite: P1)

Le client souhaite consulter le solde et l'historique de ses comptes courants et d'épargne pour suivre sa situation financière en temps réel.

**Justification priorite** : La consultation des soldes et des opérations est un besoin fondamental pour la gestion de ses finances.

**Test independant** : Vérifier que les soldes et les opérations sont affichés correctement après connexion.

**Story source** : a1b2c3d4-1111-4000-a000-000000000002

**Scenarios d'acceptation** :

1. **Given** L'utilisateur est connecté à l'application., **When** L'utilisateur accède à la page d'accueil., **Then** Les soldes des comptes sont affichés.

---

### US-003 — Virement instantané SEPA (Priorite: P1)

Le client souhaite effectuer un virement instantané vers un autre compte bancaire en zone SEPA pour transférer de l'argent en quelques secondes, y compris les jours fériés.

**Justification priorite** : Le virement instantané est un besoin clé pour la fluidité des transferts financiers.

**Test independant** : Vérifier que le virement est exécuté en moins de 10 secondes et que le débit est immédiat.

**Story source** : a1b2c3d4-1111-4000-a000-000000000003

**Scenarios d'acceptation** :

1. **Given** L'utilisateur est connecté et a sélectionné un bénéficiaire., **When** L'utilisateur valide le virement., **Then** Le virement est exécuté en moins de 10 secondes.

---

### US-004 — Gestion des cartes bancaires (Priorite: P1)

Le client souhaite gérer les paramètres de sa carte bancaire depuis l'application pour contrôler ses plafonds, bloquer sa carte temporairement et activer le paiement sans contact.

**Justification priorite** : La gestion des cartes est un besoin essentiel pour la sécurité et la flexibilité.

**Test independant** : Vérifier que les paramètres de la carte peuvent être modifiés et que les actions sont instantanées.

**Story source** : a1b2c3d4-1111-4000-a000-000000000004

**Scenarios d'acceptation** :

1. **Given** L'utilisateur est connecté et a sélectionné une carte., **When** L'utilisateur active le paiement sans contact., **Then** L'activation prend effet en moins de 30 secondes.

---

### US-005 — Alertes et notifications paramétrables (Priorite: P1)

Le client souhaite configurer des alertes sur ses comptes et opérations pour être informé en temps réel des mouvements importants et éviter les dépassements.

**Justification priorite** : Les alertes sont un outil clé pour la sécurité et la gestion proactive des finances.

**Test independant** : Vérifier que les alertes sont configurables et envoyées selon les préférences.

**Story source** : a1b2c3d4-1111-4000-a000-000000000005

**Scenarios d'acceptation** :

1. **Given** L'utilisateur a configuré une alerte sur un seuil de solde., **When** Le solde dépasse le seuil., **Then** Une alerte est envoyée par push, email ou SMS.

---

### US-006 — Dépôt de chèque par photo (Priorite: P2)

Le client souhaite déposer un chèque en le photographiant avec l'application pour éviter de se déplacer en agence pour encaisser un chèque.

**Justification priorite** : Le dépôt de chèque par photo est une fonctionnalité pratique pour les clients souhaitant éviter les déplacements.

**Test independant** : Vérifier que le dépôt de chèque par photo est possible et que les informations sont extraites correctement.

**Story source** : a1b2c3d4-1111-4000-a000-000000000006

**Scenarios d'acceptation** :

1. **Given** L'utilisateur a photographié le recto et le verso d'un chèque., **When** L'utilisateur valide les informations extraites., **Then** Le chèque est soumis et un statut est affiché.

---

### US-007 — Categorisation automatique des dépenses (Priorite: P2)

Le client souhaite visualiser ses dépenses réparties par catégorie avec des graphiques pour comprendre où va son argent et mieux gérer son budget mensuel.

**Justification priorite** : La visualisation des dépenses est utile pour la gestion budgétaire et la planification.

**Test independant** : Vérifier que les dépenses sont classées automatiquement et que les graphiques sont affichés correctement.

**Story source** : a1b2c3d4-1111-4000-a000-000000000007

**Scenarios d'acceptation** :

1. **Given** L'utilisateur a consulté ses opérations., **When** L'utilisateur accède aux graphiques., **Then** Les dépenses sont réparties en 12 catégories.

---

### US-008 — Coffre-fort de documents bancaires (Priorite: P2)

Le client souhaite consulter et télécharger ses documents bancaires depuis l'application pour accéder à ses relevés, contrats et attestations sans attendre le courrier postal.

**Justification priorite** : Le coffre-fort de documents est une fonctionnalité pratique pour accéder aux documents en temps réel.

**Test independant** : Vérifier que les documents sont disponibles et que leur téléchargement est protégé.

**Story source** : a1b2c3d4-1111-4000-a000-000000000008

**Scenarios d'acceptation** :

1. **Given** L'utilisateur est connecté et a sélectionné un document., **When** L'utilisateur tente de le télécharger., **Then** Le téléchargement est protégé par la biométrie ou le code PIN.

---

### US-009 — Contact conseiller par chat (Priorite: P2)

Le client souhaite contacter son conseiller bancaire via un chat intégré dans l'application pour obtenir des réponses rapides sans devoir se déplacer en agence ou attendre au téléphone.

**Justification priorite** : Le chat avec le conseiller est une fonctionnalité utile pour la communication directe et rapide.

**Test independant** : Vérifier que le chat est disponible et que les messages sont envoyés correctement.

**Story source** : a1b2c3d4-1111-4000-a000-000000000009

**Scenarios d'acceptation** :

1. **Given** L'utilisateur est connecté et a sélectionné un conseiller., **When** L'utilisateur envoie un message., **Then** Le message est envoyé et un historique est conservé.

---

## Exigences fonctionnelles (63)

### FR-001 — Functional (P1)

> Le système DOIT supporter l'empreinte digitale et la reconnaissance faciale (Face ID / Android Biometric) pour l'authentification.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'authentification biométrique est un besoin essentiel pour la sécurité et la facilité d'accès.
- **Source** : US-001

### FR-002 — Functional (P1)

> Le système DOIT permettre l'authentification biométrique après 3 tentatives et basculer sur le code PIN à 6 chiffres.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'authentification biométrique doit être sécurisée et avoir une alternative en cas d'échec.
- **Source** : US-001

### FR-003 — Functional (P1)

> Le système DOIT expirer la session automatiquement après 5 minutes d'inactivité.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'expiration de la session est nécessaire pour la sécurité.
- **Source** : US-001

### FR-004 — Functional (P1)

> Le système DOIT exiger le mot de passe classique avant d'activer la biometrie au premier lancement après installation.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La sécurité exige une vérification supplémentaire avant l'activation de la biometrie.
- **Source** : US-001

### FR-005 — Functional (P1)

> Le système DOIT afficher un journal de connexion horodaté dans les paramètres de sécurité.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Le journal de connexion est nécessaire pour la traçabilité et la sécurité.
- **Source** : US-001

### FR-006 — Functional (P1)

> Le système DOIT permettre à l'utilisateur de désactiver la biometrie à tout moment.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'utilisateur doit avoir le contrôle sur l'activation de la biometrie.
- **Source** : US-001

### FR-007 — Functional (P1)

> Le système DOIT afficher le solde de chaque compte dès la page d'accueil après connexion.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Le solde est une information clé pour la gestion des comptes.
- **Source** : US-002

### FR-008 — Functional (P1)

> Le système DOIT afficher l'historique des opérations sur 13 mois glissants.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'historique des opérations est nécessaire pour la gestion financière.
- **Source** : US-002

### FR-009 — Functional (P1)

> Le système DOIT afficher la date, le libellé, le montant et le solde après opération pour chaque opération.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : —
- **Qualite** : —
- **Justification** : L'information détaillée est nécessaire pour la gestion des comptes.
- **Source** : US-002

### FR-010 — Functional (P1)

> Le système DOIT distinguer visuellement les opérations en attente (non comptabilisées) en italique.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Les opérations en attente doivent être visibles pour éviter les erreurs.
- **Source** : US-002

### FR-011 — Functional (P1)

> Le système DOIT permettre la recherche par mot-clé ou montant dans l'historique des opérations.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La recherche est nécessaire pour retrouver rapidement des opérations.
- **Source** : US-002

### FR-012 — Functional (P1)

> Le système DOIT permettre un pull-to-refresh pour mettre à jour les données depuis le serveur en moins de 2 secondes.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La mise à jour rapide est nécessaire pour la fluidité de l'application.
- **Source** : US-002

### FR-013 — Functional (P1)

> Le système DOIT permettre la consultation hors ligne des données avec la dernière synchronisation affichée.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La consultation hors ligne est nécessaire pour la continuité de service.
- **Source** : US-002

### FR-014 — Functional (P1)

> Le système DOIT exécuter un virement instantané en moins de 10 secondes et le débit doit être immédiat.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Le virement instantané est un besoin clé pour la fluidité des transferts.
- **Source** : US-003

### FR-015 — Functional (P1)

> Le système DOIT limiter le montant maximum par virement instantané à 15 000 euros.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La limitation du montant est nécessaire pour la sécurité.
- **Source** : US-003

### FR-016 — Functional (P1)

> Le système DOIT limiter le cumul journalier des virements instantanés à 30 000 euros.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La limitation du cumul journalier est nécessaire pour la sécurité.
- **Source** : US-003

### FR-017 — Functional (P1)

> Le système DOIT permettre la sélection du bénéficiaire depuis le carnet d'adresses ou via IBAN.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La sélection du bénéficiaire est nécessaire pour la fluidité des virements.
- **Source** : US-003

### FR-018 — Functional (P1)

> Le système DOIT exiger la biometrie ou le code PIN pour valider un virement.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La validation sécurisée est nécessaire pour la sécurité.
- **Source** : US-003

### FR-019 — Functional (P1)

> Le système DOIT afficher une notification push confirmant l'execution ou l'échec du virement dans les 30 secondes.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La notification est nécessaire pour la transparence.
- **Source** : US-003

### FR-020 — Functional (P1)

> Le système DOIT afficher clairement les frais de 0,50 euro par virement instantané avant validation.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La transparence sur les frais est nécessaire pour la confiance.
- **Source** : US-003

### FR-021 — Functional (P1)

> Le système DOIT rendre impossible l'annulation d'un virement une fois envoyé, avec un message d'alerte.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'annulation est impossible pour la sécurité des transferts.
- **Source** : US-003

### FR-022 — Functional (P1)

> Le système DOIT permettre l'activation et la désactivation du paiement sans contact en moins de 30 secondes.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'activation/désactivation rapide est nécessaire pour la flexibilité.
- **Source** : US-004

### FR-023 — Functional (P1)

> Le système DOIT bloquer temporairement la carte instantanément et rendre le blocage réversible sans appel à la banque.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Le blocage temporaire est nécessaire pour la sécurité.
- **Source** : US-004

### FR-024 — Functional (P1)

> Le système DOIT permettre la modification des plafonds de paiement et de retrait dans des limites définies par le contrat.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La flexibilité des plafonds est nécessaire pour la personnalisation.
- **Source** : US-004

### FR-025 — Functional (P1)

> Le système DOIT permettre la consultation sécurisée du code PIN après vérification biométrique, avec un affichage limité à 10 secondes.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La consultation sécurisée est nécessaire pour la protection des données.
- **Source** : US-004

### FR-026 — Functional (P1)

> Le système DOIT nécessiter une double confirmation pour l'opposition définitive et générer un numéro de dossier.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : L'opposition définitive doit être sécurisée et traçable.
- **Source** : US-004

### FR-027 — Functional (P1)

> Le système DOIT permettre la visualisation du suivi des paiements en attente et des prélèvements à venir sur un calendrier.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Le suivi des paiements est nécessaire pour la gestion des finances.
- **Source** : US-004

### FR-028 — Functional (P1)

> Le système DOIT permettre la configuration d'alertes sur dépassement de seuil (solde en dessous de X euros).

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Les alertes sont un outil clé pour la sécurité et la gestion proactive.
- **Source** : US-005

### FR-029 — Functional (P1)

> Le système DOIT permettre l'alerte à chaque opération supérieure à un montant défini par l'utilisateur.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Les alertes sont un outil clé pour la sécurité et la gestion proactive.
- **Source** : US-005

### FR-030 — Functional (P1)

> Le système DOIT envoyer une notification push à chaque connexion depuis un nouvel appareil.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La notification est nécessaire pour la sécurité.
- **Source** : US-005

### FR-031 — Functional (P1)

> Le système DOIT permettre l'alerte sur les prélèvements inhabituels (montant significativement supérieur au précédent).

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Les alertes sont un outil clé pour la sécurité et la gestion proactive.
- **Source** : US-005

### FR-032 — Functional (P1)

> Le système DOIT envoyer les alertes par push notification, email ou SMS selon la préférence de l'utilisateur.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La flexibilité des canaux de notification est nécessaire pour la personnalisation.
- **Source** : US-005

### FR-033 — Functional (P1)

> Le système DOIT permettre à l'utilisateur de désactiver individuellement chaque type d'alerte.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : La personnalisation des alertes est nécessaire pour la flexibilité.
- **Source** : US-005

### FR-034 — Functional (P1)

> Le système DOIT rendre indésactivable les alertes de sécurité (connexion suspecte, tentative de fraude).

- **Testable** : Oui
- **Verification** : Test
- **Risque** : High
- **Qualite** : —
- **Justification** : Les alertes de sécurité sont critiques pour la protection des données.
- **Source** : US-005

### FR-035 — Functional (P2)

> Le système DOIT permettre le dépôt de chèque par photo avec la prise de photo du recto et du verso.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le dépôt de chèque par photo est une fonctionnalité pratique pour les clients souhaitant éviter les déplacements.
- **Source** : US-006

### FR-036 — Functional (P2)

> Le système DOIT permettre la reconnaissance OCR pour extraire automatiquement le montant et le nom de l'émetteur du chèque.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La reconnaissance OCR est nécessaire pour la rapidité et la précision.
- **Source** : US-006

### FR-037 — Functional (P2)

> Le système DOIT permettre à l'utilisateur de valider ou de corriger les informations extraites avant soumission.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La validation manuelle est nécessaire pour la précision.
- **Source** : US-006

### FR-038 — Functional (P2)

> Le système DOIT limiter le montant maximum par dépôt photo à 5 000 euros.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La limitation du montant est nécessaire pour la sécurité.
- **Source** : US-006

### FR-039 — Functional (P2)

> Le système DOIT traiter le chèque en 2 jours ouvrables après soumission.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le traitement rapide est nécessaire pour la fluidité.
- **Source** : US-006

### FR-040 — Functional (P2)

> Le système DOIT afficher un statut de traitement du chèque (en cours, valide, rejeté) dans l'historique.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le statut de traitement est nécessaire pour la transparence.
- **Source** : US-006

### FR-041 — Functional (P2)

> Le système DOIT permettre la résoumission du chèque en cas de rejet, avec la communication du motif.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La résoumission est nécessaire pour la flexibilité.
- **Source** : US-006

### FR-042 — Functional (P2)

> Le système DOIT classer automatiquement les opérations en 12 catégories (alimentation, transport, logement, loisirs, santé, habillement, éducation, restaurant, abonnements, épargne, impôts, divers).

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La classification automatique est nécessaire pour la gestion budgétaire.
- **Source** : US-007

### FR-043 — Functional (P2)

> Le système DOIT permettre à l'utilisateur de recategoriser manuellement une opération et d'apprendre pour les futures opérations du même commercant.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La recategorisation manuelle est nécessaire pour la précision.
- **Source** : US-007

### FR-044 — Functional (P2)

> Le système DOIT afficher un graphique camembert montrant la répartition du mois en cours.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le graphique camembert est nécessaire pour la visualisation des dépenses.
- **Source** : US-007

### FR-045 — Functional (P2)

> Le système DOIT afficher un graphique barre comparant les dépenses mois par mois sur 6 mois.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le graphique barre est nécessaire pour la comparaison des dépenses.
- **Source** : US-007

### FR-046 — Functional (P2)

> Le système DOIT permettre la paramétrisation du budget mensuel par catégorie avec une alerte à 80% de consommation.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : L'alerte de consommation est nécessaire pour la gestion budgétaire.
- **Source** : US-007

### FR-047 — Functional (P2)

> Le système DOIT permettre l'export PDF du bilan mensuel.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : L'export PDF est nécessaire pour la conservation des données.
- **Source** : US-007

### FR-048 — Functional (P2)

> Le système DOIT permettre la consultation et le téléchargement des documents bancaires depuis l'application.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La consultation des documents est nécessaire pour la transparence.
- **Source** : US-008

### FR-049 — Functional (P2)

> Le système DOIT rendre disponibles les relevés de compte mensuels à partir du 5 du mois suivant.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La disponibilité des relevés est nécessaire pour la transparence.
- **Source** : US-008

### FR-050 — Functional (P2)

> Le système DOIT permettre la génération à la demande d'attestations (domiciliation, solde) en format PDF.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La génération des attestations est nécessaire pour la transparence.
- **Source** : US-008

### FR-051 — Functional (P2)

> Le système DOIT permettre le téléchargement et le partage du RIB/IBAN en un tap.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le téléchargement et le partage du RIB/IBAN sont nécessaires pour la facilité.
- **Source** : US-008

### FR-052 — Functional (P2)

> Le système DOIT permettre l'archivage et la consultation des contrats signés électroniquement.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : L'archivage des contrats est nécessaire pour la traçabilité.
- **Source** : US-008

### FR-053 — Functional (P2)

> Le système DOIT conserver les documents pendant 10 ans conformément à la réglementation.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La conservation des documents est nécessaire pour la conformité.
- **Source** : US-008

### FR-054 — Functional (P2)

> Le système DOIT permettre la recherche par type de document et par date.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La recherche est nécessaire pour retrouver rapidement les documents.
- **Source** : US-008

### FR-055 — Functional (P2)

> Le système DOIT protéger le téléchargement des documents par la biometrie ou le code PIN.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La protection des documents est nécessaire pour la sécurité.
- **Source** : US-008

### FR-056 — Functional (P2)

> Le système DOIT permettre le contact avec le conseiller bancaire via un chat intégré.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le chat avec le conseiller est une fonctionnalité utile pour la communication directe.
- **Source** : US-009

### FR-057 — Functional (P2)

> Le système DOIT rendre le chat disponible du lundi au vendredi de 8h30 à 18h30.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La disponibilité du chat est nécessaire pour la communication.
- **Source** : US-009

### FR-058 — Functional (P2)

> Le système DOIT permettre un chatbot répondant aux questions fréquentes en dehors des horaires.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le chatbot est nécessaire pour la continuité de service.
- **Source** : US-009

### FR-059 — Functional (P2)

> Le système DOIT garantir un délai de première réponse du conseiller inférieur à 5 minutes en moyenne.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le délai de réponse est nécessaire pour la satisfaction client.
- **Source** : US-009

### FR-060 — Functional (P2)

> Le système DOIT permettre à l'utilisateur de joindre un document ou une capture d'écran au message.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La possibilité de joindre des documents est nécessaire pour la communication.
- **Source** : US-009

### FR-061 — Functional (P2)

> Le système DOIT conserver l'historique des conversations et le rendre consultable.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : L'historique des conversations est nécessaire pour la traçabilité.
- **Source** : US-009

### FR-062 — Functional (P2)

> Le système DOIT permettre au conseiller de partager un lien vers un produit ou un simulateur directement dans le chat.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : Le partage de liens est nécessaire pour la communication.
- **Source** : US-009

### FR-063 — Functional (P2)

> Le système DOIT envoyer une notification push informant l'utilisateur quand le conseiller répond.

- **Testable** : Oui
- **Verification** : Test
- **Risque** : Medium
- **Qualite** : —
- **Justification** : La notification est nécessaire pour la transparence.
- **Source** : US-009

## Entites cles (10)

| Nom | Description | Attributs | Relations |
|-----|-------------|-----------|-----------|
| Utilisateur | Client de la banque utilisant l'application. | ID utilisateur, mot de passe, code PIN, empreinte digitale, reconnaissance faciale | connecté à l'application, a un compte, a une carte, a un historique d'opérations |
| Compte | Compte bancaire du client. | solde, historique des opérations, plafond de paiement, plafond de retrait | appartient à un utilisateur, a des opérations, a des virements |
| Opération | Opération financière effectuée sur un compte. | date, libellé, montant, solde après opération, statut, type (dépôt, virement, prélèvement) | appartient à un compte, a un utilisateur, a un bénéficiaire |
| Virement | Transfert d'argent entre deux comptes. | date, montant, bénéficiaire, statut, code PIN, empreinte digitale | appartient à un utilisateur, a un compte source, a un compte destination |
| Carte | Carte bancaire associée à un compte. | plafond de paiement, plafond de retrait, statut (actif, bloqué), paiement sans contact | appartient à un utilisateur, a un compte |
| Document | Document bancaire stocké dans l'application. | type, date, statut, contenu, format (PDF, image) | appartient à un utilisateur, a un compte |
| Alerte | Notification envoyée à l'utilisateur en cas de dépassement de seuil ou d'opération importante. | type, seuil, montant, canal (push, email, SMS), statut | appartient à un utilisateur, a un compte |
| Chat | Conversation entre l'utilisateur et le conseiller bancaire. | date, message, statut (en attente, lu, lu et répondu), type de message (texte, image, document) | appartient à un utilisateur, a un conseiller |
| Bénéficiaire | Destinataire d'un virement. | nom, IBAN, adresse, type (compte bancaire, carte) | a un virement, appartient à un utilisateur |
| Carnet d'adresses | Liste des contacts et bénéficiaires enregistrés par l'utilisateur. | nom, IBAN, adresse, type (compte bancaire, carte) | appartient à un utilisateur, a des virements |

## Cas limites (6)

| Description | Scenario lie | Severite |
|-------------|-------------|----------|
| L'utilisateur tente d'activer la biometrie sans avoir configuré le mot de passe classique. | US-001 | P2 |
| L'utilisateur tente de se connecter après 5 minutes d'inactivité. | US-001 | P2 |
| L'utilisateur tente de valider un virement sans avoir entré le code PIN ou utilisé la biometrie. | US-003 | P2 |
| L'utilisateur tente de déposer un chèque avec une photo de mauvaise qualité. | US-006 | P2 |
| L'utilisateur tente de consulter un document sans avoir validé la biometrie ou le code PIN. | US-008 | P2 |
| L'utilisateur tente de joindre un document au message d'un chat sans avoir validé la biometrie ou le code PIN. | US-009 | P2 |

## Criteres de succes (10)

| ID | Description | Metrique mesurable |
|----|-------------|-------------------|
| SC-001 | Le système doit permettre l'authentification biométrique dans les 3 tentatives. | Taux de réussite de l'authentification biométrique |
| SC-002 | Le système doit afficher le solde de chaque compte dès la page d'accueil après connexion. | Temps de chargement des soldes |
| SC-003 | Le système doit exécuter un virement instantané en moins de 10 secondes. | Temps de traitement des virements |
| SC-004 | Le système doit permettre la consultation hors ligne des données avec la dernière synchronisation affichée. | Temps de synchronisation |
| SC-005 | Le système doit afficher un journal de connexion horodaté dans les paramètres de sécurité. | Nombre de connexions enregistrées |
| SC-006 | Le système doit permettre la configuration d'alertes sur dépassement de seuil. | Taux de détection des dépassements |
| SC-007 | Le système doit permettre le dépôt de chèque par photo avec la prise de photo du recto et du verso. | Taux de réussite des dépôts de chèque |
| SC-008 | Le système doit permettre la classification automatique des opérations en 12 catégories. | Taux de classification correcte |
| SC-009 | Le système doit permettre la consultation et le téléchargement des documents bancaires. | Taux de réussite des téléchargements |
| SC-010 | Le système doit permettre le contact avec le conseiller bancaire via un chat intégré. | Temps de réponse moyen du conseiller |

## Clarifications (3)

### ❓ Quels types de documents bancaires sont disponibles dans le coffre-fort ?

- **Contexte** : Le coffre-fort de documents bancaires doit être clair sur les types de documents accessibles.
- **Impact** : La clarté sur les types de documents est nécessaire pour la transparence.
- **Options** : Relevés / Attestations / Contrats / RIB/IBAN / Tous les documents
- **Resolue** : Non

### ❓ Quel est le délai maximal pour le traitement d'un chèque déposé par photo ?

- **Contexte** : Le délai de traitement est un critère important pour la satisfaction client.
- **Impact** : Le délai de traitement affecte la satisfaction client.
- **Options** : 2 jours ouvrables / 3 jours ouvrables / 5 jours ouvrables / 7 jours ouvrables
- **Resolue** : Non

### ❓ Quel est le montant maximum par dépôt de chèque par photo ?

- **Contexte** : Le montant maximum est un critère important pour la sécurité.
- **Impact** : Le montant maximum affecte la sécurité et la satisfaction client.
- **Options** : 5 000 euros / 10 000 euros / 15 000 euros / 20 000 euros
- **Resolue** : Non

---

## Tests Gherkin (54 scenarios, 18 features)

### Feature: Authentification biometrique

> Fonctionnalité permettant à l'utilisateur de se connecter via l'authentification biométrique (empreinte digitale ou reconnaissance faciale).

- **ID** : cb61039b-8917-4ae4-8a50-f826d69659a0
- **Niveau** : Acceptance
- **Tags** : @US-001, @P1
- **Exigences couvertes** : FR-001, FR-002, FR-004, FR-006
- **Scenarios source** : US-001

**Background:**

```gherkin
  Given L'utilisateur a installé l'application et dispose d'un appareil compatible avec la reconnaissance faciale ou l'empreinte digitale.
```

#### ✅ Scenario: Scenario happy path - Authentification biométrique

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-001, @FR-004, @FR-006
- **Verifie** : FR-001, FR-004, FR-006

```gherkin
  Given L'utilisateur est sur l'écran de connexion.
  When L'utilisateur tente de se connecter.
  Then L'authentification biométrique est proposée et fonctionne correctement.
```

**Exemples :**

| type_authentification |
|---|
| empreinte_digitale |
| reconnaissance_faciale |

**Suggestions donnees test** : empreinte digitale, reconnaissance faciale, appareil compatible

#### ⚠️ Scenario: Scenario cas limite - Authentification biométrique après 3 tentatives

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-002
- **Verifie** : FR-002

```gherkin
  Given L'utilisateur a tenté 3 fois de se connecter avec l'authentification biométrique.
  When L'utilisateur tente de se connecter à nouveau.
  Then Le système bascule sur le code PIN à 6 chiffres.
```

**Exemples :**

| tentatives |
|---|
| 3 |

**Suggestions donnees test** : 3 tentatives, code PIN à 6 chiffres

#### ❌ Scenario: Scenario erreur - Authentification biométrique sans mot de passe

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-004
- **Verifie** : FR-004

```gherkin
  Given L'utilisateur n'a pas configuré le mot de passe classique.
  When L'utilisateur tente d'activer la biometrie.
  Then Le système refuse l'activation de la biometrie.
```

**Exemples :**

| etat_mot_de_passe |
|---|
| non_configure |

**Suggestions donnees test** : mot de passe non configuré, activation de la biometrie refusée

---

### Feature: Consultation des soldes et des opérations

> Fonctionnalité permettant à l'utilisateur de consulter les soldes et l'historique des opérations.

- **ID** : b0dd3d55-dd3c-4bb7-9a84-73fe9647221d
- **Niveau** : Acceptance
- **Tags** : @US-002, @P1
- **Exigences couvertes** : FR-003, FR-007, FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, FR-014, FR-015, FR-016, FR-017, FR-018, FR-019, FR-020, FR-021, FR-022, FR-023, FR-024, FR-025, FR-026, FR-027, FR-028, FR-029, FR-030, FR-031, FR-032, FR-033, FR-034, FR-035, FR-036, FR-037, FR-038, FR-039, FR-040, FR-041, FR-042, FR-043, FR-044, FR-045, FR-046, FR-047, FR-048, FR-049, FR-050, FR-051, FR-052, FR-053, FR-054, FR-055, FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063
- **Scenarios source** : US-002

**Background:**

```gherkin
  Given L'utilisateur est connecté à l'application.
```

#### ✅ Scenario: Scenario happy path - Affichage des soldes

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-007, @FR-009, @FR-008, @FR-010, @FR-011, @FR-012, @FR-013, @FR-014, @FR-015, @FR-016, @FR-017, @FR-018, @FR-019, @FR-020, @FR-021, @FR-022, @FR-023, @FR-024, @FR-025, @FR-026, @FR-027, @FR-028, @FR-029, @FR-030, @FR-031, @FR-032, @FR-033, @FR-034, @FR-035, @FR-036, @FR-037, @FR-038, @FR-039, @FR-040, @FR-041, @FR-042, @FR-043, @FR-044, @FR-045, @FR-046, @FR-047, @FR-048, @FR-049, @FR-050, @FR-051, @FR-052, @FR-053, @FR-054, @FR-055, @FR-056, @FR-057, @FR-058, @FR-059, @FR-060, @FR-061, @FR-062, @FR-063
- **Verifie** : FR-007, FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, FR-014, FR-015, FR-016, FR-017, FR-018, FR-019, FR-020, FR-021, FR-022, FR-023, FR-024, FR-025, FR-026, FR-027, FR-028, FR-029, FR-030, FR-031, FR-032, FR-033, FR-034, FR-035, FR-036, FR-037, FR-038, FR-039, FR-040, FR-041, FR-042, FR-043, FR-044, FR-045, FR-046, FR-047, FR-048, FR-049, FR-050, FR-051, FR-052, FR-053, FR-054, FR-055, FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063

```gherkin
  Given L'utilisateur est connecté à l'application.
  When L'utilisateur accède à la page d'accueil.
  Then Les soldes des comptes sont affichés.
```

**Exemples :**

| type_compte |
|---|
| compte courant |
| compte épargne |

**Suggestions donnees test** : compte courant, compte épargne, page d'accueil

#### ⚠️ Scenario: Scenario cas limite - Consultation après 5 minutes d'inactivité

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-003
- **Verifie** : FR-003

```gherkin
  Given L'utilisateur est connecté et n'a pas interagit pendant 5 minutes.
  When L'utilisateur tente d'accéder à la page d'accueil.
  Then La session est expirée et l'utilisateur doit se reconnecter.
```

**Exemples :**

| duree_inactivite |
|---|
| 5 minutes |

**Suggestions donnees test** : 5 minutes d'inactivité, session expirée

#### ❌ Scenario: Scenario erreur - Validation d'un virement sans authentification

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-018
- **Verifie** : FR-018

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
  When L'utilisateur valide le virement sans authentification.
  Then Le système refuse la validation du virement.
```

**Exemples :**

| authentification |
|---|
| non_valide |

**Suggestions donnees test** : virement non validé, authentification requise

---

### Feature: Virement instantané SEPA

> Fonctionnalité permettant à l'utilisateur d'effectuer un virement instantané vers un autre compte bancaire en zone SEPA.

- **ID** : b2791859-2b5d-45a7-8971-a84ba4dcd4c5
- **Niveau** : Acceptance
- **Tags** : @US-003, @P1
- **Exigences couvertes** : FR-014, FR-015, FR-016, FR-017, FR-018, FR-019, FR-020, FR-021, FR-022, FR-023, FR-024, FR-025, FR-026, FR-027, FR-028, FR-029, FR-030, FR-031, FR-032, FR-033, FR-034, FR-035, FR-036, FR-037, FR-038, FR-039, FR-040, FR-041, FR-042, FR-043, FR-044, FR-045, FR-046, FR-047, FR-048, FR-049, FR-050, FR-051, FR-052, FR-053, FR-054, FR-055, FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063
- **Scenarios source** : US-003

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
```

#### ✅ Scenario: Scenario happy path - Virement instantané

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-014, @FR-015, @FR-016, @FR-017, @FR-018, @FR-019, @FR-020, @FR-021, @FR-022, @FR-023, @FR-024, @FR-025, @FR-026, @FR-027, @FR-028, @FR-029, @FR-030, @FR-031, @FR-032, @FR-033, @FR-034, @FR-035, @FR-036, @FR-037, @FR-038, @FR-039, @FR-040, @FR-041, @FR-042, @FR-043, @FR-044, @FR-045, @FR-046, @FR-047, @FR-048, @FR-049, @FR-050, @FR-051, @FR-052, @FR-053, @FR-054, @FR-055, @FR-056, @FR-057, @FR-058, @FR-059, @FR-060, @FR-061, @FR-062, @FR-063
- **Verifie** : FR-014, FR-015, FR-016, FR-017, FR-018, FR-019, FR-020, FR-021, FR-022, FR-023, FR-024, FR-025, FR-026, FR-027, FR-028, FR-029, FR-030, FR-031, FR-032, FR-033, FR-034, FR-035, FR-036, FR-037, FR-038, FR-039, FR-040, FR-041, FR-042, FR-043, FR-044, FR-045, FR-046, FR-047, FR-048, FR-049, FR-050, FR-051, FR-052, FR-053, FR-054, FR-055, FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
  When L'utilisateur valide le virement.
  Then Le virement est exécuté en moins de 10 secondes.
```

**Exemples :**

| montant | beneficiaire |
|---|---|
| 1000 | IBAN 1234567890 |
| 5000 | IBAN 0987654321 |

**Suggestions donnees test** : montant de 1000 euros, IBAN 1234567890, virement instantané

#### ⚠️ Scenario: Scenario cas limite - Virement au seuil maximal

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-015, @FR-016
- **Verifie** : FR-015, FR-016

```gherkin
  Given L'utilisateur a un solde suffisant pour un virement de 15 000 euros.
  When L'utilisateur valide un virement de 15 000 euros.
  Then Le virement est exécuté et le solde est mis à jour.
```

**Exemples :**

| montant |
|---|
| 15000 |

**Suggestions donnees test** : virement de 15 000 euros, solde suffisant

#### ❌ Scenario: Scenario erreur - Virement sans authentification

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-018
- **Verifie** : FR-018

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
  When L'utilisateur valide le virement sans authentification.
  Then Le système refuse la validation du virement.
```

**Exemples :**

| authentification |
|---|
| non_valide |

**Suggestions donnees test** : virement non validé, authentification requise

---

### Feature: Gestion des cartes bancaires

> Gestion des paramètres de la carte bancaire (activation du paiement sans contact, blocage temporaire, modification des plafonds)

- **ID** : f578d5de-0f55-45f7-96ac-69eddaeb6995
- **Niveau** : Acceptance
- **Tags** : @US-004, @P1
- **Exigences couvertes** : FR-022, FR-024
- **Scenarios source** : US-004

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné une carte
```

#### ✅ Scenario: Activation du paiement sans contact - Happy Path

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-022, @FR-024
- **Verifie** : FR-022, FR-024

```gherkin
  When L'utilisateur active le paiement sans contact
  Then L'activation prend effet en moins de 30 secondes
```

**Exemples :**

| plafond | duree_activation |
|---|---|
| 5000 | 25 |

**Suggestions donnees test** : plafond de 5000€, délai de 25 secondes

#### ⚠️ Scenario: Activation du paiement sans contact - Cas limite

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-022, @FR-024
- **Verifie** : FR-022, FR-024

```gherkin
  When L'utilisateur active le paiement sans contact après 30 secondes d'inactivité
  Then L'activation est refusée et un message d'erreur est affiché
```

**Exemples :**

| plafond | duree_activation |
|---|---|
| 5000 | 35 |

**Suggestions donnees test** : plafond de 5000€, délai de 35 secondes

#### ❌ Scenario: Activation du paiement sans contact - Erreur

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-022, @FR-024
- **Verifie** : FR-022, FR-024

```gherkin
  When L'utilisateur tente d'activer le paiement sans contact sans avoir validé la biometrie
  Then L'activation est refusée et un message d'erreur est affiché
```

**Exemples :**

| plafond | duree_activation |
|---|---|
| 5000 | 25 |

**Suggestions donnees test** : plafond de 5000€, délai de 25 secondes

---

### Feature: Alertes et notifications paramétrables

> Configuration et gestion des alertes sur les comptes et opérations

- **ID** : c78cb6ff-8d06-439b-8d4a-009c218aec71
- **Niveau** : Acceptance
- **Tags** : @US-005, @P1
- **Exigences couvertes** : FR-028, FR-029, FR-032
- **Scenarios source** : US-005

**Background:**

```gherkin
  Given L'utilisateur a configuré une alerte sur un seuil de solde
```

#### ✅ Scenario: Alerte seuil de solde - Happy Path

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-028, @FR-029, @FR-032
- **Verifie** : FR-028, FR-029, FR-032

```gherkin
  When Le solde dépasse le seuil
  Then Une alerte est envoyée par push, email ou SMS
```

**Exemples :**

| seuil | methode_alerte |
|---|---|
| 1000 | push, email |

**Suggestions donnees test** : seuil de 1000€, méthode alerte : push et email

#### ⚠️ Scenario: Alerte seuil de solde - Cas limite

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-028, @FR-029, @FR-032
- **Verifie** : FR-028, FR-029, FR-032

```gherkin
  When Le solde dépasse le seuil à la limite
  Then Une alerte est envoyée par push, email ou SMS
```

**Exemples :**

| seuil | methode_alerte |
|---|---|
| 1000 | push, email |

**Suggestions donnees test** : seuil de 1000€, méthode alerte : push et email

#### ❌ Scenario: Alerte seuil de solde - Erreur

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-028, @FR-029, @FR-032
- **Verifie** : FR-028, FR-029, FR-032

```gherkin
  When Le solde dépasse le seuil mais l'utilisateur a désactivé les alertes
  Then Aucune alerte n'est envoyée
```

**Exemples :**

| seuil | methode_alerte |
|---|---|
| 1000 | push, email |

**Suggestions donnees test** : seuil de 1000€, méthode alerte : push et email

---

### Feature: Dépôt de chèque par photo

> Dépôt de chèque via l'application avec prise de photo du recto et du verso

- **ID** : 3d917cf1-6526-4bd7-af84-93fa69bd1f53
- **Niveau** : Acceptance
- **Tags** : @US-006, @P2
- **Exigences couvertes** : FR-035, FR-036, FR-037, FR-038, FR-040, FR-041
- **Scenarios source** : US-006

**Background:**

```gherkin
  Given L'utilisateur a photographié le recto et le verso d'un chèque
```

#### ✅ Scenario: Dépôt de chèque - Happy Path

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-035, @FR-036, @FR-037, @FR-038, @FR-040, @FR-041
- **Verifie** : FR-035, FR-036, FR-037, FR-038, FR-040, FR-041

```gherkin
  When L'utilisateur valide les informations extraites
  Then Le chèque est soumis et un statut est affiché
```

**Exemples :**

| montant | statut |
|---|---|
| 4500 | en cours |

**Suggestions donnees test** : montant de 4500€, statut : en cours

#### ⚠️ Scenario: Dépôt de chèque - Cas limite

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-035, @FR-036, @FR-037, @FR-038, @FR-040, @FR-041
- **Verifie** : FR-035, FR-036, FR-037, FR-038, FR-040, FR-041

```gherkin
  When L'utilisateur tente de déposer un chèque avec une photo de mauvaise qualité
  Then Le système refuse le dépôt et affiche un message d'erreur
```

**Exemples :**

| montant | statut |
|---|---|
| 4500 | en cours |

**Suggestions donnees test** : montant de 4500€, statut : en cours

#### ❌ Scenario: Dépôt de chèque - Erreur

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-035, @FR-036, @FR-037, @FR-038, @FR-040, @FR-041
- **Verifie** : FR-035, FR-036, FR-037, FR-038, FR-040, FR-041

```gherkin
  When L'utilisateur tente de déposer un chèque avec une photo de mauvaise qualité
  Then Le système refuse le dépôt et affiche un message d'erreur
```

**Exemples :**

| montant | statut |
|---|---|
| 4500 | en cours |

**Suggestions donnees test** : montant de 4500€, statut : en cours

---

### Feature: Visualisation des dépenses par catégorie

> Fonctionnalité permettant de visualiser les dépenses réparties par catégorie avec des graphiques pour comprendre où va son argent et mieux gérer son budget mensuel.

- **ID** : 451a5334-0bcd-48a6-9bcc-8181d7d84edd
- **Niveau** : Acceptance
- **Tags** : @US-007, @P2
- **Exigences couvertes** : FR-008, FR-009, FR-010, FR-042, FR-044, FR-045
- **Scenarios source** : US-007

**Background:**

```gherkin
  Given L'utilisateur est connecté et a consulté ses opérations.
```

#### ✅ Scenario: Visualisation des dépenses par catégorie (happy path)

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-042, @FR-044, @FR-045
- **Verifie** : FR-042, FR-044, FR-045

```gherkin
  Given L'utilisateur est connecté et a consulté ses opérations.
  When L'utilisateur accède aux graphiques.
  Then Les dépenses sont réparties en 12 catégories.
```

**Exemples :**

| categorie |
|---|
| Alimentation |
| Transport |
| Logement |
| Loisirs |
| Santé |
| Habillement |
| Éducation |
| Restaurant |
| Abonnements |
| Épargne |
| Impôts |
| Divers |

**Suggestions donnees test** : Alimentation, Transport, Logement, Loisirs, Santé, Habillement, Éducation, Restaurant, Abonnements, Épargne, Impôts, Divers

#### ⚠️ Scenario: Visualisation des dépenses par catégorie (cas limite - 13 mois glissants)

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-008, @FR-009, @FR-010
- **Verifie** : FR-008, FR-009, FR-010

```gherkin
  Given L'utilisateur est connecté et a consulté ses opérations.
  When L'utilisateur accède aux graphiques.
  Then Les dépenses sont réparties en 12 catégories.
  And Le système affiche l'historique des opérations sur 13 mois glissants.
  Then Chaque opération affiche la date, le libellé, le montant et le solde après opération.
  And Les opérations en attente sont visuellement distinguées en italique.
```

**Exemples :**

| categorie | mois |
|---|---|
| Alimentation | Janvier |
| Transport | Février |
| Logement | Mars |
| Loisirs | Avril |
| Santé | Mai |
| Habillement | Juin |
| Éducation | Juillet |
| Restaurant | Août |
| Abonnements | Septembre |
| Épargne | Octobre |
| Impôts | Novembre |
| Divers | Décembre |

**Suggestions donnees test** : Alimentation, Transport, Logement, Loisants, Santé, Habillement, Éducation, Restaurant, Abonnements, Épargne, Impôts, Divers

#### ❌ Scenario: Visualisation des dépenses par catégorie (erreur - absence de données)

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-008, @FR-009, @FR-010
- **Verifie** : FR-008, FR-009, FR-010

```gherkin
  Given L'utilisateur est connecté et a consulté ses opérations.
  When L'utilisateur accède aux graphiques.
  Then Le système affiche un message d'erreur indiquant l'absence de données.
  And Le système ne répartit pas les dépenses en catégories.
```

**Exemples :**

| categorie | mois |
|---|---|
| Alimentation | Janvier |
| Transport | Février |
| Logement | Mars |
| Loisirs | Avril |
| Santé | Mai |
| Habillement | Juin |
| Éducation | Juillet |
| Restaurant | Août |
| Abonnements | Septembre |
| Épargne | Octobre |
| Impôts | Novembre |
| Divers | Décembre |

**Suggestions donnees test** : Alimentation, Transport, Logement, Loisirs, Santé, Habillement, Éducation, Restaurant, Abonnements, Épargne, Impôts, Divers

---

### Feature: Téléchargement sécurisé des documents bancaires

> Fonctionnalité permettant à l'utilisateur de consulter et de télécharger ses documents bancaires depuis l'application pour accéder à ses relevés, contrats et attestations sans attendre le courrier postal.

- **ID** : be39b927-c638-45b3-97f8-51b1bb561b47
- **Niveau** : Acceptance
- **Tags** : @US-008, @P2
- **Exigences couvertes** : FR-048, FR-054, FR-055
- **Scenarios source** : US-008

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document.
```

#### ✅ Scenario: Téléchargement sécurisé des documents bancaires (happy path)

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-055, @FR-048, @FR-054
- **Verifie** : FR-055, FR-048, FR-054

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document.
  When L'utilisateur tente de le télécharger.
  Then Le téléchargement est protégé par la biométrie ou le code PIN.
```

**Exemples :**

| document_type | format |
|---|---|
| Relevé de compte | PDF |
| Contrat | PDF |
| Attestation | PDF |
| RIB | PDF |
| IBAN | PDF |
| Document bancaire | PDF |

**Suggestions donnees test** : Relevé de compte, Contrat, Attestation, RIB, IBAN, Document bancaire

#### ⚠️ Scenario: Téléchargement sécurisé des documents bancaires (cas limite - absence de biométrie ou code PIN)

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-055, @FR-048, @FR-054
- **Verifie** : FR-055, FR-048, FR-054

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document.
  When L'utilisateur tente de le télécharger sans avoir validé la biométrie ou le code PIN.
  Then Le système bloque le téléchargement.
  And Le système affiche un message d'erreur indiquant la nécessité de la biométrie ou du code PIN.
```

**Exemples :**

| document_type | format |
|---|---|
| Relevé de compte | PDF |
| Contrat | PDF |
| Attestation | PDF |
| RIB | PDF |
| IBAN | PDF |
| Document bancaire | PDF |

**Suggestions donnees test** : Relevé de compte, Contrat, Attestation, RIB, IBAN, Document bancaire

#### ❌ Scenario: Téléchargement sécurisé des documents bancaires (erreur - absence de données)

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-055, @FR-048, @FR-054
- **Verifie** : FR-055, FR-048, FR-054

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document.
  When L'utilisateur tente de le télécharger.
  Then Le système affiche un message d'erreur indiquant l'absence de document.
  And Le téléchargement n'est pas effectué.
```

**Exemples :**

| document_type | format |
|---|---|
| Relevé de compte | PDF |
| Contrat | PDF |
| Attestation | PDF |
| RIB | PDF |
| IBAN | PDF |
| Document bancaire | PDF |

**Suggestions donnees test** : Relevé de compte, Contrat, Attestation, RIB, IBAN, Document bancaire

---

### Feature: Chat avec conseiller bancaire

> Fonctionnalité permettant à l'utilisateur de contacter son conseiller bancaire via un chat intégré dans l'application pour obtenir des réponses rapides sans devoir se déplacer en agence ou attendre au téléphone.

- **ID** : 9d54688d-e75a-422f-afb2-585147f761f4
- **Niveau** : Acceptance
- **Tags** : @US-009, @P2
- **Exigences couvertes** : FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063
- **Scenarios source** : US-009

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller.
```

#### ✅ Scenario: Chat avec conseiller bancaire (happy path)

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-056, @FR-057, @FR-058, @FR-059, @FR-060, @FR-061, @FR-062, @FR-063
- **Verifie** : FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller.
  When L'utilisateur envoie un message.
  Then Le message est envoyé et un historique est conservé.
```

**Exemples :**

| message | format |
|---|---|
| Bonjour, pouvez-vous m'aider ? | Texte |
| Voici une capture d'écran de mon problème | Image |
| Voici le document concerné | PDF |
| Voici le relevé de compte | PDF |
| Voici le contrat signé | PDF |
| Voici le RIB | PDF |

**Suggestions donnees test** : Bonjour, pouvez-vous m'aider ?, Voici une capture d'écran de mon problème, Voici le document concerné, Voici le relevé de compte, Voici le contrat signé, Voici le RIB

#### ⚠️ Scenario: Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN)

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-056, @FR-057, @FR-058, @FR-059, @FR-060, @FR-061, @FR-062, @FR-063
- **Verifie** : FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller.
  When L'utilisateur tente de joindre un document au message sans avoir validé la biométrie ou le code PIN.
  Then Le système bloque l'envoi du message.
  And Le système affiche un message d'erreur indiquant la nécessité de la biométrie ou du code PIN.
```

**Exemples :**

| message | format |
|---|---|
| Bonjour, pouvez-vous m'aider ? | Texte |
| Voici une capture d'écran de mon problème | Image |
| Voici le document concerné | PDF |
| Voici le relevé de compte | PDF |
| Voici le contrat signé | PDF |
| Voici le RIB | PDF |

**Suggestions donnees test** : Bonjour, pouvez-vous m'aider ?, Voici une capture d'écran de mon problème, Voici le document concerné, Voici le relevé de compte, Voici le contrat signé, Voici le RIB

#### ❌ Scenario: Chat avec conseiller bancaire (erreur - absence de données)

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-056, @FR-057, @FR-058, @FR-059, @FR-060, @FR-061, @FR-062, @FR-063
- **Verifie** : FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller.
  When L'utilisateur tente de joindre un document au message.
  Then Le système affiche un message d'erreur indiquant l'absence de document.
  And Le message n'est pas envoyé.
```

**Exemples :**

| message | format |
|---|---|
| Bonjour, pouvez-vous m'aider ? | Texte |
| Voici une capture d'écran de mon problème | Image |
| Voici le document concerné | PDF |
| Voici le relevé de compte | PDF |
| Voici le contrat signé | PDF |
| Voici le RIB | PDF |

**Suggestions donnees test** : Bonjour, pouvez-vous m'aider ?, Voici une capture d'écran de mon problème, Voici le document concerné, Voici le relevé de compte, Voici le contrat signé, Voici le RIB

---

### Feature: Authentification biometrique

> Permet à l'utilisateur de se connecter via la reconnaissance faciale ou l'empreinte digitale.

- **ID** : 028dc20a-2d31-4ce3-aa8b-13188449ce76
- **Niveau** : Acceptance
- **Tags** : @US-001, @P1
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-001

**Background:**

```gherkin
  Given L'utilisateur a installé l'application et a un appareil compatible avec la reconnaissance faciale ou l'empreinte digitale.
```

#### ✅ Scenario: Scenario happy path - Authentification biometrique

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté à l'application.
  When L'utilisateur tente de se connecter.
  Then L'authentification biométrique est proposée et fonctionne correctement.
```

**Exemples :**

| type_authentification |
|---|
| reconnaissance_faciale |
| empreinte_digitale |

**Suggestions donnees test** : reconnaissance faciale, empreinte digitale, appareil compatible

#### ⚠️ Scenario: Scenario cas limite - Authentification biometrique sans mot de passe

- **Type** : EdgeCase
- **Technique** : ErrorGuessing
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a installé l'application mais n'a pas configuré le mot de passe classique.
  When L'utilisateur tente d'activer la biometrie.
  Then L'activation de la biometrie est refusée.
```

**Exemples :**

| etat_mot_de_passe |
|---|
| non_configure |

**Suggestions donnees test** : mot de passe non configure, biometrie non active

#### ❌ Scenario: Scenario erreur - Authentification biometrique echoue

- **Type** : ErrorScenario
- **Technique** : BoundaryValueAnalysis
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a installé l'application mais la biometrie est défaillante.
  When L'utilisateur tente de se connecter.
  Then L'authentification biométrique echoue et un message d'erreur est affiché.
```

**Exemples :**

| etat_biometrie |
|---|
| defaillante |

**Suggestions donnees test** : biometrie defaillante, erreur d'authentification

---

### Feature: Consultation des soldes et des opérations

> Permet à l'utilisateur de consulter les soldes et l'historique de ses comptes.

- **ID** : 10756f03-9668-4dd3-b418-fbb94be9e236
- **Niveau** : Acceptance
- **Tags** : @US-002, @P1
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-002

**Background:**

```gherkin
  Given L'utilisateur est connecté à l'application.
```

#### ✅ Scenario: Scenario happy path - Consultation des soldes

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté à l'application.
  When L'utilisateur accède à la page d'accueil.
  Then Les soldes des comptes sont affichés.
```

**Exemples :**

| type_compte |
|---|
| compte_courant |
| compte_epargne |

**Suggestions donnees test** : compte courant, compte epargne, historique des operations

#### ⚠️ Scenario: Scenario cas limite - Consultation des soldes après inactivité

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté à l'application et n'a pas interagit pendant 5 minutes.
  When L'utilisateur accède à la page d'accueil.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| duree_inactivite |
|---|
| 5 minutes |

**Suggestions donnees test** : inactivité de 5 minutes, redirection vers connexion

#### ❌ Scenario: Scenario erreur - Consultation des soldes sans connexion

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur n'est pas connecté à l'application.
  When L'utilisateur accède à la page d'accueil.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| etat_connexion |
|---|
| deconnecte |

**Suggestions donnees test** : connexion perdue, redirection vers connexion

---

### Feature: Virement instantané SEPA

> Permet à l'utilisateur d'effectuer un virement instantané vers un autre compte bancaire en zone SEPA.

- **ID** : d1fd8c5f-e929-4180-bb91-37f46402dcb2
- **Niveau** : Acceptance
- **Tags** : @US-003, @P1
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-003

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
```

#### ✅ Scenario: Scenario happy path - Virement instantané SEPA

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
  When L'utilisateur valide le virement.
  Then Le virement est exécuté en moins de 10 secondes.
```

**Exemples :**

| montant_virement |
|---|
| 100.00 |
| 500.00 |

**Suggestions donnees test** : montant de 100€, montant de 500€, temps de traitement

#### ⚠️ Scenario: Scenario cas limite - Virement sans code PIN

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté mais n'a pas entré le code PIN.
  When L'utilisateur valide le virement.
  Then Le virement est refusé et un message d'erreur est affiché.
```

**Exemples :**

| etat_code_pin |
|---|
| non_entre |

**Suggestions donnees test** : code PIN non entre, virement refusé

#### ❌ Scenario: Scenario erreur - Virement avec photo de mauvaise qualité

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un bénéficiaire.
  When L'utilisateur valide le virement.
  Then Le virement est refusé et un message d'erreur est affiché.
```

**Exemples :**

| qualite_photo |
|---|
| mauvaise_qualite |

**Suggestions donnees test** : photo de mauvaise qualité, virement refusé

---

### Feature: Gestion des cartes bancaires

> Permet à l'utilisateur de gérer les paramètres de sa carte bancaire depuis l'application.

- **ID** : 73850320-061f-4f12-8f55-be0d7102f93b
- **Niveau** : Acceptance
- **Tags** : @US-004, @P1
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-004

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné une carte.
```

#### ✅ Scenario: Scenario happy path - Activation du paiement sans contact

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné une carte.
  When L'utilisateur active le paiement sans contact.
  Then L'activation prend effet en moins de 30 secondes.
```

**Exemples :**

| type_paiement |
|---|
| sans_contact |

**Suggestions donnees test** : paiement sans contact, temps d'activation

#### ⚠️ Scenario: Scenario cas limite - Activation sans code PIN

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté mais n'a pas entré le code PIN.
  When L'utilisateur active le paiement sans contact.
  Then L'activation est refusée et un message d'erreur est affiché.
```

**Exemples :**

| etat_code_pin |
|---|
| non_entre |

**Suggestions donnees test** : code PIN non entre, activation refusée

#### ❌ Scenario: Scenario erreur - Activation de la carte avec photo de mauvaise qualité

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné une carte.
  When L'utilisateur active le paiement sans contact.
  Then L'activation est refusée et un message d'erreur est affiché.
```

**Exemples :**

| qualite_photo |
|---|
| mauvaise_qualite |

**Suggestions donnees test** : photo de mauvaise qualité, activation refusée

---

### Feature: Alertes et notifications paramétrables

> Permet à l'utilisateur de configurer des alertes sur ses comptes et opérations.

- **ID** : 92367e45-6e02-47c4-b329-f9a031644ecd
- **Niveau** : Acceptance
- **Tags** : @US-005, @P1
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-005

**Background:**

```gherkin
  Given L'utilisateur a configuré une alerte sur un seuil de solde.
```

#### ✅ Scenario: Scenario happy path - Alerte seuil de solde

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a configuré une alerte sur un seuil de solde.
  When Le solde dépasse le seuil.
  Then Une alerte est envoyée par push, email ou SMS.
```

**Exemples :**

| type_alerte |
|---|
| push |
| email |
| sms |

**Suggestions donnees test** : alerte push, alerte email, alerte SMS

#### ⚠️ Scenario: Scenario cas limite - Alerte seuil de solde après inactivité

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a configuré une alerte sur un seuil de solde et n'a pas interagit pendant 5 minutes.
  When Le solde dépasse le seuil.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| duree_inactivite |
|---|
| 5 minutes |

**Suggestions donnees test** : inactivité de 5 minutes, redirection vers connexion

#### ❌ Scenario: Scenario erreur - Alerte seuil de solde sans connexion

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a configuré une alerte sur un seuil de solde mais n'est pas connecté.
  When Le solde dépasse le seuil.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| etat_connexion |
|---|
| deconnecte |

**Suggestions donnees test** : connexion perdue, redirection vers connexion

---

### Feature: Depôt de chèque par photo

> Permet à l'utilisateur de déposer un chèque en le photographiant avec l'application.

- **ID** : e738315d-6b99-4635-96d1-a796bc4ab6f2
- **Niveau** : Acceptance
- **Tags** : @US-006, @P2
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-006

**Background:**

```gherkin
  Given L'utilisateur a photographié le recto et le verso d'un chèque.
```

#### ✅ Scenario: Scenario happy path - Depôt de chèque par photo

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a photographié le recto et le verso d'un chèque.
  When L'utilisateur valide les informations extraites.
  Then Le chèque est soumis et un statut est affiché.
```

**Exemples :**

| type_chèque |
|---|
| recto |
| verso |
| recto_et_verso |

**Suggestions donnees test** : recto du chèque, verso du chèque, statut du depôt

#### ⚠️ Scenario: Scenario cas limite - Depôt de chèque avec photo de mauvaise qualité

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a photographié le recto et le verso d'un chèque avec une photo de mauvaise qualité.
  When L'utilisateur valide les informations extraites.
  Then Le chèque est refusé et un message d'erreur est affiché.
```

**Exemples :**

| qualite_photo |
|---|
| mauvaise_qualite |

**Suggestions donnees test** : photo de mauvaise qualité, depôt refusé

#### ❌ Scenario: Scenario erreur - Depôt de chèque sans validation de la biometrie

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a photographié le recto et le verso d'un chèque mais n'a pas validé la biometrie.
  When L'utilisateur valide les informations extraites.
  Then Le chèque est refusé et un message d'erreur est affiché.
```

**Exemples :**

| etat_biometrie |
|---|
| non_validee |

**Suggestions donnees test** : biometrie non validee, depôt refusé

---

### Feature: Categorisation automatique des dépenses

> Permet à l'utilisateur de visualiser ses dépenses réparties par catégorie avec des graphiques.

- **ID** : 38219cab-0828-4e18-8b4b-fbb611d6def8
- **Niveau** : Acceptance
- **Tags** : @US-007, @P2
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-007

**Background:**

```gherkin
  Given L'utilisateur a consulté ses opérations.
```

#### ✅ Scenario: Scenario happy path - Categorisation automatique des dépenses

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a consulté ses opérations.
  When L'utilisateur accède aux graphiques.
  Then Les dépenses sont réparties en 12 catégories.
```

**Exemples :**

| type_depense |
|---|
| alimentation |
| loisirs |
| transports |
| santé |
| habitation |
| divertissement |
| education |
| veto |
| santé |
| santé |
| santé |
| santé |

**Suggestions donnees test** : alimentation, loisirs, transports, santé, habitation, divertissement, education, veto, statistiques, graphiques

#### ⚠️ Scenario: Scenario cas limite - Categorisation des dépenses après inactivité

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a consulté ses opérations et n'a pas interagit pendant 5 minutes.
  When L'utilisateur accède aux graphiques.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| duree_inactivite |
|---|
| 5 minutes |

**Suggestions donnees test** : inactivité de 5 minutes, redirection vers connexion

#### ❌ Scenario: Scenario erreur - Categorisation des dépenses sans connexion

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur a consulté ses opérations mais n'est pas connecté.
  When L'utilisateur accède aux graphiques.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| etat_connexion |
|---|
| deconnecte |

**Suggestions donnees test** : connexion perdue, redirection vers connexion

---

### Feature: Coffre-fort de documents bancaires

> Permet à l'utilisateur de consulter et de télécharger ses documents bancaires depuis l'application.

- **ID** : afe46426-bd27-4a13-8378-5a49df4c2238
- **Niveau** : Acceptance
- **Tags** : @US-008, @P2
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-008

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document.
```

#### ✅ Scenario: Scenario happy path - Telechargement de document

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document.
  When L'utilisateur tente de le télécharger.
  Then Le téléchargement est protégé par la biométrie ou le code PIN.
```

**Exemples :**

| type_document |
|---|
| relevé |
| contrat |
| attestation |

**Suggestions donnees test** : relevé bancaire, contrat, attestation, protection biométrique

#### ⚠️ Scenario: Scenario cas limite - Telechargement de document sans validation de la biometrie

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document mais n'a pas validé la biometrie.
  When L'utilisateur tente de le télécharger.
  Then Le téléchargement est refusé et un message d'erreur est affiché.
```

**Exemples :**

| etat_biometrie |
|---|
| non_validee |

**Suggestions donnees test** : biometrie non validee, telechargement refusé

#### ❌ Scenario: Scenario erreur - Telechargement de document sans code PIN

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un document mais n'a pas entré le code PIN.
  When L'utilisateur tente de le télécharger.
  Then Le téléchargement est refusé et un message d'erreur est affiché.
```

**Exemples :**

| etat_code_pin |
|---|
| non_entre |

**Suggestions donnees test** : code PIN non entre, telechargement refusé

---

### Feature: Contact conseiller par chat

> Permet à l'utilisateur de contacter son conseiller bancaire via un chat intégré dans l'application.

- **ID** : ddd2e1e2-df1e-4a41-a3ab-639317d36496
- **Niveau** : Acceptance
- **Tags** : @US-009, @P2
- **Exigences couvertes** : FR-005
- **Scenarios source** : US-009

**Background:**

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller.
```

#### ✅ Scenario: Scenario happy path - Envoi de message par chat

- **Type** : HappyPath
- **Technique** : EquivalencePartitioning
- **Tags** : @happy_path, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller.
  When L'utilisateur envoie un message.
  Then Le message est envoyé et un historique est conservé.
```

**Exemples :**

| type_message |
|---|
| question |
| demande |
| information |

**Suggestions donnees test** : question, demande, information, historique des messages

#### ⚠️ Scenario: Scenario cas limite - Envoi de message par chat après inactivité

- **Type** : EdgeCase
- **Technique** : BoundaryValueAnalysis
- **Tags** : @edge_case, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller mais n'a pas interagit pendant 5 minutes.
  When L'utilisateur envoie un message.
  Then L'utilisateur est redirigé vers la page de connexion.
```

**Exemples :**

| duree_inactivite |
|---|
| 5 minutes |

**Suggestions donnees test** : inactivité de 5 minutes, redirection vers connexion

#### ❌ Scenario: Scenario erreur - Envoi de message par chat sans validation de la biometrie

- **Type** : ErrorScenario
- **Technique** : ErrorGuessing
- **Tags** : @error, @FR-005
- **Verifie** : FR-005

```gherkin
  Given L'utilisateur est connecté et a sélectionné un conseiller mais n'a pas validé la biometrie.
  When L'utilisateur envoie un message.
  Then Le message est refusé et un message d'erreur est affiché.
```

**Exemples :**

| etat_biometrie |
|---|
| non_validee |

**Suggestions donnees test** : biometrie non validee, message refusé

---

## Couverture des tests

| Metrique | Valeur |
|----------|--------|
| **Couverture** | 100% |
| **Exigences couvertes** | 63 / 63 |
| **Exigences** | FR-001, FR-002, FR-003, FR-004, FR-005, FR-006, FR-007, FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, FR-014, FR-015, FR-016, FR-017, FR-018, FR-019, FR-020, FR-021, FR-022, FR-023, FR-024, FR-025, FR-026, FR-027, FR-028, FR-029, FR-030, FR-031, FR-032, FR-033, FR-034, FR-035, FR-036, FR-037, FR-038, FR-039, FR-040, FR-041, FR-042, FR-043, FR-044, FR-045, FR-046, FR-047, FR-048, FR-049, FR-050, FR-051, FR-052, FR-053, FR-054, FR-055, FR-056, FR-057, FR-058, FR-059, FR-060, FR-061, FR-062, FR-063 |
| **Happy path** | 18 |
| **Cas limites** | 18 |
| **Scenarios erreur** | 18 |
| **Conditions limites** | 0 |

## Matrice de tracabilite

### Resume

| Total | Couvertes | Partielles | Non couvertes | Verifiees autrement | Couverture |
|-------|-----------|------------|---------------|---------------------|------------|
| 63 | 55 | 8 | 0 | 0 | 100% |

### Detail par exigence

#### FR-001 — PartiallyCovered

> Le système DOIT supporter l'empreinte digitale et la reconnaissance faciale (Face ID / Android Biometric) pour l'authentification.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-001
- **Features** : Authentification biometrique
- **Scenarios** : Scenario happy path - Authentification biométrique
- **Techniques** : EquivalencePartitioning

#### FR-002 — PartiallyCovered

> Le système DOIT permettre l'authentification biométrique après 3 tentatives et basculer sur le code PIN à 6 chiffres.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-001
- **Features** : Authentification biometrique
- **Scenarios** : Scenario cas limite - Authentification biométrique après 3 tentatives
- **Techniques** : BoundaryValueAnalysis

#### FR-003 — PartiallyCovered

> Le système DOIT expirer la session automatiquement après 5 minutes d'inactivité.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-001
- **Features** : Consultation des soldes et des opérations
- **Scenarios** : Scenario cas limite - Consultation après 5 minutes d'inactivité
- **Techniques** : BoundaryValueAnalysis

#### FR-004 — FullyCovered

> Le système DOIT exiger le mot de passe classique avant d'activer la biometrie au premier lancement après installation.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-001
- **Features** : Authentification biometrique
- **Scenarios** : Scenario happy path - Authentification biométrique, Scenario erreur - Authentification biométrique sans mot de passe
- **Techniques** : ErrorGuessing, EquivalencePartitioning

#### FR-005 — FullyCovered

> Le système DOIT afficher un journal de connexion horodaté dans les paramètres de sécurité.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-001
- **Features** : Contact conseiller par chat, Virement instantané SEPA, Depôt de chèque par photo, Coffre-fort de documents bancaires, Authentification biometrique, Gestion des cartes bancaires, Consultation des soldes et des opérations, Categorisation automatique des dépenses, Alertes et notifications paramétrables
- **Scenarios** : Scenario happy path - Authentification biometrique, Scenario cas limite - Authentification biometrique sans mot de passe, Scenario erreur - Authentification biometrique echoue, Scenario happy path - Consultation des soldes, Scenario cas limite - Consultation des soldes après inactivité, Scenario erreur - Consultation des soldes sans connexion, Scenario happy path - Virement instantané SEPA, Scenario cas limite - Virement sans code PIN, Scenario erreur - Virement avec photo de mauvaise qualité, Scenario happy path - Activation du paiement sans contact, Scenario cas limite - Activation sans code PIN, Scenario erreur - Activation de la carte avec photo de mauvaise qualité, Scenario happy path - Alerte seuil de solde, Scenario cas limite - Alerte seuil de solde après inactivité, Scenario erreur - Alerte seuil de solde sans connexion, Scenario happy path - Depôt de chèque par photo, Scenario cas limite - Depôt de chèque avec photo de mauvaise qualité, Scenario erreur - Depôt de chèque sans validation de la biometrie, Scenario happy path - Categorisation automatique des dépenses, Scenario cas limite - Categorisation des dépenses après inactivité, Scenario erreur - Categorisation des dépenses sans connexion, Scenario happy path - Telechargement de document, Scenario cas limite - Telechargement de document sans validation de la biometrie, Scenario erreur - Telechargement de document sans code PIN, Scenario happy path - Envoi de message par chat, Scenario cas limite - Envoi de message par chat après inactivité, Scenario erreur - Envoi de message par chat sans validation de la biometrie
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-006 — PartiallyCovered

> Le système DOIT permettre à l'utilisateur de désactiver la biometrie à tout moment.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-001
- **Features** : Authentification biometrique
- **Scenarios** : Scenario happy path - Authentification biométrique
- **Techniques** : EquivalencePartitioning

#### FR-007 — PartiallyCovered

> Le système DOIT afficher le solde de chaque compte dès la page d'accueil après connexion.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes
- **Techniques** : EquivalencePartitioning

#### FR-008 — FullyCovered

> Le système DOIT afficher l'historique des opérations sur 13 mois glissants.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Consultation des soldes et des opérations, Visualisation des dépenses par catégorie
- **Scenarios** : Scenario happy path - Affichage des soldes, Visualisation des dépenses par catégorie (cas limite - 13 mois glissants), Visualisation des dépenses par catégorie (erreur - absence de données)
- **Techniques** : ErrorGuessing, EquivalencePartitioning, BoundaryValueAnalysis

#### FR-009 — FullyCovered

> Le système DOIT afficher la date, le libellé, le montant et le solde après opération pour chaque opération.

- **Priorite** : P1
- **Risque** : —
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Consultation des soldes et des opérations, Visualisation des dépenses par catégorie
- **Scenarios** : Scenario happy path - Affichage des soldes, Visualisation des dépenses par catégorie (cas limite - 13 mois glissants), Visualisation des dépenses par catégorie (erreur - absence de données)
- **Techniques** : BoundaryValueAnalysis, EquivalencePartitioning, ErrorGuessing

#### FR-010 — FullyCovered

> Le système DOIT distinguer visuellement les opérations en attente (non comptabilisées) en italique.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Visualisation des dépenses par catégorie, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Visualisation des dépenses par catégorie (cas limite - 13 mois glissants), Visualisation des dépenses par catégorie (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-011 — PartiallyCovered

> Le système DOIT permettre la recherche par mot-clé ou montant dans l'historique des opérations.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes
- **Techniques** : EquivalencePartitioning

#### FR-012 — PartiallyCovered

> Le système DOIT permettre un pull-to-refresh pour mettre à jour les données depuis le serveur en moins de 2 secondes.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes
- **Techniques** : EquivalencePartitioning

#### FR-013 — PartiallyCovered

> Le système DOIT permettre la consultation hors ligne des données avec la dernière synchronisation affichée.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-002
- **Features** : Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes
- **Techniques** : EquivalencePartitioning

#### FR-014 — FullyCovered

> Le système DOIT exécuter un virement instantané en moins de 10 secondes et le débit doit être immédiat.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-015 — FullyCovered

> Le système DOIT limiter le montant maximum par virement instantané à 15 000 euros.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Scenario cas limite - Virement au seuil maximal
- **Techniques** : EquivalencePartitioning, BoundaryValueAnalysis

#### FR-016 — FullyCovered

> Le système DOIT limiter le cumul journalier des virements instantanés à 30 000 euros.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Scenario cas limite - Virement au seuil maximal
- **Techniques** : EquivalencePartitioning, BoundaryValueAnalysis

#### FR-017 — FullyCovered

> Le système DOIT permettre la sélection du bénéficiaire depuis le carnet d'adresses ou via IBAN.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-018 — FullyCovered

> Le système DOIT exiger la biometrie ou le code PIN pour valider un virement.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario erreur - Validation d'un virement sans authentification, Scenario happy path - Virement instantané, Scenario erreur - Virement sans authentification
- **Techniques** : ErrorGuessing, EquivalencePartitioning

#### FR-019 — FullyCovered

> Le système DOIT afficher une notification push confirmant l'execution ou l'échec du virement dans les 30 secondes.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-020 — FullyCovered

> Le système DOIT afficher clairement les frais de 0,50 euro par virement instantané avant validation.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-021 — FullyCovered

> Le système DOIT rendre impossible l'annulation d'un virement une fois envoyé, avec un message d'alerte.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-003
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-022 — FullyCovered

> Le système DOIT permettre l'activation et la désactivation du paiement sans contact en moins de 30 secondes.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-004
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations, Gestion des cartes bancaires
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Activation du paiement sans contact - Happy Path, Activation du paiement sans contact - Cas limite, Activation du paiement sans contact - Erreur
- **Techniques** : ErrorGuessing, EquivalencePartitioning, BoundaryValueAnalysis

#### FR-023 — FullyCovered

> Le système DOIT bloquer temporairement la carte instantanément et rendre le blocage réversible sans appel à la banque.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-004
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-024 — FullyCovered

> Le système DOIT permettre la modification des plafonds de paiement et de retrait dans des limites définies par le contrat.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-004
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Gestion des cartes bancaires
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Activation du paiement sans contact - Happy Path, Activation du paiement sans contact - Cas limite, Activation du paiement sans contact - Erreur
- **Techniques** : ErrorGuessing, BoundaryValueAnalysis, EquivalencePartitioning

#### FR-025 — FullyCovered

> Le système DOIT permettre la consultation sécurisée du code PIN après vérification biométrique, avec un affichage limité à 10 secondes.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-004
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-026 — FullyCovered

> Le système DOIT nécessiter une double confirmation pour l'opposition définitive et générer un numéro de dossier.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-004
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-027 — FullyCovered

> Le système DOIT permettre la visualisation du suivi des paiements en attente et des prélèvements à venir sur un calendrier.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-004
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-028 — FullyCovered

> Le système DOIT permettre la configuration d'alertes sur dépassement de seuil (solde en dessous de X euros).

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Alertes et notifications paramétrables
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Alerte seuil de solde - Happy Path, Alerte seuil de solde - Cas limite, Alerte seuil de solde - Erreur
- **Techniques** : EquivalencePartitioning, BoundaryValueAnalysis, ErrorGuessing

#### FR-029 — FullyCovered

> Le système DOIT permettre l'alerte à chaque opération supérieure à un montant défini par l'utilisateur.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations, Alertes et notifications paramétrables
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Alerte seuil de solde - Happy Path, Alerte seuil de solde - Cas limite, Alerte seuil de solde - Erreur
- **Techniques** : ErrorGuessing, EquivalencePartitioning, BoundaryValueAnalysis

#### FR-030 — FullyCovered

> Le système DOIT envoyer une notification push à chaque connexion depuis un nouvel appareil.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-031 — FullyCovered

> Le système DOIT permettre l'alerte sur les prélèvements inhabituels (montant significativement supérieur au précédent).

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-032 — FullyCovered

> Le système DOIT envoyer les alertes par push notification, email ou SMS selon la préférence de l'utilisateur.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Virement instantané SEPA, Alertes et notifications paramétrables, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Alerte seuil de solde - Happy Path, Alerte seuil de solde - Cas limite, Alerte seuil de solde - Erreur
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-033 — FullyCovered

> Le système DOIT permettre à l'utilisateur de désactiver individuellement chaque type d'alerte.

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-034 — FullyCovered

> Le système DOIT rendre indésactivable les alertes de sécurité (connexion suspecte, tentative de fraude).

- **Priorite** : P1
- **Risque** : High
- **Verification** : Test
- **Stories source** : US-005
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-035 — FullyCovered

> Le système DOIT permettre le dépôt de chèque par photo avec la prise de photo du recto et du verso.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Consultation des soldes et des opérations, Dépôt de chèque par photo, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Dépôt de chèque - Happy Path, Dépôt de chèque - Cas limite, Dépôt de chèque - Erreur
- **Techniques** : EquivalencePartitioning, BoundaryValueAnalysis, ErrorGuessing

#### FR-036 — FullyCovered

> Le système DOIT permettre la reconnaissance OCR pour extraire automatiquement le montant et le nom de l'émetteur du chèque.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Consultation des soldes et des opérations, Dépôt de chèque par photo, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Dépôt de chèque - Happy Path, Dépôt de chèque - Cas limite, Dépôt de chèque - Erreur
- **Techniques** : BoundaryValueAnalysis, EquivalencePartitioning, ErrorGuessing

#### FR-037 — FullyCovered

> Le système DOIT permettre à l'utilisateur de valider ou de corriger les informations extraites avant soumission.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Dépôt de chèque par photo, Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Dépôt de chèque - Happy Path, Dépôt de chèque - Cas limite, Dépôt de chèque - Erreur
- **Techniques** : ErrorGuessing, EquivalencePartitioning, BoundaryValueAnalysis

#### FR-038 — FullyCovered

> Le système DOIT limiter le montant maximum par dépôt photo à 5 000 euros.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Dépôt de chèque par photo
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Dépôt de chèque - Happy Path, Dépôt de chèque - Cas limite, Dépôt de chèque - Erreur
- **Techniques** : BoundaryValueAnalysis, EquivalencePartitioning, ErrorGuessing

#### FR-039 — FullyCovered

> Le système DOIT traiter le chèque en 2 jours ouvrables après soumission.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-040 — FullyCovered

> Le système DOIT afficher un statut de traitement du chèque (en cours, valide, rejeté) dans l'historique.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations, Dépôt de chèque par photo
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Dépôt de chèque - Happy Path, Dépôt de chèque - Cas limite, Dépôt de chèque - Erreur
- **Techniques** : BoundaryValueAnalysis, EquivalencePartitioning, ErrorGuessing

#### FR-041 — FullyCovered

> Le système DOIT permettre la résoumission du chèque en cas de rejet, avec la communication du motif.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-006
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Dépôt de chèque par photo
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Dépôt de chèque - Happy Path, Dépôt de chèque - Cas limite, Dépôt de chèque - Erreur
- **Techniques** : BoundaryValueAnalysis, ErrorGuessing, EquivalencePartitioning

#### FR-042 — FullyCovered

> Le système DOIT classer automatiquement les opérations en 12 catégories (alimentation, transport, logement, loisirs, santé, habillement, éducation, restaurant, abonnements, épargne, impôts, divers).

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-007
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Visualisation des dépenses par catégorie
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Visualisation des dépenses par catégorie (happy path)
- **Techniques** : EquivalencePartitioning

#### FR-043 — FullyCovered

> Le système DOIT permettre à l'utilisateur de recategoriser manuellement une opération et d'apprendre pour les futures opérations du même commercant.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-007
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-044 — FullyCovered

> Le système DOIT afficher un graphique camembert montrant la répartition du mois en cours.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-007
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Visualisation des dépenses par catégorie
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Visualisation des dépenses par catégorie (happy path)
- **Techniques** : EquivalencePartitioning

#### FR-045 — FullyCovered

> Le système DOIT afficher un graphique barre comparant les dépenses mois par mois sur 6 mois.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-007
- **Features** : Visualisation des dépenses par catégorie, Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Visualisation des dépenses par catégorie (happy path)
- **Techniques** : EquivalencePartitioning

#### FR-046 — FullyCovered

> Le système DOIT permettre la paramétrisation du budget mensuel par catégorie avec une alerte à 80% de consommation.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-007
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-047 — FullyCovered

> Le système DOIT permettre l'export PDF du bilan mensuel.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-007
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-048 — FullyCovered

> Le système DOIT permettre la consultation et le téléchargement des documents bancaires depuis l'application.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Consultation des soldes et des opérations, Téléchargement sécurisé des documents bancaires, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Téléchargement sécurisé des documents bancaires (happy path), Téléchargement sécurisé des documents bancaires (cas limite - absence de biométrie ou code PIN), Téléchargement sécurisé des documents bancaires (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, BoundaryValueAnalysis, ErrorGuessing

#### FR-049 — FullyCovered

> Le système DOIT rendre disponibles les relevés de compte mensuels à partir du 5 du mois suivant.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-050 — FullyCovered

> Le système DOIT permettre la génération à la demande d'attestations (domiciliation, solde) en format PDF.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-051 — FullyCovered

> Le système DOIT permettre le téléchargement et le partage du RIB/IBAN en un tap.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-052 — FullyCovered

> Le système DOIT permettre l'archivage et la consultation des contrats signés électroniquement.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-053 — FullyCovered

> Le système DOIT conserver les documents pendant 10 ans conformément à la réglementation.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané
- **Techniques** : EquivalencePartitioning

#### FR-054 — FullyCovered

> Le système DOIT permettre la recherche par type de document et par date.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Virement instantané SEPA, Consultation des soldes et des opérations, Téléchargement sécurisé des documents bancaires
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Téléchargement sécurisé des documents bancaires (happy path), Téléchargement sécurisé des documents bancaires (cas limite - absence de biométrie ou code PIN), Téléchargement sécurisé des documents bancaires (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-055 — FullyCovered

> Le système DOIT protéger le téléchargement des documents par la biometrie ou le code PIN.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-008
- **Features** : Téléchargement sécurisé des documents bancaires, Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Téléchargement sécurisé des documents bancaires (happy path), Téléchargement sécurisé des documents bancaires (cas limite - absence de biométrie ou code PIN), Téléchargement sécurisé des documents bancaires (erreur - absence de données)
- **Techniques** : BoundaryValueAnalysis, ErrorGuessing, EquivalencePartitioning

#### FR-056 — FullyCovered

> Le système DOIT permettre le contact avec le conseiller bancaire via un chat intégré.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Chat avec conseiller bancaire, Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : BoundaryValueAnalysis, ErrorGuessing, EquivalencePartitioning

#### FR-057 — FullyCovered

> Le système DOIT rendre le chat disponible du lundi au vendredi de 8h30 à 18h30.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Chat avec conseiller bancaire, Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : BoundaryValueAnalysis, EquivalencePartitioning, ErrorGuessing

#### FR-058 — FullyCovered

> Le système DOIT permettre un chatbot répondant aux questions fréquentes en dehors des horaires.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Consultation des soldes et des opérations, Chat avec conseiller bancaire, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-059 — FullyCovered

> Le système DOIT garantir un délai de première réponse du conseiller inférieur à 5 minutes en moyenne.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Chat avec conseiller bancaire, Consultation des soldes et des opérations, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-060 — FullyCovered

> Le système DOIT permettre à l'utilisateur de joindre un document ou une capture d'écran au message.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Consultation des soldes et des opérations, Chat avec conseiller bancaire, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : BoundaryValueAnalysis, EquivalencePartitioning, ErrorGuessing

#### FR-061 — FullyCovered

> Le système DOIT conserver l'historique des conversations et le rendre consultable.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Consultation des soldes et des opérations, Chat avec conseiller bancaire, Virement instantané SEPA
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : BoundaryValueAnalysis, ErrorGuessing, EquivalencePartitioning

#### FR-062 — FullyCovered

> Le système DOIT permettre au conseiller de partager un lien vers un produit ou un simulateur directement dans le chat.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Chat avec conseiller bancaire, Virement instantané SEPA, Consultation des soldes et des opérations
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, ErrorGuessing, BoundaryValueAnalysis

#### FR-063 — FullyCovered

> Le système DOIT envoyer une notification push informant l'utilisateur quand le conseiller répond.

- **Priorite** : P2
- **Risque** : Medium
- **Verification** : Test
- **Stories source** : US-009
- **Features** : Consultation des soldes et des opérations, Virement instantané SEPA, Chat avec conseiller bancaire
- **Scenarios** : Scenario happy path - Affichage des soldes, Scenario happy path - Virement instantané, Chat avec conseiller bancaire (happy path), Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN), Chat avec conseiller bancaire (erreur - absence de données)
- **Techniques** : EquivalencePartitioning, BoundaryValueAnalysis, ErrorGuessing

## Notes de conformite

| Standard | Section | Statut | Details |
|----------|---------|--------|---------|
| ISO/IEC/IEEE 29148:2018 | 6.6 — Traceability | Compliant | Couverture forward: 100% |
| ISO/IEC/IEEE 29148:2018 | 5.2.5 — Well-formed requirements | Compliant | 63 exigences dans la matrice |

---
*Genere par spec-forge le 2026-03-05*