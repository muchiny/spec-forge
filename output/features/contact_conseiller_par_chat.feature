# language: fr

@US-009 @P2
Fonctionnalité: Contact conseiller par chat
  Permet à l'utilisateur de contacter son conseiller bancaire via un chat intégré dans l'application.

  Contexte:
    Soit L'utilisateur est connecté et a sélectionné un conseiller.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Envoi de message par chat
    Soit L'utilisateur est connecté et a sélectionné un conseiller.
    Quand L'utilisateur envoie un message.
    Alors Le message est envoyé et un historique est conservé.

    Exemples:
      | type_message |
      | question |
      | demande |
      | information |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Envoi de message par chat après inactivité
    Soit L'utilisateur est connecté et a sélectionné un conseiller mais n'a pas interagit pendant 5 minutes.
    Quand L'utilisateur envoie un message.
    Alors L'utilisateur est redirigé vers la page de connexion.

    Exemples:
      | duree_inactivite |
      | 5 minutes |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Envoi de message par chat sans validation de la biometrie
    Soit L'utilisateur est connecté et a sélectionné un conseiller mais n'a pas validé la biometrie.
    Quand L'utilisateur envoie un message.
    Alors Le message est refusé et un message d'erreur est affiché.

    Exemples:
      | etat_biometrie |
      | non_validee |

