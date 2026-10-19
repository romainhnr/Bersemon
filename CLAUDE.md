# Bersemon

Jeu RPG textuel au tour par tour, écrit en Rust, qui mêle un univers dark fantasy (inspiration Berserk, Dark Souls) et la capture de monstres (inspiration Pokémon). Bersemon est le nom du projet et du jeu.

## Document de référence

- `docs/gamedesign.md` est la référence fonctionnelle du projet. Lis-le avant toute tâche qui touche aux règles du jeu.
- Toute décision de game design prise pendant une session est reportée dans `docs/gamedesign.md`, dans la même tâche : corps du document, section des questions ouvertes et journal des décisions.
- Si une demande contredit ce document ou s'il ne couvre pas un cas nécessaire au code, signale-le et pose la question avant de coder. N'invente pas de règle métier.

## Architecture

- Un noyau métier en Rust pur, indépendant de toute interface : ses règles ne dépendent ni de la console, ni de Discord, ni du stockage.
- Une interface console, puis plus tard un bot Discord, qui utilisent ce noyau.
- La structure du workspace Cargo est définie au démarrage de la V1 ; explique le raisonnement de chaque choix structurant.

## Environnement

- Deux postes Windows : Windows 11 avec VSCodium et Windows 10 avec VS Code, extension rust-analyzer sur les deux.
- Sur le poste Windows 10, Rust est installé par l'installateur MSI, sans rustup, derrière un proxy d'entreprise. N'utilise rien qui exige rustup, une version nightly ou une version de Rust plus récente que celle disponible sur ce poste.
- Commandes : `cargo build`, `cargo test`, `cargo fmt`, `cargo clippy`.
- Avant de considérer une tâche comme terminée : `cargo fmt`, puis `cargo clippy` sans avertissement, puis `cargo test` au vert, lorsque ces outils sont disponibles sur le poste.
- Les sessions cloud tournent sous Linux : le code doit compiler et passer ses tests sous Windows comme sous Linux. Ne code rien de propre à un seul système sans le signaler.

## Posture

- Réponds en français, de façon directe, concise et sobre, sans formules de politesse ni compliments.
- Je peux me tromper : remets en question mes raisonnements et signale leurs faiblesses. Si une demande ou une approche te semble mauvaise, dis-le avant de l'exécuter et explique pourquoi.
- Quand plusieurs solutions sont valables, présente les principales options avec leurs compromis et indique celle que tu recommandes.
- N'invente rien et ne devine pas. S'il manque une information, demande-la ; si tu dois faire une hypothèse, signale-la explicitement.

## Fiabilité dans le code

- N'utilise pas d'API, de méthode, de paramètre, de crate ou d'option de configuration dont tu n'es pas certain qu'il existe dans la version utilisée.
- Distingue le code que tu as compilé ou testé de celui que tu n'as pas pu vérifier.
- Lorsqu'un choix implique des dimensions architecturales ou métier, explique le raisonnement qui le justifie.

## Code

- Le code technique est en anglais. Les concepts métier (entités, value objects, règles, termes du domaine) sont nommés en français, combinés avec les suffixes techniques usuels en anglais. Exemples : `Monstre`, `ValeursInternes`, `Habilete`, `SauvegardeRepository`, `calculer_chance_capture`.
- Applique les conventions Rust : types et traits en `PascalCase`, fonctions, variables et modules en `snake_case`, constantes en `SCREAMING_SNAKE_CASE`.
- Utilise des noms parlants et immédiatement compréhensibles pour un développeur français. Évite les termes rares, ambigus ou traduits littéralement.
- Les accents sont interdits dans les identifiants (`Habilete` et non `Habileté`).
- Commente en français. Un commentaire explique le pourquoi (intention, contrainte, règle métier, choix non évident), pas ce que le code dit déjà. Pas de commentaire évident, pas de code mort en commentaire, pas de numéro de ticket.
- Dans le code, les commentaires, la configuration, les scripts et les commandes, n'utilise que des caractères saisissables au clavier standard : pas de tirets longs, de flèches, de points de suspension typographiques, de guillemets typographiques ou chevrons, de puces, d'espaces insécables ni de caractères invisibles. Les accents sont autorisés dans les commentaires et dans les textes affichés au joueur, rédigés en français correct.

## Modifications

- Limite tes modifications à ce qui est demandé. Si tu repères un autre problème (bug, incohérence, dette technique), signale-le sans le corriger d'office.
- N'ajoute pas de dépendance (crate) sans l'indiquer et la justifier, et attends mon accord.
- Respecte le style des fichiers existants : indentation, encodage, fins de ligne et conventions en place.
- En session locale, ne fais pas de commit git sans que je le demande.
- En session cloud, travaille sur une branche dédiée, jamais sur main : fais des commits réguliers et pousse la branche, pour que le travail ne soit pas perdu si la session s'arrête. Ne fusionne jamais toi-même dans main.
- En session cloud, je ne suis souvent pas là pour répondre : quand une règle du game design manque ou qu'un choix me revient, ne l'invente pas. Note la question dans la section des questions ouvertes de `docs/gamedesign.md`, avance sur le reste de la tâche et signale la question dans ton compte rendu final.
- À la fin d'une tâche, liste les fichiers créés, modifiés, supprimés ou renommés, et précise ce qui a été compilé et testé.
