use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AgeAndDeath {
    pub mean_age: f64,
    pub stddev_age: f64,
    pub mid_age: f64, // age where death chance is 50%
    pub max_start_age: f64,
    pub steepness: f64, // how quickly death probability rises with age
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Education {
    pub initial_adult_min: f64,
    pub initial_adult_max: f64,
    pub elemental_education_threshold: f64,
    pub children_education_jitter: f64,
    pub learning_rate_min: f64,
    pub learning_rate_max: f64,
    pub max: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct IncomeAndConsumption {
    pub income_age_parameter: f64,
    pub income_education_parameter: f64,
    pub base_consumption: f64,
    pub aditional_consumption_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Transaction {
    pub transaction_probability: f64,
    pub education_parameter: f64,
    pub age_parameter: f64,
    pub tax_rate: f64,
    pub amount_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Wealth {
    pub min_initial_wealth: f64,
    pub max_initial_wealth: f64,
    pub min_inheritance_at_birth_rate: f64,
    pub max_inheritance_at_birth_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EnvironmentConfig {
    pub num_agents: usize,
    pub length: usize,
    pub width: usize,
    pub interaction_radius: f64,
    pub max_movement: f64,
    pub age_and_death: AgeAndDeath,
    pub education: Education,
    pub income_and_consumption: IncomeAndConsumption,
    pub transaction: Transaction,
    pub wealth: Wealth,
}

impl AgeAndDeath {
    pub fn new(
        mean_age: f64,
        stddev_age: f64,
        mid_age: f64,
        max_start_age: f64,
        steepness: f64,
    ) -> Self {
        Self {
            mean_age,
            stddev_age,
            mid_age,
            max_start_age,
            steepness,
        }
    }
}

impl Education {
    pub fn new(
        initial_adult_min: f64,
        initial_adult_max: f64,
        elemental_education_threshold: f64,
        children_education_jitter: f64,
        learning_rate_min: f64,
        learning_rate_max: f64,
        max: f64,
    ) -> Self {
        Self {
            initial_adult_min,
            initial_adult_max,
            elemental_education_threshold,
            children_education_jitter,
            learning_rate_min,
            learning_rate_max,
            max,
        }
    }
}

impl IncomeAndConsumption {
    pub fn new(
        income_age_parameter: f64,
        income_education_parameter: f64,
        base_consumption: f64,
        aditional_consumption_rate: f64,
    ) -> Self {
        Self {
            income_age_parameter,
            income_education_parameter,
            base_consumption,
            aditional_consumption_rate,
        }
    }
}

impl Transaction {
    pub fn new(
        transaction_probability: f64,
        education_parameter: f64,
        age_parameter: f64,
        tax_rate: f64,
        amount_rate: f64,
    ) -> Self {
        Self {
            transaction_probability,
            education_parameter,
            age_parameter,
            tax_rate,
            amount_rate,
        }
    }
}

impl Wealth {
    pub fn new(
        min_initial_wealth: f64,
        max_initial_wealth: f64,
        min_inheritance_at_birth_rate: f64,
        max_inheritance_at_birth_rate: f64,
    ) -> Self {
        Self {
            min_initial_wealth,
            max_initial_wealth,
            min_inheritance_at_birth_rate,
            max_inheritance_at_birth_rate,
        }
    }
}

impl EnvironmentConfig {
    pub fn new(
        num_agents: usize,
        length: usize,
        width: usize,
        interaction_radius: f64,
        max_movement: f64,
        age_and_death: AgeAndDeath,
        education: Education,
        income_and_consumption: IncomeAndConsumption,
        transaction: Transaction,
        wealth: Wealth,
    ) -> Self {
        Self {
            num_agents,
            length,
            width,
            interaction_radius,
            max_movement,
            age_and_death,
            education,
            income_and_consumption,
            transaction,
            wealth,
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
