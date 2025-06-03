use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EnvironmentConfig {
    pub num_agents: usize,
    pub length: usize,
    pub width: usize,
    pub interaction_radius: f64,
    pub interaction_probability: f64,
    pub max_movement: f64,
    pub tax_rate: f64,
    pub mean_age: f64,
    pub stddev_age: f64,
    pub mid_age: f64,
    pub max_start_age: f64,
    pub steepness: f64,
}

impl EnvironmentConfig {
    pub fn new(
        num_agents: usize,
        length: usize,
        width: usize,
        interaction_radius: f64,
        interaction_probability: f64,
        max_movement: f64,
        tax_rate: f64,
        mean_age: f64,
        stddev_age: f64,
        mid_age: f64,
        max_start_age: f64,
        steepness: f64,
    ) -> Self {
        Self {
            num_agents,
            length,
            width,
            interaction_radius,
            interaction_probability,
            max_movement,
            tax_rate,
            mean_age,
            stddev_age,
            mid_age,
            max_start_age,
            steepness,
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
