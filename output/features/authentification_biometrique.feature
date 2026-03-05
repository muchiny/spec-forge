# language: fr

@US-001 @P1
Fonctionnalité: Authentification biometrique
  Permet à l'utilisateur de se connecter via la reconnaissance faciale ou l'empreinte digitale.

  Contexte:
    Soit L'utilisateur a installé l'application et a un appareil compatible avec la reconnaissance faciale ou l'empreinte digitale.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Authentification biometrique
    Soit L'utilisateur est connecté à l'application.
    Quand L'utilisateur tente de se connecter.
    Alors L'authentification biométrique est proposée et fonctionne correctement.

    Exemples:
      | type_authentification |
      | reconnaissance_faciale |
      | empreinte_digitale |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Authentification biometrique sans mot de passe
    Soit L'utilisateur a installé l'application mais n'a pas configuré le mot de passe classique.
    Quand L'utilisateur tente d'activer la biometrie.
    Alors L'activation de la biometrie est refusée.

    Exemples:
      | etat_mot_de_passe |
      | non_configure |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Authentification biometrique echoue
    Soit L'utilisateur a installé l'application mais la biometrie est défaillante.
    Quand L'utilisateur tente de se connecter.
    Alors L'authentification biométrique echoue et un message d'erreur est affiché.

    Exemples:
      | etat_biometrie |
      | defaillante |

