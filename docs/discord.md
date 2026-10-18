# Bot Discord - Bersemon

Mise en route du bot Discord. Depuis l'étape 12, toute la partie se joue sur Discord : création du héros, camp, expéditions, combats, gestion des monstres et codex, avec les mêmes règles et les mêmes textes que la console.

## 1. Créer l'application Discord

À faire une seule fois, avec ton compte Discord, sur le portail développeur : https://discord.com/developers/applications. Les intitulés ci-dessous sont ceux du portail en anglais ; ils peuvent changer.

1. New Application : nommer l'application Bersemon.
2. Onglet Bot : Reset Token pour obtenir le jeton du bot. Le jeton donne le contrôle du bot : il ne se partage pas et ne s'écrit dans aucun fichier du dépôt. Les Privileged Gateway Intents restent désactivés : les commandes slash n'en ont pas besoin.
3. Onglet OAuth2, URL Generator : cocher les portées `bot` et `applications.commands`, sans aucune permission, puis ouvrir l'adresse générée pour inviter le bot sur un serveur.

Pour jouer en message privé avec le bot, il faut partager au moins un serveur avec lui.

## 2. Lancer le bot

Le bot lit sa configuration dans des variables d'environnement :

| Variable | Rôle |
|---|---|
| `DISCORD_TOKEN` | Jeton du bot, obligatoire. |
| `BERSEMON_SERVEUR_DE_TEST` | Facultative, pour développer : identifiant d'un serveur où les commandes sont enregistrées tout de suite, mais seulement là : `/bersemon` n'est alors disponible ni dans les autres serveurs ni en message privé. Sans elle, les commandes sont enregistrées pour tous les serveurs et les messages privés, ce qui peut prendre jusqu'à une heure. |
| `BERSEMON_DONNEES` | Facultative : dossier des sauvegardes des joueurs, `%APPDATA%\Bersemon\discord` par défaut. Chaque compte Discord y a un fichier, `<identifiant du compte>.json`, séparé de la sauvegarde de la console. |

L'identifiant d'un serveur se copie dans Discord après avoir activé le mode développeur (Paramètres utilisateur, Avancés), par un clic droit sur le serveur.

Dans un terminal PowerShell, à la racine du dépôt :

```powershell
$env:DISCORD_TOKEN = "jeton du bot"
$env:BERSEMON_SERVEUR_DE_TEST = "identifiant du serveur"
cargo run -p bersemon-discord
```

Ces variables ne valent que pour ce terminal. Le bot affiche "Bersemon est connecté à Discord." une fois prêt. Ctrl+C arrête le bot.

Pour jouer en message privé, lancer le bot sans `BERSEMON_SERVEUR_DE_TEST` (dans un terminal où elle a déjà été définie : `Remove-Item Env:BERSEMON_SERVEUR_DE_TEST`). Au démarrage, le bot efface alors les commandes qu'un lancement en mode test a laissées dans ses serveurs, pour que `/bersemon` n'y apparaisse pas en double. La commande globale peut mettre jusqu'à une heure à apparaître en message privé.

## 3. Jouer

`/bersemon` ouvre un message visible du seul joueur, dans un salon comme en message privé. Le message montre ce qui vient de se passer, puis la question en cours ; chaque bouton ou menu y répond, et le message se met à jour. Le nom du héros et les surnoms se saisissent dans un formulaire. Chaque compte Discord a une seule partie, la même partout.

- Au camp : un menu pour partir en expédition, et des boutons pour voir ses monstres, les gérer (choisir un monstre, puis le déplacer, l'envoyer au camp ou le faire venir, l'échanger, le renommer ou le relâcher) et consulter le codex.
- Entre deux étapes : continuer ou rentrer, et des menus pour utiliser une potion ou mettre un monstre en tête de l'équipe.
- En combat : un bouton par attaque, le filet et la fuite, et des menus pour changer de monstre ou utiliser une potion.

La partie est sauvegardée à sa création, puis au camp. Une expédition en cours attend le joueur jusqu'à son prochain `/bersemon` ; si le bot redémarre, chacun reprend à sa dernière sauvegarde. Les composants d'un message dépassé, ouvert avant un autre choix ou avant un redémarrage, ne font rien d'autre que remettre le message à jour.

## 4. Si le jeton a fuité

Le régénérer aussitôt dans l'onglet Bot (Reset Token) : l'ancien cesse de fonctionner.
