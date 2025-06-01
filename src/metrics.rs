use crate::agent::Agent;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Metrics {
    file_path: String,
}

impl Metrics {
    pub fn new(file_path: &str) -> Self {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Failed to create metrics file");

        writeln!(
            file,
            "iteration,gini,min,p10,p25,p50,p75,p90,max,total_wealth"
        )
        .unwrap();

        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn log(&self, iteration: usize, agents: &[Agent]) {
        let mut wealths: Vec<f64> = agents
            .iter()
            .filter(|a| a.alive)
            .map(|a| a.wealth)
            .collect();
        if wealths.is_empty() {
            return;
        }

        wealths.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = wealths.len();
        let gini = Self::gini(&wealths);
        let total_wealth: f64 = wealths.iter().sum();

        let percentile = |p: f64| -> f64 {
            let idx = (p * n as f64).floor() as usize;
            wealths.get(idx.min(n - 1)).cloned().unwrap_or(0.0)
        };

        let min = *wealths.first().unwrap_or(&0.0);
        let max = *wealths.last().unwrap_or(&0.0);
        let p10 = percentile(0.10);
        let p25 = percentile(0.25);
        let p50 = percentile(0.50);
        let p75 = percentile(0.75);
        let p90 = percentile(0.90);

        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .expect("Failed to open metrics file");
        writeln!(
            file,
            "{},{:.5},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
            iteration, gini, min, p10, p25, p50, p75, p90, max, total_wealth
        )
        .unwrap();
    }

    fn gini(wealths: &[f64]) -> f64 {
        let n = wealths.len() as f64;
        let sum_x = wealths.iter().sum::<f64>();
        let sum_ix = wealths
            .iter()
            .enumerate()
            .map(|(i, x)| (i + 1) as f64 * x)
            .sum::<f64>();
        if sum_x == 0.0 {
            0.0
        } else {
            (2.0 * sum_ix) / (n * sum_x) - (n + 1.0) / n
        }
    }
}
