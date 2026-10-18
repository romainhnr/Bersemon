# Déploiement du bot Discord - Bersemon

Le bot tourne sur le PC Windows 11 de l'utilisateur, lancé à la main quand il doit être en ligne (Q63). Deux applications Discord sont utilisées : le bot en service, que les joueurs utilisent, et un bot de test pour les essais. La création d'une application et le fonctionnement des variables sont décrits dans docs/discord.md.

## 1. Les deux applications Discord

1. Créer deux applications sur le portail développeur (docs/discord.md, section 1), par exemple Bersemon et Bersemon (test). Chacune a son propre jeton.
2. Rendre chacune privée : onglet Bot, décocher Public Bot. Seul le propriétaire de l'application peut alors l'inviter dans un serveur. Si le portail refuse au motif que l'application a un lien d'installation, choisir d'abord None comme Install Link dans l'onglet Installation.
3. Inviter le bot en service dans les serveurs des joueurs, et le bot de test dans un serveur à part.

Deux instances lancées avec le même jeton recevraient toutes les deux les clics des joueurs et écriraient les mêmes sauvegardes : les essais se font toujours avec le bot de test, jamais avec le jeton du bot en service.

## 2. Jetons et réglages

Le script de lancement lit ses réglages dans `%APPDATA%\Bersemon`, hors du dépôt :

| Fichier | Contenu |
|---|---|
| `jeton-production.txt` | Jeton du bot en service. |
| `jeton-test.txt` | Jeton du bot de test. |
| `serveur-de-test.txt` | Facultatif : identifiant du serveur de test, où les commandes du bot de test apparaissent tout de suite. |

Chaque fichier ne contient que la valeur, sur une ligne. Pour en créer un dans PowerShell :

```powershell
Set-Content -Path "$env:APPDATA\Bersemon\jeton-production.txt" -Value "jeton du bot en service"
```

Ces fichiers ne doivent jamais être copiés dans le dépôt. Si un jeton a fuité, le régénérer dans l'onglet Bot (Reset Token) et mettre le fichier à jour.

Les parties des joueurs sont rangées à part pour chaque bot : `%APPDATA%\Bersemon\discord` pour le bot en service, `%APPDATA%\Bersemon\discord-test` pour le bot de test. La sauvegarde de la console, `%APPDATA%\Bersemon\sauvegarde.json`, n'est pas concernée.

## 3. Lancer le bot

Dans un terminal PowerShell, à la racine du dépôt :

```powershell
powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1
```

Pour le bot de test :

```powershell
powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1 -Test
```

L'option `-ExecutionPolicy Bypass` ne vaut que pour ce lancement : elle permet d'exécuter le script du dépôt sans changer la stratégie d'exécution de Windows.

Le script compile le bot en mode release s'il a changé, renseigne ses variables à partir des fichiers de la section 2, puis le lance. Le bot en service enregistre ses commandes pour tous les serveurs et les messages privés ; le bot de test, dans le serveur de test s'il y en a un. Les messages du bot s'affichent dans le terminal ; Ctrl+C l'arrête. Les expéditions en cours sont alors perdues, et chaque joueur reprend à sa dernière sauvegarde (section 13.9 de docs/gamedesign.md).

## 4. Mettre à jour

Arrêter le bot, récupérer la nouvelle version du code, puis le relancer avec le script : il recompile ce qui a changé. Les sauvegardes restent lisibles d'une version à l'autre, car leur format est versionné.

## 5. Copie de sécurité

Les parties sont de simples fichiers : il suffit de copier le dossier, de préférence bot arrêté. Par exemple, dans PowerShell :

```powershell
Copy-Item -Recurse "$env:APPDATA\Bersemon\discord" "$env:USERPROFILE\Documents\bersemon-discord-$(Get-Date -Format yyyy-MM-dd)"
```

Pour restaurer, remettre le dossier copié à la place de `%APPDATA%\Bersemon\discord`, bot arrêté.

## 6. Plus tard : un serveur permanent

Si des joueurs veulent le bot en ligne en continu, il faudra une machine allumée en permanence, par exemple un serveur Linux loué, avec un service qui relance le bot s'il s'arrête. Ce choix reste à faire.
