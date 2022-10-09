use serde_json::Value;

pub trait Item {
    /// It creates a new instance of the desired item.
    fn new() -> Self;

    /// Load the default webpage and store it.
    /// Each call of the load function will clear the old data and override it.
    fn load(&mut self) -> &mut Self;

    /// Load `url` and store it.
    /// Each call of the load function will clear the old data and override it.
    fn load_from_page(&mut self, url: &str) -> &mut Self;

    /// It exports the item in json format.
    fn export(&self) -> Value;

    /// Removes all existing data from the item.
    fn clear(&mut self) -> &mut Self;
}
