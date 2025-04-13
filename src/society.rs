use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::agent::Agent;

pub struct Society {
    pub agents: Vec<Agent>,
    pub tax_rate: f64,
    pub redistribution: bool,
}

impl Society {
    pub fn new(agent_count: usize, tax_rate: f64, redistribution: bool) -> Self {
        let agents = (0..agent_count).map(|id| Agent::new(id)).collect();

        Society {
            agents,
            tax_rate,
            redistribution,
        }
    }

    /// Simulate one round of random transactions
    pub fn simulate_transactions(&mut self, exchanges: usize) {
        let mut rng = thread_rng();
        let len = self.agents.len();

        for _ in 0..exchanges {
            if len < 2 {
                break;
            }

            // Randomly pick two distinct indices
            let i = rng.gen_range(0..len);
            let mut j = rng.gen_range(0..len);
            while j == i {
                j = rng.gen_range(0..len);
            }

            // Now borrow them separately
            let (a1, a2) = if i < j {
                let (left, right) = self.agents.split_at_mut(j);
                (&mut left[i], &mut right[0])
            } else {
                let (left, right) = self.agents.split_at_mut(i);
                (&mut right[0], &mut left[j])
            };

            let amount = rng.gen_range(1.0..10.0);
            if a1.wealth >= amount {
                a1.wealth -= amount;
                a2.wealth += amount;
            }
        }
    }

    /// Apply taxation and optional redistribution
    pub fn apply_taxation(&mut self) {
        let mut tax_pool = 0.0;

        for agent in &mut self.agents {
            let tax = agent.wealth * self.tax_rate;
            agent.wealth -= tax;
            tax_pool += tax;
        }

        if self.redistribution && !self.agents.is_empty() {
            let per_agent = tax_pool / self.agents.len() as f64;
            for agent in &mut self.agents {
                agent.wealth += per_agent;
            }
        }
    }

    pub fn average_wealth(&self) -> f64 {
        let total: f64 = self.agents.iter().map(|a| a.wealth).sum();
        total / self.agents.len() as f64
    }

    pub fn print_summary(&self) {
        let avg = self.average_wealth();
        let min = self
            .agents
            .iter()
            .map(|a| a.wealth)
            .fold(f64::INFINITY, f64::min);
        let max = self
            .agents
            .iter()
            .map(|a| a.wealth)
            .fold(f64::NEG_INFINITY, f64::max);
        println!(
            "Wealth Summary - Avg: {:.2}, Min: {:.2}, Max: {:.2}",
            avg, min, max
        );
    }
}
