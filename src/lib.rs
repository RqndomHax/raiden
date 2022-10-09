use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod items;
pub mod models;

use items::{alchemy::Alchemy, item::Item};

#[derive(Serialize, Deserialize)]
pub struct Scrapper {
    pub alchemy: Alchemy,
}

impl Default for Scrapper {
    fn default() -> Self {
        Self {
            alchemy: Alchemy::new(),
        }
    }
}

impl Scrapper {
    /// Each call of load_items will override the previous elements.
    pub fn load_items(&mut self) -> &mut Self {
        self.alchemy.load();
        self
    }

    /// Export the loaded items to a json format
    pub fn export_items(&self) -> serde_json::Value {
        serde_json::to_value(<&Scrapper>::clone(&self)).unwrap_or(Value::Null)
    }
}
