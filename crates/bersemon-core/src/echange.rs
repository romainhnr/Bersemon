//! Don et échange de monstres entre deux joueurs (section 15.1). Une offre
//! garde une copie des monstres qu'elle désigne : elle n'est plus valable si
//! l'un d'eux a changé ou changé de place avant l'acceptation.

use std::fmt;

use crate::collection::{Emplacement, Placement};
use crate::espece::Rarete;
use crate::monstre::Monstre;
use crate::partie::Partie;

/// Un monstre désigné par une offre, à sa place au moment de l'offre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonstreOffert {
    pub emplacement: Emplacement,
    pub monstre: Monstre,
}

/// Le monstre que cède celui qui fait l'offre, et, pour un échange, celui
/// qu'il demande en retour ; sans contrepartie, c'est un don.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offre {
    pub cede: MonstreOffert,
    pub demande: Option<MonstreOffert>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurEchange {
    MonstreInexistant(Emplacement),
    /// Un Apôtre reste à celui qui l'a vaincu.
    Apotre,
    /// Un monstre loué reste à Malvoise (section 14.2).
    MonstreLoue,
    /// Un don laisserait le donneur sans monstre, en comptant ceux partis en
    /// expédition automatique.
    DernierMonstre,
    /// Ni l'équipe ni le camp de celui qui reçoit un don n'ont de place.
    PlusDePlace,
    /// Un des monstres a changé ou changé de place depuis l'offre.
    OffrePerimee,
    /// L'un des joueurs n'est pas au camp, ou une question l'y attend.
    DonneurPasAuCamp,
    ReceveurPasAuCamp,
}

impl fmt::Display for ErreurEchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurEchange::MonstreInexistant(emplacement) => {
                write!(f, "aucun monstre à l'emplacement {emplacement:?}")
            }
            ErreurEchange::Apotre => write!(f, "un Apôtre ne se donne ni ne s'échange"),
            ErreurEchange::MonstreLoue => {
                write!(f, "un monstre loué ne se donne ni ne s'échange")
            }
            ErreurEchange::DernierMonstre => {
                write!(f, "le joueur doit garder au moins un monstre")
            }
            ErreurEchange::PlusDePlace => write!(f, "l'équipe et le camp sont pleins"),
            ErreurEchange::OffrePerimee => write!(f, "un des monstres a changé depuis l'offre"),
            ErreurEchange::DonneurPasAuCamp => write!(f, "le donneur n'est pas au camp"),
            ErreurEchange::ReceveurPasAuCamp => write!(f, "le receveur n'est pas au camp"),
        }
    }
}

impl std::error::Error for ErreurEchange {}

/// Où est arrivé chaque monstre : chez celui qui reçoit l'offre, et, pour un
/// échange, chez celui qui l'a faite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conclusion {
    pub chez_le_receveur: Emplacement,
    pub chez_le_donneur: Option<Emplacement>,
}

impl Offre {
    /// Le monstre du donneur à cet emplacement, sans contrepartie.
    pub fn don(
        donneur: &Partie,
        emplacement: Emplacement,
        receveur: &Partie,
    ) -> Result<Offre, ErreurEchange> {
        let offre = Offre {
            cede: offert(donneur, emplacement)?,
            demande: None,
        };
        verifier_les_places(donneur, receveur, &offre)?;
        Ok(offre)
    }

    /// Le monstre du donneur à cet emplacement, contre celui du receveur à
    /// `demande`.
    pub fn echange(
        donneur: &Partie,
        emplacement: Emplacement,
        receveur: &Partie,
        demande: Emplacement,
    ) -> Result<Offre, ErreurEchange> {
        Ok(Offre {
            cede: offert(donneur, emplacement)?,
            demande: Some(offert(receveur, demande)?),
        })
    }

    pub fn est_un_don(&self) -> bool {
        self.demande.is_none()
    }
}

/// Rien n'a changé depuis l'offre, et elle peut encore se conclure.
pub fn verifier_offre(
    donneur: &Partie,
    receveur: &Partie,
    offre: &Offre,
) -> Result<(), ErreurEchange> {
    toujours_la(donneur, &offre.cede)?;
    if let Some(demande) = &offre.demande {
        toujours_la(receveur, demande)?;
    }
    verifier_les_places(donneur, receveur, offre)
}

/// Un échange met chaque monstre à la place de celui qu'il remplace ; un
/// don va dans l'équipe s'il y reste une place, sinon au camp. Le monstre
/// reçu porte le nom du héros qui l'a cédé, et son espèce passe capturée au
/// codex. Ne change rien si l'offre n'est plus valable.
pub(crate) fn conclure_offre(
    donneur: &mut Partie,
    receveur: &mut Partie,
    offre: &Offre,
) -> Result<Conclusion, ErreurEchange> {
    verifier_offre(donneur, receveur, offre)?;
    let nom_du_donneur = donneur.heros().nom().clone();
    let nom_du_receveur = receveur.heros().nom().clone();
    let emplacement_cede = offre.cede.emplacement;
    match &offre.demande {
        None => {
            let monstre = donneur
                .collection_mut()
                .relacher(emplacement_cede)
                .map_err(|_| ErreurEchange::OffrePerimee)?;
            let monstre = receveur.preparer_monstre_recu(monstre, &nom_du_donneur);
            let chez_le_receveur = match receveur.collection_mut().accueillir_capture(monstre) {
                Placement::Equipe(index) => Emplacement::Equipe(index),
                Placement::Camp(index) => Emplacement::Camp(index),
                Placement::PlaceManquante(_) => unreachable!("place vérifiée par verifier_offre"),
            };
            Ok(Conclusion {
                chez_le_receveur,
                chez_le_donneur: None,
            })
        }
        Some(demande) => {
            let pour_le_receveur =
                receveur.preparer_monstre_recu(offre.cede.monstre.clone(), &nom_du_donneur);
            let pour_le_donneur =
                donneur.preparer_monstre_recu(demande.monstre.clone(), &nom_du_receveur);
            receveur
                .collection_mut()
                .relacher_pour_capture(demande.emplacement, pour_le_receveur)
                .map_err(|_| ErreurEchange::OffrePerimee)?;
            donneur
                .collection_mut()
                .relacher_pour_capture(emplacement_cede, pour_le_donneur)
                .map_err(|_| ErreurEchange::OffrePerimee)?;
            Ok(Conclusion {
                chez_le_receveur: demande.emplacement,
                chez_le_donneur: Some(emplacement_cede),
            })
        }
    }
}

fn offert(partie: &Partie, emplacement: Emplacement) -> Result<MonstreOffert, ErreurEchange> {
    let monstre = partie
        .collection()
        .monstre(emplacement)
        .ok_or(ErreurEchange::MonstreInexistant(emplacement))?;
    if monstre.espece().rarete() == Rarete::Apotre {
        return Err(ErreurEchange::Apotre);
    }
    if monstre.est_loue() {
        return Err(ErreurEchange::MonstreLoue);
    }
    Ok(MonstreOffert {
        emplacement,
        monstre: monstre.clone(),
    })
}

/// Les PV et le statut ne comptent pas : ils changent pendant une
/// expédition, et le camp les soigne.
fn toujours_la(partie: &Partie, offert: &MonstreOffert) -> Result<(), ErreurEchange> {
    let soigne = |monstre: &Monstre| {
        let mut soigne = monstre.clone();
        soigne.soigner_completement();
        soigne
    };
    match partie.collection().monstre(offert.emplacement) {
        Some(monstre) if soigne(monstre) == soigne(&offert.monstre) => Ok(()),
        _ => Err(ErreurEchange::OffrePerimee),
    }
}

/// Un échange garde le nombre de monstres de chacun ; un don retire un
/// monstre au donneur et en ajoute un au receveur.
fn verifier_les_places(
    donneur: &Partie,
    receveur: &Partie,
    offre: &Offre,
) -> Result<(), ErreurEchange> {
    if !offre.est_un_don() {
        return Ok(());
    }
    if donneur.collection().nombre_de_monstres_a_soi() <= 1 {
        return Err(ErreurEchange::DernierMonstre);
    }
    if !receveur.collection().a_de_la_place() {
        return Err(ErreurEchange::PlusDePlace);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogue::Catalogue;
    use crate::codex::EtatCodex;
    use crate::contenu::ContenuV1;
    use crate::espece::EspeceId;
    use crate::heros::{Classe, Heros};
    use crate::niveau::Niveau;
    use crate::nom::Nom;
    use crate::stats::ValeursInternes;

    fn creer(contenu: &ContenuV1, id: &str, niveau: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        Monstre::creer(
            espece,
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Aldric a une Salamandre et un Loup, Ysolde une Mandragore.
    fn deux_parties(contenu: &ContenuV1) -> (Partie, Partie) {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut aldric = Partie::nouvelle(heros, creer(contenu, "salamandre", 5));
        aldric
            .collection_mut()
            .accueillir_capture(creer(contenu, "loup", 7));
        let heros = Heros::new(Nom::new("Ysolde").unwrap(), Classe::Templier);
        let ysolde = Partie::nouvelle(heros, creer(contenu, "mandragore", 5));
        (aldric, ysolde)
    }

    fn nom_du_donneur(monstre: &Monstre) -> Option<&str> {
        monstre.recu_de().map(Nom::as_str)
    }

    #[test]
    fn un_don_rejoint_l_equipe_marque_du_donneur() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let offre = Offre::don(&aldric, Emplacement::Equipe(1), &ysolde).unwrap();
        assert!(offre.est_un_don());
        let conclusion = conclure_offre(&mut aldric, &mut ysolde, &offre).unwrap();
        assert_eq!(
            conclusion,
            Conclusion {
                chez_le_receveur: Emplacement::Equipe(1),
                chez_le_donneur: None,
            }
        );
        assert_eq!(aldric.collection().equipe().len(), 1);
        let loup = &ysolde.collection().equipe()[1];
        assert_eq!(loup.espece().id().as_str(), "loup");
        assert_eq!(loup.niveau().valeur(), 7);
        assert_eq!(nom_du_donneur(loup), Some("Aldric"));
        assert_eq!(
            ysolde.codex().etat(&EspeceId::new("loup")),
            EtatCodex::Capturee
        );
    }

    #[test]
    fn un_echange_met_chaque_monstre_a_la_place_de_l_autre() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let offre = Offre::echange(
            &aldric,
            Emplacement::Equipe(0),
            &ysolde,
            Emplacement::Equipe(0),
        )
        .unwrap();
        conclure_offre(&mut aldric, &mut ysolde, &offre).unwrap();
        let mandragore = &aldric.collection().equipe()[0];
        assert_eq!(mandragore.espece().id().as_str(), "mandragore");
        assert_eq!(nom_du_donneur(mandragore), Some("Ysolde"));
        let salamandre = &ysolde.collection().equipe()[0];
        assert_eq!(salamandre.espece().id().as_str(), "salamandre");
        assert_eq!(nom_du_donneur(salamandre), Some("Aldric"));
        assert_eq!(
            aldric.codex().etat(&EspeceId::new("mandragore")),
            EtatCodex::Capturee
        );
        assert_eq!(
            ysolde.codex().etat(&EspeceId::new("salamandre")),
            EtatCodex::Capturee
        );
        // Chacun garde son nombre de monstres.
        assert_eq!(aldric.collection().nombre_de_monstres(), 2);
        assert_eq!(ysolde.collection().nombre_de_monstres(), 1);
    }

    #[test]
    fn ni_le_dernier_monstre_ni_un_apotre_ne_se_donnent() {
        let contenu = ContenuV1::new();
        let (mut aldric, ysolde) = deux_parties(&contenu);
        assert_eq!(
            Offre::don(&ysolde, Emplacement::Equipe(0), &aldric),
            Err(ErreurEchange::DernierMonstre)
        );
        aldric
            .collection_mut()
            .accueillir_capture(creer(&contenu, "baron-pendu", 20));
        assert_eq!(
            Offre::don(&aldric, Emplacement::Equipe(2), &ysolde),
            Err(ErreurEchange::Apotre)
        );
        assert_eq!(
            Offre::echange(
                &ysolde,
                Emplacement::Equipe(0),
                &aldric,
                Emplacement::Equipe(2)
            ),
            Err(ErreurEchange::Apotre)
        );
        assert_eq!(
            Offre::don(&aldric, Emplacement::Camp(0), &ysolde),
            Err(ErreurEchange::MonstreInexistant(Emplacement::Camp(0)))
        );
    }

    #[test]
    fn un_don_sans_place_est_refuse() {
        let contenu = ContenuV1::new();
        let (aldric, mut ysolde) = deux_parties(&contenu);
        while ysolde.collection().a_de_la_place() {
            ysolde
                .collection_mut()
                .accueillir_capture(creer(&contenu, "loup", 3));
        }
        assert_eq!(
            Offre::don(&aldric, Emplacement::Equipe(1), &ysolde),
            Err(ErreurEchange::PlusDePlace)
        );
        // Un échange garde le nombre de monstres : il reste possible.
        assert!(
            Offre::echange(
                &aldric,
                Emplacement::Equipe(1),
                &ysolde,
                Emplacement::Camp(0)
            )
            .is_ok()
        );
    }

    #[test]
    fn une_offre_perimee_ne_change_rien() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let offre = Offre::don(&aldric, Emplacement::Equipe(1), &ysolde).unwrap();
        aldric
            .collection_mut()
            .deplacer_dans_l_equipe(1, 0)
            .unwrap();
        let (avant_aldric, avant_ysolde) = (aldric.clone(), ysolde.clone());
        assert_eq!(
            conclure_offre(&mut aldric, &mut ysolde, &offre),
            Err(ErreurEchange::OffrePerimee)
        );
        assert_eq!((aldric, ysolde), (avant_aldric, avant_ysolde));
    }

    #[test]
    fn les_pv_perdus_depuis_l_offre_ne_la_periment_pas() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let mut blesse = aldric.clone();
        blesse
            .collection_mut()
            .membre_de_l_equipe_mut(1)
            .unwrap()
            .subir_degats(5);
        let offre = Offre::don(&blesse, Emplacement::Equipe(1), &ysolde).unwrap();
        assert_eq!(verifier_offre(&aldric, &ysolde, &offre), Ok(()));
        assert!(conclure_offre(&mut aldric, &mut ysolde, &offre).is_ok());
    }
}
