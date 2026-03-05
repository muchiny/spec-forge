# language: fr

@US-007 @P2
Fonctionnalité: Categorisation automatique des dépenses
  Permet à l'utilisateur de visualiser ses dépenses réparties par catégorie avec des graphiques.

  Contexte:
    Soit L'utilisateur a consulté ses opérations.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Categorisation automatique des dépenses
    Soit L'utilisateur a consulté ses opérations.
    Quand L'utilisateur accède aux graphiques.
    Alors Les dépenses sont réparties en 12 catégories.

    Exemples:
      | type_depense |
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

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Categorisation des dépenses après inactivité
    Soit L'utilisateur a consulté ses opérations et n'a pas interagit pendant 5 minutes.
    Quand L'utilisateur accède aux graphiques.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | duree_inactivite |
      | 5 minutes |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Categorisation des dépenses sans connexion
    Soit L'utilisateur a consulté ses opérations mais n'est pas connecté.
    Quand L'utilisateur accède aux graphiques.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | etat_connexion |
      | deconnecte |

