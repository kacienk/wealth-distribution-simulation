use crate::agent::Agent;

pub struct Metrics {
    gini_values: Vec<f64>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            gini_values: vec![],
        }
    }

    pub fn record(&mut self, agents: &[Agent]) {
        let alive_wealths: Vec<f64> = agents
            .iter()
            .filter(|a| a.alive)
            .map(|a| a.wealth)
            .collect();
        self.gini_values.push(Self::gini(&alive_wealths));
    }

    pub fn gini(values: &[f64]) -> f64 {
        let n = values.len();
        if n == 0 {
            return 0.0;
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let sum: f64 = sorted.iter().sum();
        let mut cumulative = 0.0;
        for (i, v) in sorted.iter().enumerate() {
            cumulative += (i + 1) as f64 * v;
        }
        let gini = (2.0 * cumulative) / (n as f64 * sum) - (n as f64 + 1.0) / n as f64;
        gini
    }

    pub fn report(&self) {
        for (i, g) in self.gini_values.iter().enumerate() {
            println!("Step {}: Gini = {:.3}", i, g);
        }
    }
}
