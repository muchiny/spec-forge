# language: fr

@US-009 @P2
Fonctionnalité: Chat avec conseiller bancaire
  Fonctionnalité permettant à l'utilisateur de contacter son conseiller bancaire via un chat intégré dans l'application pour obtenir des réponses rapides sans devoir se déplacer en agence ou attendre au téléphone.

  Contexte:
    Soit L'utilisateur est connecté et a sélectionné un conseiller.

  @happy_path @FR-056 @FR-057 @FR-058 @FR-059 @FR-060 @FR-061 @FR-062 @FR-063
  Plan du Scénario: Chat avec conseiller bancaire (happy path)
    Soit L'utilisateur est connecté et a sélectionné un conseiller.
    Quand L'utilisateur envoie un message.
    Alors Le message est envoyé et un historique est conservé.

    Exemples:
      | message | format |
      | Bonjour, pouvez-vous m'aider ? | Texte |
      | Voici une capture d'écran de mon problème | Image |
      | Voici le document concerné | PDF |
      | Voici le relevé de compte | PDF |
      | Voici le contrat signé | PDF |
      | Voici le RIB | PDF |

  @edge_case @FR-056 @FR-057 @FR-058 @FR-059 @FR-060 @FR-061 @FR-062 @FR-063
  Plan du Scénario: Chat avec conseiller bancaire (cas limite - absence de biométrie ou code PIN)
    Soit L'utilisateur est connecté et a sélectionné un conseiller.
    Quand L'utilisateur tente de joindre un document au message sans avoir validé la biométrie ou le code PIN.
    Alors Le système bloque l'envoi du message.
    Et Le système affiche un message d'erreur indiquant la nécessité de la biométrie ou du code PIN.

    Exemples:
      | message | format |
      | Bonjour, pouvez-vous m'aider ? | Texte |
      | Voici une capture d'écran de mon problème | Image |
      | Voici le document concerné | PDF |
      | Voici le relevé de compte | PDF |
      | Voici le contrat signé | PDF |
      | Voici le RIB | PDF |

  @error @FR-056 @FR-057 @FR-058 @FR-059 @FR-060 @FR-061 @FR-062 @FR-063
  Plan du Scénario: Chat avec conseiller bancaire (erreur - absence de données)
    Soit L'utilisateur est connecté et a sélectionné un conseiller.
    Quand L'utilisateur tente de joindre un document au message.
    Alors Le système affiche un message d'erreur indiquant l'absence de document.
    Et Le message n'est pas envoyé.

    Exemples:
      | message | format |
      | Bonjour, pouvez-vous m'aider ? | Texte |
      | Voici une capture d'écran de mon problème | Image |
      | Voici le document concerné | PDF |
      | Voici le relevé de compte | PDF |
      | Voici le contrat signé | PDF |
      | Voici le RIB | PDF |

