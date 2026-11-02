# Déploiement du bot Discord - Bersemon

Le bot tourne en continu sur une VM Linux, une instance Oracle Cloud Always Free (Ubuntu 24.04, Arm aarch64), mise à jour automatiquement depuis GitHub (Q63). Deux bots y tournent côte à côte : le bot en service, que les joueurs utilisent, construit depuis la branche `master`, et le bot de test, construit depuis la branche `dev`. Le lancement à la main sous Windows reste possible pour les essais (section 9).

La création d'une application Discord et le fonctionnement des variables sont décrits dans docs/discord.md.

## 1. Les deux applications Discord

1. Créer deux applications sur le portail développeur (docs/discord.md, section 1), par exemple Bersemon et Bersemon (test). Chacune a son propre jeton.
2. Rendre chacune privée : onglet Bot, décocher Public Bot. Seul le propriétaire de l'application peut alors l'inviter dans un serveur. Si le portail refuse au motif que l'application a un lien d'installation, choisir d'abord None comme Install Link dans l'onglet Installation.
3. Inviter le bot en service dans les serveurs des joueurs, et le bot de test dans un serveur à part.

Deux instances lancées avec le même jeton recevraient toutes les deux les clics des joueurs et écriraient chacune leurs sauvegardes : un jeton ne sert jamais à deux bots en même temps. Une fois la VM en service, le bot en service ne se lance plus sous Windows, et le bot de test de la VM est mis en pause avant un essai sous Windows (section 9).

## 2. Hébergement sur la VM : vue d'ensemble

| Branche | Environnement | Service systemd | Bot |
|---|---|---|---|
| `dev` | dev | `bersemon-dev` | bot de test |
| `master` | prod | `bersemon-prod` | bot en service |

À chaque push sur `dev` ou `master`, GitHub Actions (`.github/workflows/deploiement.yml`) :

1. vérifie le code : `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, avec la `rust-version` du workspace (1.85), celle que Cargo.toml prévoit pour le poste Windows 10 ;
2. si tout passe, se connecte en SSH à la VM avec le compte `deploiement` et demande le déploiement du commit ;
3. sur la VM, le script `bersemon-deployer` récupère ce commit, le compile en release avec la même version de Rust, installe le binaire, redémarre le service et attend jusqu'à 90 secondes que le bot affiche "Bersemon est connecté à Discord.". Sinon, il remet le binaire précédent et le job échoue.

Le déploiement ne tourne que si la variable de dépôt `DEPLOIEMENT_ACTIF` vaut `oui` (section 3.4). Sans elle, seule la vérification a lieu.

Un redémarrage fait perdre les expéditions en cours : chaque joueur reprend à sa dernière sauvegarde (section 13.9 de docs/gamedesign.md). Chaque fusion dans `master` en provoque un pour le bot en service.

### Organisation de la VM

| Élément | Emplacement | Accès |
|---|---|---|
| Code et compilation (`target`) | `/srv/bersemon/<env>/code` | compte `bersemon-<env>` |
| Cache de cargo | `/srv/bersemon/<env>/cargo` | compte `bersemon-<env>` |
| Binaire en service, précédent, commit déployé | `/srv/bersemon/<env>/bin` | compte `bersemon-<env>` |
| Parties des joueurs (`BERSEMON_DONNEES`) | `/srv/bersemon/<env>/parties` | compte `bersemon-<env>` |
| Réglages et jeton | `/etc/bersemon/<env>.env` | compte `bersemon-<env>`, lecture et écriture, 0600 |
| Rust (rustup et toolchains) | `/opt/rust` | root, lecture pour tous |
| Scripts | `/usr/local/sbin/bersemon-*` | root |
| Sauvegardes | `/var/backups/bersemon/<env>` | root, lecture pour le groupe `bersemon-sauvegarde` |

`<env>` vaut `prod` ou `dev`. Les deux environnements ne partagent ni code, ni compilation, ni parties.

### Comptes et sécurité

- `bersemon-prod` et `bersemon-dev` : comptes système sans connexion. Chacun compile et exécute son bot, et ne peut ni lire ni modifier les fichiers de l'autre. Le code de `dev`, où Claude fusionne seul, ne peut donc ni lire le jeton du bot en service ni toucher à ses parties.
- Le service n'a le droit d'écrire que dans son dossier de parties : ni dans son binaire, ni dans son code, ni ailleurs dans le système. Il refuse de démarrer si `BERSEMON_DONNEES` désigne un autre dossier. Il est relancé 10 secondes après tout arrêt, sans limite.
- `deploiement` : compte SSH de GitHub Actions. Sa clé est déclarée avec une commande forcée : quelle que soit la commande envoyée, seul `bersemon-deploiement-ssh` s'exécute, sans terminal ni redirection de port. Ce script transmet l'environnement et le commit à `bersemon-deployer`, seule commande que sudo lui autorise. `bersemon-deployer` n'accepte que `prod` ou `dev` et un commit de la branche correspondante : une clé volée ne permettrait de déployer que du code déjà fusionné.
- Les scripts exécutés en root ne sont jamais mis à jour par un push : seul `installer-vm.sh`, lancé à la main, les installe (section 7). Sinon, une fusion dans `dev` suffirait à exécuter du code en root.
- Aucun port entrant n'est ouvert, à part SSH : le bot ouvre lui-même la connexion vers Discord.

Les fichiers de la VM sont dans `deploiement/linux` : `installer-vm.sh`, `bersemon-deployer`, `bersemon-deploiement-ssh`, `bersemon-sauvegarder`, le modèle des services, le service et le timer de sauvegarde, et le modèle des réglages.

## 3. Installer la VM

### 3.1 Créer la VM chez Oracle

Les intitulés de la console Oracle peuvent changer.

1. Sur le PC Windows, créer une clé SSH d'administration, dans PowerShell : `ssh-keygen -t ed25519 -f "$env:USERPROFILE\.ssh\bersemon-admin"`. Une phrase de passe est conseillée.
2. Dans la console Oracle Cloud, Compute, Instances, Create instance :
   - image : Canonical Ubuntu 24.04, dans sa version aarch64, proposée avec la forme Ampere. La version Minimal convient aussi, mais il lui manque des outils (section 3.2) ;
   - forme : VM.Standard.A1.Flex, par exemple 2 OCPU et 12 Go de mémoire. Le quota Always Free des instances Ampere A1 est, à ma connaissance, de 4 OCPU et 24 Go au total : le vérifier sur la page Always Free Resources d'Oracle ;
   - réseau : une adresse IP publique ;
   - clé SSH : coller le contenu de `bersemon-admin.pub`.
3. Dans le réseau virtuel de l'instance (VCN, Security List), vérifier que les règles d'entrée n'ouvrent que TCP 22.
4. Se connecter : `ssh -i "$env:USERPROFILE\.ssh\bersemon-admin" ubuntu@<IP publique>`. Le compte d'administration des images Ubuntu d'Oracle est `ubuntu`.

Deux risques propres à Oracle Always Free, à vérifier dans la documentation d'Oracle :

- Oracle peut récupérer les instances Always Free jugées inactives sur 7 jours (processeur, réseau et mémoire sous un seuil, cité à 10, 15 ou 20 % selon les sources). Un bot Discord est presque toujours inactif. La parade citée est de passer le compte en Pay As You Go, où les ressources Always Free restent gratuites. Générer une charge artificielle n'est pas recommandé.
- La capacité Ampere A1 est souvent épuisée dans certaines régions : la création de l'instance peut échouer et doit être retentée plus tard.

Le script d'installation étant rejouable, une VM perdue se reconstruit en quelques commandes (section 8).

### 3.2 Lancer le script d'installation

Sur la VM, installer d'abord git et un éditeur, qui manquent parfois aux images Minimal (sans effet s'ils sont déjà là) :

```bash
sudo apt-get update
```

```bash
sudo apt-get install -y git nano
```

Puis :

```bash
git clone https://github.com/romainhnr/Bersemon.git
```

```bash
sudo bash ~/Bersemon/deploiement/linux/installer-vm.sh
```

Le script installe les paquets nécessaires, rustup dans `/opt/rust`, crée les comptes et les dossiers, clone le dépôt pour chaque environnement, installe les scripts, la règle sudo, les services et le timer de sauvegarde, et crée `/etc/bersemon/prod.env` et `/etc/bersemon/dev.env` s'ils n'existent pas. Il ajoute le compte qui l'a lancé avec sudo au groupe `bersemon-sauvegarde`, effectif à la connexion suivante.

Pare-feu de la VM : si ufw est actif, ou si des règles iptables filtrent déjà les entrées, comme sur les images Oracle, le script les laisse telles quelles et les affiche pour vérification. Sinon, il active ufw : tout est refusé en entrée sauf SSH. `BERSEMON_PARE_FEU=non` lui fait ignorer cette étape.

Il se termine par la liste de ce qui reste à faire. Il peut être relancé autant de fois que nécessaire : il ne touche ni aux jetons ni aux parties.

### 3.3 Jetons et réglages

Pour chaque environnement :

```bash
sudoedit /etc/bersemon/prod.env
```

```bash
sudoedit /etc/bersemon/dev.env
```

| Variable | Contenu |
|---|---|
| `DISCORD_TOKEN` | Jeton du bot : celui du bot en service dans `prod.env`, celui du bot de test dans `dev.env`. |
| `BERSEMON_DONNEES` | Déjà renseignée, à ne pas changer : `/srv/bersemon/<env>/parties`. |
| `BERSEMON_SERVEUR_DE_TEST` | Facultative, dans `dev.env` seulement : identifiant du serveur de test. |
| `BERSEMON_MESSAGES_PUBLICS` | Facultative : messages de `/bersemon` visibles de tout le salon, pour tester à plusieurs. |

Le fichier ne contient que des lignes `NOM=valeur`, sans guillemets. Ces fichiers ne sont lisibles que par le compte du service, et par root ; ils ne sont jamais copiés dans le dépôt. Après une modification : `sudo systemctl restart bersemon-prod` (ou `bersemon-dev`).

Si un jeton a fuité, le régénérer dans l'onglet Bot du portail (Reset Token), puis mettre le fichier à jour et redémarrer le service.

### 3.4 Clé de déploiement et GitHub

1. Sur le PC Windows, créer la clé de GitHub Actions, sans phrase de passe (Entrée deux fois quand elle est demandée) : `ssh-keygen -t ed25519 -C github-actions-bersemon -f "$env:USERPROFILE\.ssh\bersemon-deploiement"`.
2. Copier la clé publique sur la VM, depuis PowerShell : `scp -i "$env:USERPROFILE\.ssh\bersemon-admin" "$env:USERPROFILE\.ssh\bersemon-deploiement.pub" ubuntu@<IP publique>:deploiement.pub`, puis sur la VM :

   ```bash
   sudo cp ~/deploiement.pub /etc/bersemon/deploiement.pub
   ```

   ```bash
   sudo bash ~/Bersemon/deploiement/linux/installer-vm.sh
   ```

3. Relever l'empreinte de la VM, sur la VM : `echo "<IP publique> $(cut -d ' ' -f 1,2 /etc/ssh/ssh_host_ed25519_key.pub)"`.
4. Dans GitHub, Settings du dépôt, Secrets and variables, Actions, onglet Secrets, créer :

   | Secret | Valeur |
   |---|---|
   | `DEPLOIEMENT_HOTE` | Adresse IP publique de la VM. |
   | `DEPLOIEMENT_CLE_SSH` | Contenu complet du fichier `bersemon-deploiement` (la clé privée, sans `.pub`). |
   | `DEPLOIEMENT_HOTE_CONNU` | La ligne affichée à l'étape 3. |

5. Supprimer la clé privée du PC une fois copiée dans GitHub, ou la ranger hors de portée : elle n'est utile qu'à GitHub.
6. Après le premier déploiement manuel réussi (section 3.5), onglet Variables, créer `DEPLOIEMENT_ACTIF` avec la valeur `oui`.

Le dépôt étant public, la VM récupère le code sans identifiant. S'il devenait privé, il faudrait ajouter une clé de lecture (deploy key) au dépôt et l'utiliser sur la VM : ce cas n'est pas prévu par les scripts.

### 3.5 Premier déploiement

Une fois les jetons renseignés, sur la VM :

```bash
sudo bersemon-deployer dev
```

```bash
sudo bersemon-deployer prod
```

Sans commit, le script déploie le dernier commit de la branche. La première fois, il installe la version de Rust du workspace et compile toutes les dépendances : compter plusieurs minutes.

### 3.6 Reprendre les parties du PC Windows

Si des joueurs ont déjà des parties sur le bot lancé sous Windows, les transférer avant le premier déploiement de prod, bot Windows arrêté pour de bon. Depuis PowerShell :

```powershell
scp -i "$env:USERPROFILE\.ssh\bersemon-admin" -r "$env:APPDATA\Bersemon\discord" ubuntu@<IP publique>:parties-import
```

Puis sur la VM :

```bash
sudo systemctl stop bersemon-prod
```

```bash
sudo cp -r ~/parties-import/. /srv/bersemon/prod/parties/
```

```bash
sudo chown -R bersemon-prod:bersemon-prod /srv/bersemon/prod/parties
```

```bash
rm -r ~/parties-import
```

Puis `sudo systemctl start bersemon-prod`, ou le premier `sudo bersemon-deployer prod`. Même chose pour le bot de test avec `discord-test`, `bersemon-dev` et `/srv/bersemon/dev/parties`, si ses parties méritent d'être gardées.

## 4. Déploiement au quotidien

- Push sur `dev` : le bot de test est mis à jour. Claude y fusionne ses branches quand fmt, clippy et les tests passent (CLAUDE.md).
- Fusion de `dev` dans `master`, par l'utilisateur seul : le bot en service est mis à jour.
- Le résultat se lit dans l'onglet Actions du dépôt : la sortie de la compilation sur la VM y apparaît.
- Un échec de vérification n'arrête rien sur la VM : le bot continue avec la version précédente.
- Un seul déploiement à la fois par environnement ; un push plus récent attend la fin du précédent.

Déployer à la main un commit précis, par exemple pour revenir à une version antérieure :

```bash
sudo bersemon-deployer prod <identifiant complet du commit>
```

Le commit doit appartenir à la branche de l'environnement. Attention : un binaire plus ancien ne lit pas une partie déjà enregistrée dans un format de sauvegarde plus récent. Le retour automatique au binaire précédent, lui, est sans risque : le nouveau binaire ne s'est jamais connecté, donc n'a rien enregistré.

Le commit en service : `sudo cat /srv/bersemon/prod/bin/version`.

## 5. Exploitation

| Besoin | Commande |
|---|---|
| État du bot | `systemctl status bersemon-prod` |
| Messages du bot, en continu | `journalctl -u bersemon-prod -f` |
| 100 derniers messages | `journalctl -u bersemon-prod -n 100` |
| Redémarrer | `sudo systemctl restart bersemon-prod` |
| Mettre en pause, sans relance au déploiement ni au démarrage de la VM | `sudo systemctl disable --now bersemon-dev` |
| Reprendre | `sudo systemctl enable --now bersemon-dev` |

Remplacer `prod` par `dev` pour le bot de test. Un service en pause reçoit quand même les nouvelles versions : le déploiement installe le binaire sans le lancer.

## 6. Sauvegarde et restauration

Chaque jour à 4 h, heure de Paris, le timer `bersemon-sauvegarde` archive le dossier de parties de chaque environnement dans `/var/backups/bersemon/<env>/parties-<env>-AAAA-MM-JJ.tar.gz`. Si la VM était éteinte à cette heure, la sauvegarde a lieu à son démarrage. Les 14 archives les plus récentes de chaque environnement sont gardées : la rotation se fait par nombre, pour qu'une longue extinction ne supprime pas tout. Le nombre se règle par `BERSEMON_CONSERVATION` dans `deploiement/linux/bersemon-sauvegarde.service`.

L'archive est faite bot en marche. Chaque partie est écrite dans un fichier temporaire puis renommée, donc chaque partie archivée est complète. Les fichiers `retours-annonces.txt` et `pseudos.txt` sont en revanche réécrits directement : en cas de malchance, l'archive peut en contenir une version tronquée. Au pire, un retour d'expédition est alors annoncé une seconde fois, ou un pseudo manque dans la liste des joueurs.

| Besoin | Commande, sur la VM |
|---|---|
| Lancer une sauvegarde tout de suite | `sudo systemctl start bersemon-sauvegarde` |
| Prochain passage | `systemctl list-timers bersemon-sauvegarde.timer` |
| Résultat du dernier passage | `journalctl -u bersemon-sauvegarde -n 20` |

### Copier les sauvegardes hors de la VM

Depuis PowerShell, sur le PC Windows (client OpenSSH de Windows) :

```powershell
scp -i "$env:USERPROFILE\.ssh\bersemon-admin" -r ubuntu@<IP publique>:/var/backups/bersemon "$env:USERPROFILE\Documents\bersemon-sauvegardes-$(Get-Date -Format yyyy-MM-dd)"
```

Le compte d'administration doit être dans le groupe `bersemon-sauvegarde` (section 3.2).

### Restaurer

Sur la VM, pour le bot en service (remplacer `prod` par `dev` pour le bot de test) :

```bash
sudo systemctl stop bersemon-prod
```

```bash
sudo mv /srv/bersemon/prod/parties /srv/bersemon/prod/parties-avant-restauration
```

```bash
sudo tar -xzf /var/backups/bersemon/prod/parties-prod-AAAA-MM-JJ.tar.gz -C /srv/bersemon/prod
```

```bash
sudo chown -R bersemon-prod:bersemon-prod /srv/bersemon/prod/parties
```

```bash
sudo systemctl start bersemon-prod
```

Une archive copiée sur le PC se renvoie d'abord sur la VM avec `scp`. Supprimer `parties-avant-restauration` une fois la restauration vérifiée.

## 7. Mettre à jour la VM

Après une modification des fichiers de `deploiement/linux`, une fois fusionnée dans `master`, sur la VM :

```bash
git -C ~/Bersemon pull
```

```bash
sudo bash ~/Bersemon/deploiement/linux/installer-vm.sh
```

Si les unités des bots ont changé, le script l'indique : les appliquer par `sudo systemctl restart bersemon-prod bersemon-dev`.

Ubuntu installe ses mises à jour de sécurité tout seul (unattended-upgrades, à ma connaissance actif par défaut sur les images Ubuntu ; le vérifier avec `systemctl status unattended-upgrades`). Pour le reste : `sudo apt update`, `sudo apt upgrade`, puis `sudo reboot` si demandé. Les bots et le timer redémarrent avec la VM.

## 8. Changer d'hébergeur

1. Créer une VM Ubuntu 24.04, aarch64 ou x86_64, avec la clé d'administration, et n'y ouvrir que SSH chez l'hébergeur.
2. Sur l'ancienne VM : `sudo systemctl disable --now bersemon-prod bersemon-dev`, pour qu'un même jeton ne serve pas à deux bots. Puis une dernière sauvegarde, `sudo systemctl start bersemon-sauvegarde`, et la copier sur le PC (section 6).
3. Sur la nouvelle VM : cloner le dépôt et lancer le script (section 3.2), renseigner les jetons (section 3.3) et copier la clé publique de déploiement (section 3.4).
4. Restaurer la dernière sauvegarde de chaque environnement (section 6).
5. Déployer à la main (section 3.5), puis mettre à jour les secrets `DEPLOIEMENT_HOTE` et `DEPLOIEMENT_HOTE_CONNU` dans GitHub.

## 9. Lancer le bot sous Windows

Le lancement à la main sous Windows reste possible, pour des essais en local. Il utilise les mêmes jetons que la VM :

- ne plus lancer le bot en service sous Windows tant que la VM est en service ;
- avant un essai avec le bot de test, le mettre en pause sur la VM : `sudo systemctl disable --now bersemon-dev`, puis `sudo systemctl enable --now bersemon-dev` une fois l'essai fini (section 5). En pause, il n'est pas relancé par un déploiement.

### 9.1 Jetons et réglages

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

Ces fichiers ne doivent jamais être copiés dans le dépôt. Si un jeton a fuité, le régénérer dans l'onglet Bot (Reset Token) et mettre à jour le fichier, ainsi que le fichier de la VM (section 3.3).

Les parties des joueurs sont rangées à part pour chaque bot : `%APPDATA%\Bersemon\discord` pour le bot en service, `%APPDATA%\Bersemon\discord-test` pour le bot de test. Chaque dossier contient aussi `retours-annonces.txt`, la liste des retours d'expéditions automatiques déjà annoncés par message privé. La sauvegarde de la console, `%APPDATA%\Bersemon\sauvegarde.json`, n'est pas concernée. Ces parties sont distinctes de celles de la VM.

### 9.2 Lancer le bot

Dans un terminal PowerShell, à la racine du dépôt :

```powershell
powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1
```

Pour le bot de test :

```powershell
powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1 -Test
```

Pour tester à plusieurs, l'option `-Public` rend les messages de `/bersemon` visibles de tout le salon ; seul le joueur qui a ouvert un message peut se servir de ses boutons :

```powershell
powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1 -Test -Public
```

L'option `-ExecutionPolicy Bypass` ne vaut que pour ce lancement : elle permet d'exécuter le script du dépôt sans changer la stratégie d'exécution de Windows.

Le script compile le bot en mode release s'il a changé, renseigne ses variables à partir des fichiers de la section 9.1, puis le lance. Le bot en service enregistre ses commandes pour tous les serveurs et les messages privés ; le bot de test, dans le serveur de test s'il y en a un. Les messages du bot s'affichent dans le terminal ; Ctrl+C l'arrête. Les expéditions en cours sont alors perdues, et chaque joueur reprend à sa dernière sauvegarde (section 13.9 de docs/gamedesign.md).

### 9.3 Mettre à jour

Arrêter le bot, récupérer la nouvelle version du code, puis le relancer avec le script : il recompile ce qui a changé. Les sauvegardes restent lisibles d'une version à l'autre, car leur format est versionné.

### 9.4 Copie de sécurité

Les parties sont de simples fichiers : il suffit de copier le dossier, de préférence bot arrêté. Par exemple, dans PowerShell :

```powershell
Copy-Item -Recurse "$env:APPDATA\Bersemon\discord" "$env:USERPROFILE\Documents\bersemon-discord-$(Get-Date -Format yyyy-MM-dd)"
```

Pour restaurer, remettre le dossier copié à la place de `%APPDATA%\Bersemon\discord`, bot arrêté.
