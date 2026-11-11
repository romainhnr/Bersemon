//! Don et échange de monstres et de ressources entre deux joueurs (section
//! 15.1). Une offre garde une copie des monstres qu'elle désigne : elle
//! n'est plus valable si l'un d'eux a changé ou changé de place avant
//! l'acceptation, ni si les ressources qu'elle désigne ne sont plus là.

use std::fmt;

use crate::collection::{Emplacement, Placement};
use crate::espece::Rarete;
use crate::monstre::Monstre;
use crate::partie::Partie;
use crate::ressource::Ressource;

/// Un monstre désigné par une offre, à sa place au moment de l'offre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonstreOffert {
    pub emplacement: Emplacement,
    pub monstre: Monstre,
}

/// Ce qu'une offre fait passer d'un joueur à l'autre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cession {
    /// En boîte : un monstre pèse bien plus que des ressources.
    Monstre(Box<MonstreOffert>),
    /// Prises du sac d'abord, puis du coffre, comme un coût (section 13.3),
    /// et reçues au coffre du camp, qui n'a pas de limite.
    Ressources(Ressource, u32),
}

/// Ce que cède celui qui fait l'offre, et, pour un échange, ce qu'il
/// demande en retour ; sans contrepartie, c'est un don.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offre {
    pub cede: Cession,
    pub demande: Option<Cession>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurEchange {
    MonstreInexistant(Emplacement),
    /// Un Apôtre reste à celui qui l'a vaincu.
    Apotre,
    /// Un monstre loué reste à Malvoise (section 14.2).
    MonstreLoue,
    /// L'offre laisserait un des joueurs sans monstre, en comptant ceux
    /// partis en expédition automatique.
    DernierMonstre,
    /// Ni l'équipe ni le camp de celui qui reçoit un monstre sans en céder
    /// un n'ont de place.
    PlusDePlace,
    /// Une offre de ressources en compte au moins une.
    AucuneRessource,
    /// Le sac et le coffre réunis n'en ont pas autant.
    RessourcesManquantes,
    /// Un des monstres a changé ou changé de place depuis l'offre, ou les
    /// ressources ne sont plus là.
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
                write!(f, "chaque joueur doit garder au moins un monstre")
            }
            ErreurEchange::PlusDePlace => write!(f, "l'équipe et le camp sont pleins"),
            ErreurEchange::AucuneRessource => write!(f, "aucune ressource offerte"),
            ErreurEchange::RessourcesManquantes => write!(f, "ressources manquantes"),
            ErreurEchange::OffrePerimee => {
                write!(
                    f,
                    "un des monstres ou des ressources a changé depuis l'offre"
                )
            }
            ErreurEchange::DonneurPasAuCamp => write!(f, "le donneur n'est pas au camp"),
            ErreurEchange::ReceveurPasAuCamp => write!(f, "le receveur n'est pas au camp"),
        }
    }
}

impl std::error::Error for ErreurEchange {}

/// Où est arrivé chaque monstre : chez celui qui reçoit l'offre s'il en
/// reçoit un, et chez celui qui l'a faite s'il en reçoit un.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conclusion {
    pub chez_le_receveur: Option<Emplacement>,
    pub chez_le_donneur: Option<Emplacement>,
}

impl Cession {
    /// Le monstre de cette partie à cet emplacement.
    pub fn monstre(partie: &Partie, emplacement: Emplacement) -> Result<Cession, ErreurEchange> {
        offert(partie, emplacement).map(|offert| Cession::Monstre(Box::new(offert)))
    }

    /// Ce nombre de ressources de cette sorte, du sac et du coffre de cette
    /// partie réunis.
    pub fn ressources(
        partie: &Partie,
        ressource: Ressource,
        nombre: u32,
    ) -> Result<Cession, ErreurEchange> {
        if nombre == 0 {
            return Err(ErreurEchange::AucuneRessource);
        }
        if !partie
            .ressources_manquantes(&[(ressource.clone(), nombre)])
            .is_empty()
        {
            return Err(ErreurEchange::RessourcesManquantes);
        }
        Ok(Cession::Ressources(ressource, nombre))
    }

    fn est_un_monstre(&self) -> bool {
        matches!(self, Cession::Monstre(_))
    }
}

impl Offre {
    /// `cede`, pris à `donneur`, contre `demande`, pris à `receveur` ; sans
    /// contrepartie, c'est un don. Refuse ce que refuserait la conclusion.
    pub fn new(
        donneur: &Partie,
        cede: Cession,
        receveur: &Partie,
        demande: Option<Cession>,
    ) -> Result<Offre, ErreurEchange> {
        let offre = Offre { cede, demande };
        verifier_offre(donneur, receveur, &offre)?;
        Ok(offre)
    }

    /// Le monstre du donneur à cet emplacement, sans contrepartie.
    pub fn don(
        donneur: &Partie,
        emplacement: Emplacement,
        receveur: &Partie,
    ) -> Result<Offre, ErreurEchange> {
        Offre::new(
            donneur,
            Cession::monstre(donneur, emplacement)?,
            receveur,
            None,
        )
    }

    /// Le monstre du donneur à cet emplacement, contre celui du receveur à
    /// `demande`.
    pub fn echange(
        donneur: &Partie,
        emplacement: Emplacement,
        receveur: &Partie,
        demande: Emplacement,
    ) -> Result<Offre, ErreurEchange> {
        Offre::new(
            donneur,
            Cession::monstre(donneur, emplacement)?,
            receveur,
            Some(Cession::monstre(receveur, demande)?),
        )
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

/// Un monstre échangé contre un monstre prend la place de celui qu'il
/// remplace ; un monstre cédé sans monstre en retour va dans l'équipe s'il
/// y reste une place, sinon au camp. Le monstre reçu porte le nom du héros
/// qui l'a cédé, et son espèce passe capturée au codex. Ne change rien si
/// l'offre n'est plus valable.
pub(crate) fn conclure_offre(
    donneur: &mut Partie,
    receveur: &mut Partie,
    offre: &Offre,
) -> Result<Conclusion, ErreurEchange> {
    verifier_offre(donneur, receveur, offre)?;
    if let (Cession::Monstre(cede), Some(Cession::Monstre(demande))) = (&offre.cede, &offre.demande)
    {
        echanger_les_monstres(donneur, cede, receveur, demande)?;
        return Ok(Conclusion {
            chez_le_receveur: Some(demande.emplacement),
            chez_le_donneur: Some(cede.emplacement),
        });
    }
    let chez_le_receveur = transferer(donneur, &offre.cede, receveur)?;
    let chez_le_donneur = match &offre.demande {
        Some(demande) => transferer(receveur, demande, donneur)?,
        None => None,
    };
    Ok(Conclusion {
        chez_le_receveur,
        chez_le_donneur,
    })
}

/// Chaque monstre prend la place de l'autre : le nombre de monstres de
/// chacun ne change pas.
fn echanger_les_monstres(
    donneur: &mut Partie,
    cede: &MonstreOffert,
    receveur: &mut Partie,
    demande: &MonstreOffert,
) -> Result<(), ErreurEchange> {
    let nom_du_donneur = donneur.heros().nom().clone();
    let nom_du_receveur = receveur.heros().nom().clone();
    let pour_le_receveur = receveur.preparer_monstre_recu(cede.monstre.clone(), &nom_du_donneur);
    let pour_le_donneur = donneur.preparer_monstre_recu(demande.monstre.clone(), &nom_du_receveur);
    receveur
        .collection_mut()
        .relacher_pour_capture(demande.emplacement, pour_le_receveur)
        .map_err(|_| ErreurEchange::OffrePerimee)?;
    donneur
        .collection_mut()
        .relacher_pour_capture(cede.emplacement, pour_le_donneur)
        .map_err(|_| ErreurEchange::OffrePerimee)?;
    Ok(())
}

/// Fait passer la cession de `de` à `vers`, sans rien en retour. Rend la
/// place du monstre reçu, s'il s'agit d'un monstre.
fn transferer(
    de: &mut Partie,
    cession: &Cession,
    vers: &mut Partie,
) -> Result<Option<Emplacement>, ErreurEchange> {
    match cession {
        Cession::Monstre(offert) => {
            let nom = de.heros().nom().clone();
            let monstre = de
                .collection_mut()
                .relacher(offert.emplacement)
                .map_err(|_| ErreurEchange::OffrePerimee)?;
            let monstre = vers.preparer_monstre_recu(monstre, &nom);
            match vers.collection_mut().accueillir_capture(monstre) {
                Placement::Equipe(index) => Ok(Some(Emplacement::Equipe(index))),
                Placement::Camp(index) => Ok(Some(Emplacement::Camp(index))),
                Placement::PlaceManquante(_) => unreachable!("place vérifiée par verifier_offre"),
            }
        }
        Cession::Ressources(ressource, nombre) => {
            de.payer_ressources(&[(ressource.clone(), *nombre)]);
            vers.recevoir_ressources(ressource.clone(), *nombre);
            Ok(None)
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
/// expédition, et le camp les soigne. Les ressources peuvent avoir bougé
/// entre le sac et le coffre : seul leur nombre compte.
fn toujours_la(partie: &Partie, cession: &Cession) -> Result<(), ErreurEchange> {
    let soigne = |monstre: &Monstre| {
        let mut soigne = monstre.clone();
        soigne.soigner_completement();
        soigne
    };
    let present = match cession {
        Cession::Monstre(offert) => partie
            .collection()
            .monstre(offert.emplacement)
            .is_some_and(|monstre| soigne(monstre) == soigne(&offert.monstre)),
        Cession::Ressources(ressource, nombre) => partie
            .ressources_manquantes(&[(ressource.clone(), *nombre)])
            .is_empty(),
    };
    if present {
        Ok(())
    } else {
        Err(ErreurEchange::OffrePerimee)
    }
}

/// Un monstre contre un monstre garde le nombre de monstres de chacun ; un
/// monstre sans monstre en retour en retire un à celui qui le cède, qui
/// doit en garder au moins un, et en ajoute un à l'autre, qui doit avoir de
/// la place.
fn verifier_les_places(
    donneur: &Partie,
    receveur: &Partie,
    offre: &Offre,
) -> Result<(), ErreurEchange> {
    let demande_un_monstre = offre.demande.as_ref().is_some_and(Cession::est_un_monstre);
    if offre.cede.est_un_monstre() == demande_un_monstre {
        return Ok(());
    }
    let (cedant, recevant) = if offre.cede.est_un_monstre() {
        (donneur, receveur)
    } else {
        (receveur, donneur)
    };
    if cedant.collection().nombre_de_monstres_a_soi() <= 1 {
        return Err(ErreurEchange::DernierMonstre);
    }
    if !recevant.collection().a_de_la_place() {
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
    use crate::inventaire::Objet;
    use crate::niveau::Niveau;
    use crate::nom::Nom;
    use crate::ressource::RessourceId;
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
                chez_le_receveur: Some(Emplacement::Equipe(1)),
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

    fn fragments(contenu: &ContenuV1) -> Ressource {
        contenu
            .ressource(&RessourceId::new("fragment-d-armure"))
            .unwrap()
    }

    fn nombre(partie: &Partie, ressource: &Ressource) -> (u32, u32) {
        (
            partie.sac().nombre_de_ressources(ressource.id()),
            partie.coffre_du_camp().nombre_de_ressources(ressource.id()),
        )
    }

    #[test]
    fn un_don_de_ressources_les_prend_au_sac_puis_au_coffre() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let fragments = fragments(&contenu);
        aldric.recevoir_ressources(fragments.clone(), 8);
        aldric
            .sortir_du_coffre(&Objet::Ressources(fragments.id().clone(), 3))
            .unwrap();
        assert_eq!(nombre(&aldric, &fragments), (3, 5));
        assert_eq!(
            Cession::ressources(&aldric, fragments.clone(), 9),
            Err(ErreurEchange::RessourcesManquantes)
        );
        assert_eq!(
            Cession::ressources(&aldric, fragments.clone(), 0),
            Err(ErreurEchange::AucuneRessource)
        );
        let cede = Cession::ressources(&aldric, fragments.clone(), 6).unwrap();
        let offre = Offre::new(&aldric, cede, &ysolde, None).unwrap();
        let conclusion = conclure_offre(&mut aldric, &mut ysolde, &offre).unwrap();
        assert_eq!(
            conclusion,
            Conclusion {
                chez_le_receveur: None,
                chez_le_donneur: None,
            }
        );
        assert_eq!(nombre(&aldric, &fragments), (0, 2));
        assert_eq!(nombre(&ysolde, &fragments), (0, 6));
    }

    #[test]
    fn un_monstre_s_echange_contre_des_ressources() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let fragments = fragments(&contenu);
        ysolde.recevoir_ressources(fragments.clone(), 10);
        let offre = Offre::new(
            &aldric,
            Cession::monstre(&aldric, Emplacement::Equipe(1)).unwrap(),
            &ysolde,
            Some(Cession::ressources(&ysolde, fragments.clone(), 10).unwrap()),
        )
        .unwrap();
        let conclusion = conclure_offre(&mut aldric, &mut ysolde, &offre).unwrap();
        assert_eq!(
            conclusion,
            Conclusion {
                chez_le_receveur: Some(Emplacement::Equipe(1)),
                chez_le_donneur: None,
            }
        );
        assert_eq!(aldric.collection().nombre_de_monstres(), 1);
        assert_eq!(ysolde.collection().nombre_de_monstres(), 2);
        assert_eq!(nombre(&aldric, &fragments), (0, 10));
        assert_eq!(nombre(&ysolde, &fragments), (0, 0));
    }

    #[test]
    fn des_ressources_contre_le_dernier_monstre_sont_refusees() {
        let contenu = ContenuV1::new();
        let (mut aldric, ysolde) = deux_parties(&contenu);
        let fragments = fragments(&contenu);
        aldric.recevoir_ressources(fragments.clone(), 4);
        let cede = Cession::ressources(&aldric, fragments, 4).unwrap();
        let demande = Cession::monstre(&ysolde, Emplacement::Equipe(0)).unwrap();
        assert_eq!(
            Offre::new(&aldric, cede, &ysolde, Some(demande)),
            Err(ErreurEchange::DernierMonstre)
        );
    }

    #[test]
    fn des_ressources_depensees_depuis_l_offre_la_periment() {
        let contenu = ContenuV1::new();
        let (mut aldric, mut ysolde) = deux_parties(&contenu);
        let fragments = fragments(&contenu);
        aldric.recevoir_ressources(fragments.clone(), 4);
        let cede = Cession::ressources(&aldric, fragments.clone(), 4).unwrap();
        let offre = Offre::new(&aldric, cede, &ysolde, None).unwrap();
        aldric.payer_ressources(&[(fragments, 1)]);
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
