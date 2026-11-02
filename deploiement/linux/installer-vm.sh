#!/bin/bash
# Installe, ou met à jour, la VM Linux qui héberge les bots Discord de
# Bersemon : bot en service (prod, branche master) et bot de test (dev,
# branche dev). Pensé pour Ubuntu 24.04, en aarch64 comme en x86_64.
#
# Usage, depuis un clone du dépôt :
#     sudo bash deploiement/linux/installer-vm.sh
#
# Rejouable : chaque étape vérifie l'état avant d'agir. Les jetons
# (/etc/bersemon/*.env) et les parties des joueurs ne sont jamais écrasés.
# À relancer après une modification des fichiers de deploiement/linux : un
# push ne met jamais à jour les scripts exécutés en root.
#
# Variables facultatives :
#     BERSEMON_DEPOT      adresse du dépôt Git (par défaut, le dépôt GitHub)
#     BERSEMON_ADMIN      compte d'administration, autorisé à lire les
#                         sauvegardes (par défaut, celui qui a lancé sudo)
#     BERSEMON_PARE_FEU   "non" pour ne pas toucher au pare-feu de la VM
#
# Voir docs/deploiement.md.

set -euo pipefail
umask 022

SOURCE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
readonly SOURCE
readonly DEPOT=${BERSEMON_DEPOT:-https://github.com/romainhnr/Bersemon.git}
readonly ADMIN=${BERSEMON_ADMIN:-${SUDO_USER:-}}
readonly PARE_FEU=${BERSEMON_PARE_FEU:-oui}
readonly RACINE=/srv/bersemon
readonly RUST=/opt/rust
readonly REGLAGES=/etc/bersemon
readonly SAUVEGARDES=/var/backups/bersemon
readonly GROUPE_SAUVEGARDE=bersemon-sauvegarde
readonly COMPTE_DEPLOIEMENT=deploiement
readonly CLE_DEPLOIEMENT=$REGLAGES/deploiement.pub
readonly ENVIRONNEMENTS=(prod dev)

reste_a_faire=()
a_redemarrer=()
nouveaux_services=()

etape() {
    printf '\n== %s\n' "$*"
}

echec() {
    echo "Installation interrompue : $*" >&2
    exit 1
}

# Commande lancée sous le compte d'un environnement, depuis son dossier et
# avec un environnement minimal, comme dans bersemon-deployer.
en_tant_que() {
    local utilisateur=$1 dossier=$2
    shift 2
    (cd "$dossier" && runuser -u "$utilisateur" -- env -i \
        HOME="$dossier" PATH=/usr/bin:/bin LANG=C.UTF-8 GIT_TERMINAL_PROMPT=0 "$@")
}

# Installe un fichier généré à sa place, en signalant s'il a changé.
remplacer_si_different() {
    local nouveau=$1 cible=$2
    if [[ -f $cible ]] && cmp -s "$nouveau" "$cible"; then
        rm -f "$nouveau"
        return 1
    fi
    mv -f "$nouveau" "$cible"
    return 0
}

[[ $EUID -eq 0 ]] || echec "lancer ce script avec sudo."
command -v apt-get >/dev/null || echec "ce script suppose une distribution à base de Debian ou d'Ubuntu (apt-get)."
for fichier in bersemon-deployer bersemon-deploiement-ssh bersemon-sauvegarder \
    bersemon.service.modele bersemon-sauvegarde.service bersemon-sauvegarde.timer modele.env; do
    [[ -f $SOURCE/$fichier ]] || echec "$SOURCE/$fichier est introuvable : lancer le script depuis un clone du dépôt."
done

case "$(uname -m)" in
    aarch64) readonly CIBLE_RUST=aarch64-unknown-linux-gnu ;;
    x86_64) readonly CIBLE_RUST=x86_64-unknown-linux-gnu ;;
    *) echec "architecture $(uname -m) non prévue." ;;
esac

etape "Paquets"
export DEBIAN_FRONTEND=noninteractive
apt-get update -q
# build-essential fournit le compilateur C dont a besoin la crate ring (TLS).
apt-get install -y -q --no-install-recommends ca-certificates curl git build-essential

etape "Rust"
# rustup dans /opt/rust, à root : les comptes des bots n'y écrivent pas. Les
# toolchains sont installées par bersemon-deployer, selon la rust-version du
# dépôt.
if [[ -x $RUST/cargo/bin/rustup ]]; then
    echo "rustup est déjà installé dans $RUST."
else
    temporaire=$(mktemp -d)
    trap 'rm -rf "$temporaire"' EXIT
    adresse="https://static.rust-lang.org/rustup/dist/$CIBLE_RUST/rustup-init"
    curl --proto '=https' --tlsv1.2 -sSf -o "$temporaire/rustup-init" "$adresse"
    curl --proto '=https' --tlsv1.2 -sSf -o "$temporaire/rustup-init.sha256" "$adresse.sha256"
    empreinte=$(cut -d ' ' -f 1 "$temporaire/rustup-init.sha256")
    echo "$empreinte  $temporaire/rustup-init" | sha256sum --check --quiet \
        || echec "l'empreinte de rustup-init ne correspond pas."
    chmod 0755 "$temporaire/rustup-init"
    RUSTUP_HOME=$RUST/rustup CARGO_HOME=$RUST/cargo "$temporaire/rustup-init" \
        -y --no-modify-path --profile minimal --default-toolchain none
    echo "rustup installé dans $RUST."
fi

etape "Comptes, dossiers et clones"
install -d -o root -g root -m 0755 "$RACINE" "$REGLAGES"
if ! getent group "$GROUPE_SAUVEGARDE" >/dev/null; then
    groupadd --system "$GROUPE_SAUVEGARDE"
fi
install -d -o root -g "$GROUPE_SAUVEGARDE" -m 2750 "$SAUVEGARDES"
if [[ -n $ADMIN && $ADMIN != root ]] && id -u "$ADMIN" >/dev/null 2>&1; then
    usermod -a -G "$GROUPE_SAUVEGARDE" "$ADMIN"
    echo "$ADMIN peut lire les sauvegardes (groupe $GROUPE_SAUVEGARDE, à la prochaine connexion)."
else
    reste_a_faire+=("Ajouter le compte d'administration au groupe $GROUPE_SAUVEGARDE : sudo usermod -a -G $GROUPE_SAUVEGARDE <compte>.")
fi

for environnement in "${ENVIRONNEMENTS[@]}"; do
    utilisateur="bersemon-$environnement"
    dossier="$RACINE/$environnement"
    if ! id -u "$utilisateur" >/dev/null 2>&1; then
        useradd --system --user-group --home-dir "$dossier" --no-create-home \
            --shell /usr/sbin/nologin "$utilisateur"
        echo "Compte $utilisateur créé."
    fi
    # Chaque environnement a ses propres code, compilation, cache cargo,
    # binaire et parties, illisibles par l'autre.
    install -d -o "$utilisateur" -g "$utilisateur" -m 0750 \
        "$dossier" "$dossier/cargo" "$dossier/bin" "$dossier/parties"

    if [[ -d $dossier/code/.git ]]; then
        en_tant_que "$utilisateur" "$dossier" git -C "$dossier/code" remote set-url origin "$DEPOT"
        echo "Clone de $environnement déjà présent : $dossier/code."
    else
        en_tant_que "$utilisateur" "$dossier" git clone --quiet "$DEPOT" "$dossier/code"
        echo "Dépôt cloné dans $dossier/code."
    fi

    reglages="$REGLAGES/$environnement.env"
    if [[ ! -f $reglages ]]; then
        sed "s/@ENV@/$environnement/g" "$SOURCE/modele.env" >"$reglages"
        echo "Réglages créés : $reglages."
    fi
    # Le fichier contient le jeton : lisible par le seul compte du service.
    chown "$utilisateur:$utilisateur" "$reglages"
    chmod 0600 "$reglages"
    if ! grep -Eq '^DISCORD_TOKEN=.+' "$reglages"; then
        reste_a_faire+=("Renseigner DISCORD_TOKEN dans $reglages : sudoedit $reglages.")
    fi
done

etape "Scripts et services"
install -o root -g root -m 0755 "$SOURCE/bersemon-deployer" /usr/local/sbin/bersemon-deployer
install -o root -g root -m 0755 "$SOURCE/bersemon-deploiement-ssh" /usr/local/sbin/bersemon-deploiement-ssh
install -o root -g root -m 0755 "$SOURCE/bersemon-sauvegarder" /usr/local/sbin/bersemon-sauvegarder
for environnement in "${ENVIRONNEMENTS[@]}"; do
    unite="/etc/systemd/system/bersemon-$environnement.service"
    if [[ ! -f $unite ]]; then
        nouveaux_services+=("bersemon-$environnement.service")
    fi
    sed "s/@ENV@/$environnement/g" "$SOURCE/bersemon.service.modele" >"$unite.nouveau"
    chmod 0644 "$unite.nouveau"
    if remplacer_si_different "$unite.nouveau" "$unite" \
        && systemctl is-active --quiet "bersemon-$environnement"; then
        a_redemarrer+=("bersemon-$environnement")
    fi
done
for unite in bersemon-sauvegarde.service bersemon-sauvegarde.timer; do
    install -o root -g root -m 0644 "$SOURCE/$unite" "/etc/systemd/system/$unite"
done
systemctl daemon-reload
# Les bots démarrent au premier déploiement, une fois leur binaire compilé.
# Seuls les services nouveaux sont activés : un service désactivé à la main
# est en pause (docs/deploiement.md) et le reste.
if [[ ${#nouveaux_services[@]} -gt 0 ]]; then
    systemctl enable --quiet "${nouveaux_services[@]}"
    echo "Services activés : ${nouveaux_services[*]}."
fi
systemctl enable --now --quiet bersemon-sauvegarde.timer
echo "Sauvegarde quotidienne programmée."

etape "Compte de déploiement"
if ! id -u "$COMPTE_DEPLOIEMENT" >/dev/null 2>&1; then
    # Un vrai shell est nécessaire : sshd lance la commande forcée par lui.
    useradd --create-home --shell /bin/sh "$COMPTE_DEPLOIEMENT"
    echo "Compte $COMPTE_DEPLOIEMENT créé."
fi
# Aucun mot de passe valable ; "*" plutôt que "!", que sshd peut prendre pour
# un compte verrouillé et refuser même avec une clé.
usermod -p '*' "$COMPTE_DEPLOIEMENT"
dossier_deploiement=$(getent passwd "$COMPTE_DEPLOIEMENT" | cut -d : -f 6)
# .ssh et authorized_keys appartiennent à root : le compte ne peut pas retirer
# sa commande forcée.
install -d -o root -g root -m 0755 "$dossier_deploiement/.ssh"
if [[ -f $CLE_DEPLOIEMENT ]]; then
    cle=$(grep -Ev '^[[:space:]]*(#|$)' "$CLE_DEPLOIEMENT" | head -n 1 || true)
    [[ $cle =~ ^(ssh-ed25519|ecdsa-sha2-nistp256|ecdsa-sha2-nistp384|ecdsa-sha2-nistp521|ssh-rsa)\ [A-Za-z0-9+/]+=*(\ .*)?$ ]] \
        || echec "$CLE_DEPLOIEMENT ne contient pas une clé publique SSH."
    printf 'restrict,command="/usr/local/sbin/bersemon-deploiement-ssh" %s\n' "$cle" \
        >"$dossier_deploiement/.ssh/authorized_keys.nouveau"
    chmod 0644 "$dossier_deploiement/.ssh/authorized_keys.nouveau"
    mv -f "$dossier_deploiement/.ssh/authorized_keys.nouveau" "$dossier_deploiement/.ssh/authorized_keys"
    echo "Clé de déploiement autorisée, limitée au script de déploiement."
elif [[ ! -f $dossier_deploiement/.ssh/authorized_keys ]]; then
    reste_a_faire+=("Copier la clé publique de déploiement dans $CLE_DEPLOIEMENT, puis relancer ce script.")
fi

regle_sudo=$(mktemp)
cat >"$regle_sudo" <<EOF
# Installé par installer-vm.sh : le compte $COMPTE_DEPLOIEMENT ne peut lancer
# que le script de déploiement, qui vérifie lui-même ses arguments.
$COMPTE_DEPLOIEMENT ALL=(root) NOPASSWD: /usr/local/sbin/bersemon-deployer
EOF
visudo -c -q -f "$regle_sudo" || { rm -f "$regle_sudo"; echec "la règle sudo générée est invalide."; }
install -o root -g root -m 0440 "$regle_sudo" /etc/sudoers.d/bersemon-deploiement
rm -f "$regle_sudo"
echo "Règle sudo installée : /etc/sudoers.d/bersemon-deploiement."

etape "Pare-feu"
# Les bots n'ont besoin d'aucun port entrant : ils ouvrent eux-mêmes la
# connexion vers Discord. Seul SSH reste ouvert. Un pare-feu déjà actif n'est
# pas modifié, pour ne jamais couper l'accès à la VM.
if [[ $PARE_FEU == non ]]; then
    echo "Pare-feu laissé tel quel (BERSEMON_PARE_FEU=non)."
elif command -v ufw >/dev/null && LC_ALL=C ufw status | grep -q '^Status: active'; then
    echo "ufw est déjà actif, règles laissées telles quelles :"
    LC_ALL=C ufw status
    reste_a_faire+=("Vérifier que le pare-feu de la VM n'ouvre que SSH (règles ci-dessus).")
elif command -v iptables >/dev/null \
    && iptables -S INPUT 2>/dev/null | grep -Eq '^-P INPUT (DROP|REJECT)|-j (DROP|REJECT)'; then
    echo "Des règles iptables filtrent déjà les entrées (cas des images Oracle), laissées telles quelles :"
    iptables -S INPUT
    reste_a_faire+=("Vérifier que le pare-feu de la VM n'ouvre que SSH (règles ci-dessus).")
else
    ports_ssh=$(sshd -T 2>/dev/null | awk '$1 == "port" { print $2 }' | sort -u || true)
    ports_ssh=${ports_ssh:-22}
    apt-get install -y -q --no-install-recommends ufw
    ufw default deny incoming
    ufw default allow outgoing
    for port in $ports_ssh; do
        ufw allow "$port/tcp"
    done
    ufw --force enable
    echo "ufw activé : seules les connexions SSH entrent."
fi
reste_a_faire+=("Vérifier chez l'hébergeur (Security List chez Oracle) que seul le port 22 est ouvert en entrée.")

etape "Terminé"
if [[ ${#a_redemarrer[@]} -gt 0 ]]; then
    echo "Unités modifiées, à appliquer par : sudo systemctl restart ${a_redemarrer[*]}"
    echo "Le redémarrage fait perdre les expéditions en cours des joueurs."
fi
if [[ ${#reste_a_faire[@]} -gt 0 ]]; then
    echo "Reste à faire :"
    for action in "${reste_a_faire[@]}"; do
        echo "  - $action"
    done
fi
echo "Premier déploiement, une fois les jetons renseignés : sudo bersemon-deployer dev, puis sudo bersemon-deployer prod."
