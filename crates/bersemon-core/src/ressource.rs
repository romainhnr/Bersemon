//! Ressources lâchées par les monstres sauvages battus (section 14.3) :
//! une par espèce, que Malvoise rachète en attendant la fabrication et les
//! améliorations.

/// Identifiant stable d'une ressource, enregistré dans la sauvegarde.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RessourceId(String);

impl RessourceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ressource {
    id: RessourceId,
    nom: String,
    /// Ce que Malvoise en donne, en pièces d'argent (section 14.5).
    prix: u32,
}

impl Ressource {
    pub fn new(id: RessourceId, nom: impl Into<String>, prix: u32) -> Self {
        Self {
            id,
            nom: nom.into(),
            prix,
        }
    }

    pub fn id(&self) -> &RessourceId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn prix(&self) -> u32 {
        self.prix
    }
}
