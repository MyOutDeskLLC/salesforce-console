use std::sync::{Arc, Mutex};
use rustforce::Client;

pub struct AppStructure {
    pub sf_client: Client,
}


impl Default for AppStructure {
    fn default() -> Self {
        AppStructure {
            sf_client: Client::new(Some("".to_string()), Some("".to_string())),
        }
    }
}

pub struct AppState(pub Arc<Mutex<AppStructure>>);