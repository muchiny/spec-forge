# language: fr

@US-007 @P2
Fonctionnalité: Visualisation des dépenses par catégorie
  Fonctionnalité permettant de visualiser les dépenses réparties par catégorie avec des graphiques pour comprendre où va son argent et mieux gérer son budget mensuel.

  Contexte:
    Soit L'utilisateur est connecté et a consulté ses opérations.

  @happy_path @FR-042 @FR-044 @FR-045
  Plan du Scénario: Visualisation des dépenses par catégorie (happy path)
    Soit L'utilisateur est connecté et a consulté ses opérations.
    Quand L'utilisateur accède aux graphiques.
    Alors Les dépenses sont réparties en 12 catégories.

    Exemples:
      | categorie |
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

  @edge_case @FR-008 @FR-009 @FR-010
  Plan du Scénario: Visualisation des dépenses par catégorie (cas limite - 13 mois glissants)
    Soit L'utilisateur est connecté et a consulté ses opérations.
    Quand L'utilisateur accède aux graphiques.
    Alors Les dépenses sont réparties en 12 catégories.
    Et Le système affiche l'historique des opérations sur 13 mois glissants.
    Alors Chaque opération affiche la date, le libellé, le montant et le solde après opération.
    Et Les opérations en attente sont visuellement distinguées en italique.

    Exemples:
      | categorie | mois |
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

  @error @FR-008 @FR-009 @FR-010
  Plan du Scénario: Visualisation des dépenses par catégorie (erreur - absence de données)
    Soit L'utilisateur est connecté et a consulté ses opérations.
    Quand L'utilisateur accède aux graphiques.
    Alors Le système affiche un message d'erreur indiquant l'absence de données.
    Et Le système ne répartit pas les dépenses en catégories.

    Exemples:
      | categorie | mois |
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

