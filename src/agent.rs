use rand::Rng;
use std::f64::consts::PI;

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
}

impl Agent {
    pub fn new(id: usize, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            children: Vec::new(),
            x: rng.gen_range(min_x..max_x),
            y: rng.gen_range(min_y..max_y),
            wealth: rng.gen_range(10.0..100.0),
            education: rng.gen_range(5.0..10.0),
            age: rng.gen_range(18 * 12..80 * 12),
            alive: true,
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
        // Parameters for the sigmoid
        let mid_age = 80.0 * 12.0; // age where death chance is 50%
        let steepness = 0.1; // how quickly probability rises with age
        let death_chance = 1.0 / (1.0 + (-steepness * (self.age as f64 - mid_age)).exp());
        if rand::random::<f64>() < death_chance {
            self.alive = false;
        }
        !self.alive
    }

    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}
