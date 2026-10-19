<#
Lance le bot Discord de Bersemon, compilé en mode release, depuis le dépôt.

Par défaut, le bot en service : jeton dans %APPDATA%\Bersemon\jeton-production.txt,
parties dans %APPDATA%\Bersemon\discord, commandes enregistrées pour tous les
serveurs et les messages privés.

Avec -Test, le bot de développement : jeton dans jeton-test.txt, parties dans
discord-test, et commandes enregistrées seulement dans le serveur de test dont
l'identifiant est dans serveur-de-test.txt, si ce fichier existe.

Avec -Public, option de test : /bersemon répond par un message visible de tout
le salon, et seul le joueur qui l'a ouvert peut se servir de ses boutons.

Les jetons restent dans ces fichiers, hors du dépôt. Voir docs/deploiement.md.

Exemples :
    powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1
    powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1 -Test
    powershell -ExecutionPolicy Bypass -File deploiement\lancer-bersemon.ps1 -Test -Public
#>
param(
    [switch]$Test,
    [switch]$Public
)

$racine = Split-Path -Parent $PSScriptRoot
$reglages = Join-Path $env:APPDATA 'Bersemon'

if ($Test) {
    $fichierJeton = Join-Path $reglages 'jeton-test.txt'
    $parties = Join-Path $reglages 'discord-test'
    $fichierServeur = Join-Path $reglages 'serveur-de-test.txt'
} else {
    $fichierJeton = Join-Path $reglages 'jeton-production.txt'
    $parties = Join-Path $reglages 'discord'
    $fichierServeur = $null
}

if (-not (Test-Path $fichierJeton)) {
    Write-Host "Jeton introuvable : enregistrez le jeton du bot dans $fichierJeton (voir docs/deploiement.md)."
    exit 1
}

# Le jeton ne passe que par la variable d'environnement de ce processus.
$env:DISCORD_TOKEN = (Get-Content -Raw $fichierJeton).Trim()
$env:BERSEMON_DONNEES = $parties
if ($fichierServeur -and (Test-Path $fichierServeur)) {
    $env:BERSEMON_SERVEUR_DE_TEST = (Get-Content -Raw $fichierServeur).Trim()
} else {
    # Sans serveur de test, les commandes sont globales, messages privés
    # compris : une variable héritée du terminal ne doit pas s'en mêler.
    Remove-Item Env:BERSEMON_SERVEUR_DE_TEST -ErrorAction SilentlyContinue
}
if ($Public) {
    $env:BERSEMON_MESSAGES_PUBLICS = '1'
} else {
    Remove-Item Env:BERSEMON_MESSAGES_PUBLICS -ErrorAction SilentlyContinue
}

Push-Location $racine
try {
    cargo build --release -p bersemon-discord
    if ($LASTEXITCODE -ne 0) {
        Write-Host "La compilation du bot a échoué : il n'est pas lancé."
        exit 1
    }
    & (Join-Path $racine 'target\release\bersemon-discord.exe')
} finally {
    Pop-Location
}
