use rand::Rng;
use std::f64::consts::PI;

use crate::environment_config::{AgeAndDeath, Education, Wealth};

#[derive(Clone)]
pub struct Agent {
    pub children: Vec<usize>,
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub wealth: f64,
    pub education: f64,
    pub age: u32,
    pub alive: bool,
    pub mid_age: f64,   // age where death chance is 50%
    pub steepness: f64, // how quickly death probability rises with age
}

impl Agent {
    pub fn new(
        id: usize,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        age_and_death: &AgeAndDeath,
        education: &Education,
        wealth: &Wealth,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let age_years = rng.gen_range(0.0..age_and_death.max_start_age);

        let age_months = (age_years * 12.0).floor() as u32;

        let education = if age_years < 6.0 {
            0.0
        } else if age_years < 18.0 {
            let base = ((age_years - 6.0) / 12.0) * education.elemental_education_threshold;
            base + rng.gen_range(0.0..education.children_education_jitter)
        } else {
            education.initial_adult_min
                + rng.gen_range(0.0..1.0)
                    * (education.initial_adult_max - education.initial_adult_min)
        };

        Self {
            id,
            children: Vec::new(),
            x: rng.gen_range(min_x..max_x),
            y: rng.gen_range(min_y..max_y),
            wealth: rng.gen_range(wealth.min_initial_wealth..wealth.max_initial_wealth),
            education: education,
            age: age_months,
            alive: true,
            mid_age: age_and_death.mid_age,
            steepness: age_and_death.steepness,
        }
    }

    pub fn move_randomly(
        &mut self,
        max_distance: f64,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
    ) {
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(0.0..2.0 * PI);
        let delta = rng.gen_range(0.0..max_distance);
        // Ensure the agent stays within bounds
        self.x = (self.x + delta * theta.cos()).clamp(min_x, max_x);
        self.y = (self.y + delta * theta.sin()).clamp(min_y, max_y);
    }

    pub fn income(&self, alpha: f64, beta: f64) -> f64 {
        alpha * self.education + beta * self.age as f64
    }

    pub fn age_and_check_death(&mut self) -> bool {
        self.age += 1;
        let mid_age = self.mid_age * 12.0; // age where death chance is 50%
        let steepness = self.steepness; // how quickly probability rises with age
        let death_chance = 1.0 / (1.0 + (-steepness * (self.age as f64 - mid_age)).exp());
        if rand::random::<f64>() < death_chance {
            self.alive = false;
        }
        !self.alive
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18 * 12
    }
}
