use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Consumable {
    pub image: String,
    pub name: String,
    pub rarity: u8,
    pub bonus_description: String,
    pub ingredients: Vec<String>,
}
