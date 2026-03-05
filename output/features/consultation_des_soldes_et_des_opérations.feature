# language: fr

@US-002 @P1
Fonctionnalité: Consultation des soldes et des opérations
  Permet à l'utilisateur de consulter les soldes et l'historique de ses comptes.

  Contexte:
    Soit L'utilisateur est connecté à l'application.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Consultation des soldes
    Soit L'utilisateur est connecté à l'application.
    Quand L'utilisateur accède à la page d'accueil.
    Alors Les soldes des comptes sont affichés.

    Exemples:
      | type_compte |
      | compte_courant |
      | compte_epargne |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Consultation des soldes après inactivité
    Soit L'utilisateur est connecté à l'application et n'a pas interagit pendant 5 minutes.
    Quand L'utilisateur accède à la page d'accueil.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | duree_inactivite |
      | 5 minutes |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Consultation des soldes sans connexion
    Soit L'utilisateur n'est pas connecté à l'application.
    Quand L'utilisateur accède à la page d'accueil.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | etat_connexion |
      | deconnecte |

