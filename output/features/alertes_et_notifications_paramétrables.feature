# language: fr

@US-005 @P1
Fonctionnalité: Alertes et notifications paramétrables
  Permet à l'utilisateur de configurer des alertes sur ses comptes et opérations.

  Contexte:
    Soit L'utilisateur a configuré une alerte sur un seuil de solde.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Alerte seuil de solde
    Soit L'utilisateur a configuré une alerte sur un seuil de solde.
    Quand Le solde dépasse le seuil.
    Alors Une alerte est envoyée par push, email ou SMS.

    Exemples:
      | type_alerte |
      | push |
      | email |
      | sms |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Alerte seuil de solde après inactivité
    Soit L'utilisateur a configuré une alerte sur un seuil de solde et n'a pas interagit pendant 5 minutes.
    Quand Le solde dépasse le seuil.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | duree_inactivite |
      | 5 minutes |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Alerte seuil de solde sans connexion
    Soit L'utilisateur a configuré une alerte sur un seuil de solde mais n'est pas connecté.
    Quand Le solde dépasse le seuil.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | etat_connexion |
      | deconnecte |

