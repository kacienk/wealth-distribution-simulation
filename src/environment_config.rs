use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub length: usize,
    pub width: usize,
    pub interaction_radius: f64,
    pub interaction_probability: f64,
    pub max_movement: f64,
    pub tax_rate: f64,
}

impl EnvironmentConfig {
    pub fn new(
        length: usize,
        width: usize,
        interaction_radius: f64,
        interaction_probability: f64,
        max_movement: f64,
        tax_rate: f64,
    ) -> Self {
        Self {
            length,
            width,
            interaction_radius,
            interaction_probability,
            max_movement,
            tax_rate,
        }
    }
    pub fn load_from_file(path: &str) -> Self {
        let mut file = File::open(path).expect("Failed to open config file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read config");
        serde_json::from_str(&content).expect("Failed to deserialize config")
    }

    pub fn save_to_file(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        let mut file = File::create(path).expect("Failed to create config file");
        file.write_all(json.as_bytes())
            .expect("Failed to write config");
    }
}
