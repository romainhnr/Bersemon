//! Bot Discord de Bersemon. À cette étape, un squelette : la connexion et une
//! commande `/bersemon` qui montre que le bot fonctionne. Le jeu y viendra
//! par le déroulement du noyau (`bersemon_core::deroulement`), comme pour la
//! console. La mise en route est décrite dans docs/discord.md.

mod commandes;
mod configuration;

use bersemon_core::contenu::ContenuV1;
use poise::serenity_prelude as serenity;

use crate::configuration::Configuration;

/// Partagé par toutes les commandes.
pub struct DonneesDuBot {
    pub contenu: ContenuV1,
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
    let contenu = ContenuV1::new();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commandes::bersemon()],
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
                Ok(DonneesDuBot { contenu })
            })
        })
        .build();

    // Les commandes slash n'ont besoin d'aucun intent privilégié, comme le
    // contenu des messages : rien n'est à demander à Discord.
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
