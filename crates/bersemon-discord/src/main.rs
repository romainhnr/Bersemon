//! Bot Discord de Bersemon. La partie suit le déroulement du noyau
//! (`bersemon_core::deroulement`), comme dans la console : `/bersemon` ouvre
//! un message qui pose la question en cours, et chaque bouton y répond. La
//! mise en route est décrite dans docs/discord.md.

mod commandes;
mod composants;
mod configuration;
mod ecran_accueil;
mod ecran_camp;
mod ecran_combat;
mod ecran_expedition;
mod formulaires;
mod interactions;
mod joueurs;
mod message;
#[cfg(test)]
mod simulation;

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::contenu::ContenuV1;
use poise::serenity_prelude as serenity;

use crate::configuration::Configuration;
use crate::joueurs::Joueurs;

/// Partagé par toutes les commandes et les interactions.
pub struct DonneesDuBot {
    pub contenu: ContenuV1,
    pub joueurs: Mutex<Joueurs>,
}

pub type Erreur = Box<dyn std::error::Error + Send + Sync>;
pub type ContexteDeCommande<'a> = poise::Context<'a, DonneesDuBot, Erreur>;

#[tokio::main]
async fn main() {
    let configuration = match Configuration::depuis_l_environnement() {
        Ok(configuration) => configuration,
        Err(erreur) => {
            eprintln!("Configuration du bot incomplète : {erreur}.");
            std::process::exit(1);
        }
    };
    let serveur_de_test = configuration.serveur_de_test.map(serenity::GuildId::from);
    let donnees = DonneesDuBot {
        contenu: ContenuV1::new(),
        joueurs: Mutex::new(Joueurs::new(
            configuration.dossier_des_parties.clone(),
            graine_depuis_horloge(),
        )),
    };
    println!(
        "Parties des joueurs : {}",
        configuration.dossier_des_parties.display()
    );

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commandes::bersemon()],
            event_handler: |cadre, evenement| Box::pin(traiter_evenement(cadre, evenement)),
            ..Default::default()
        })
        .setup(move |contexte, _pret, framework| {
            Box::pin(async move {
                // Dans un serveur de test, les commandes apparaissent tout de
                // suite ; l'enregistrement global, valable partout et en
                // message privé, peut prendre jusqu'à une heure.
                let commandes = &framework.options().commands;
                match serveur_de_test {
                    Some(serveur) => {
                        poise::builtins::register_in_guild(contexte, commandes, serveur).await?;
                    }
                    None => poise::builtins::register_globally(contexte, commandes).await?,
                }
                println!("Bersemon est connecté à Discord.");
                Ok(donnees)
            })
        })
        .build();

    // Les commandes slash et les boutons n'ont besoin d'aucun intent
    // privilégié, comme le contenu des messages : rien n'est à demander à
    // Discord.
    let intents = serenity::GatewayIntents::non_privileged();
    let client = serenity::ClientBuilder::new(&configuration.jeton, intents)
        .framework(framework)
        .await;
    let resultat = match client {
        Ok(mut client) => client.start().await,
        Err(erreur) => Err(erreur),
    };
    if let Err(erreur) = resultat {
        eprintln!("Le bot s'est arrêté sur une erreur : {erreur}.");
        std::process::exit(1);
    }
}

/// Les commandes slash passent par poise ; les boutons et les formulaires,
/// par ici. Un échec de réponse est signalé sans arrêter le bot.
async fn traiter_evenement(
    cadre: poise::FrameworkContext<'_, DonneesDuBot, Erreur>,
    evenement: &serenity::FullEvent,
) -> Result<(), Erreur> {
    let serenity::FullEvent::InteractionCreate { interaction } = evenement else {
        return Ok(());
    };
    let resultat = match interaction {
        serenity::Interaction::Component(composant) => {
            interactions::composant(cadre.serenity_context, cadre.user_data, composant).await
        }
        serenity::Interaction::Modal(formulaire) => {
            interactions::formulaire(cadre.serenity_context, cadre.user_data, formulaire).await
        }
        _ => Ok(()),
    };
    if let Err(erreur) = resultat {
        eprintln!("Réponse impossible à une interaction : {erreur}.");
    }
    Ok(())
}

/// Une graine différente à chaque lancement suffit : le hasard d'un jeu n'a
/// pas besoin d'être imprévisible au sens cryptographique.
fn graine_depuis_horloge() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duree| duree.as_nanos() as u64)
        .unwrap_or(0)
}
