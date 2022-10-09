use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::items::item::Item;
use crate::models::consumable::Consumable;

#[derive(Serialize, Deserialize)]
pub struct Alchemy {
    pub consumables: Vec<Consumable>,
}

impl Item for Alchemy {
    fn new() -> Self {
        Self {
            consumables: Vec::new(),
        }
    }

    fn load(&mut self) -> &mut Self {
        self.load_from_page("https://genshin.gg/alchemy/")
    }

    fn load_from_page(&mut self, _url: &str) -> &mut Self {
        self
    }

    fn export(&self) -> Value {
        serde_json::to_value(self.consumables.clone()).unwrap_or(Value::Null)
    }

    fn clear(&mut self) -> &mut Self {
        self.consumables = Vec::new();
        self
    }
}
