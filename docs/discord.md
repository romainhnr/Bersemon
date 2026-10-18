# Bot Discord - Bersemon

Mise en route du bot Discord. À l'étape 10, le bot se connecte et répond à une seule commande, `/bersemon`, qui confirme qu'il est en ligne ; le jeu complet arrive aux étapes suivantes (docs/gamedesign.md, section 16).

## 1. Créer l'application Discord

À faire une seule fois, avec ton compte Discord, sur le portail développeur : https://discord.com/developers/applications. Les intitulés ci-dessous sont ceux du portail en anglais ; ils peuvent changer.

1. New Application : nommer l'application Bersemon.
2. Onglet Bot : Reset Token pour obtenir le jeton du bot. Le jeton donne le contrôle du bot : il ne se partage pas et ne s'écrit dans aucun fichier du dépôt. Les Privileged Gateway Intents restent désactivés : les commandes slash n'en ont pas besoin.
3. Onglet OAuth2, URL Generator : cocher les portées `bot` et `applications.commands`, sans aucune permission, puis ouvrir l'adresse générée pour inviter le bot sur un serveur.

Pour jouer en message privé avec le bot, il faut partager au moins un serveur avec lui.

## 2. Lancer le bot

Le bot lit sa configuration dans deux variables d'environnement :

| Variable | Rôle |
|---|---|
| `DISCORD_TOKEN` | Jeton du bot, obligatoire. |
| `BERSEMON_SERVEUR_DE_TEST` | Facultative : identifiant d'un serveur où les commandes sont enregistrées tout de suite, pour développer. Sans elle, les commandes sont enregistrées pour tous les serveurs et les messages privés, ce qui peut prendre jusqu'à une heure. |

L'identifiant d'un serveur se copie dans Discord après avoir activé le mode développeur (Paramètres utilisateur, Avancés), par un clic droit sur le serveur.

Dans un terminal PowerShell, à la racine du dépôt :

```powershell
$env:DISCORD_TOKEN = "jeton du bot"
$env:BERSEMON_SERVEUR_DE_TEST = "identifiant du serveur"
cargo run -p bersemon-discord
```

Ces variables ne valent que pour ce terminal. Le bot affiche "Bersemon est connecté à Discord." une fois prêt ; `/bersemon` répond alors par un message visible du seul joueur, dans un salon comme en message privé. Ctrl+C arrête le bot.

## 3. Si le jeton a fuité

Le régénérer aussitôt dans l'onglet Bot (Reset Token) : l'ancien cesse de fonctionner.
