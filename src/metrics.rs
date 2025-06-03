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
            "iteration,gini,min,p10,p25,p50,p75,p90,max,total_wealth,adult_agents,edu_mean,edu_min,edu_p10,edu_p25,edu_p50,edu_p75,edu_p90,edu_max"
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
        let mut educations: Vec<f64> = agents
            .iter()
            .filter(|a| a.alive)
            .map(|a| a.education)
            .collect();

        if wealths.is_empty() {
            return;
        }

        wealths.sort_by(|a, b| a.partial_cmp(b).unwrap());
        educations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = wealths.len();
        let gini = Self::gini(&wealths);
        let total_wealth: f64 = wealths.iter().sum();

        let percentile = |v: &Vec<f64>, p: f64| -> f64 {
            let idx = (p * v.len() as f64).floor() as usize;
            v.get(idx.min(v.len() - 1)).cloned().unwrap_or(0.0)
        };

        let min = *wealths.first().unwrap_or(&0.0);
        let max = *wealths.last().unwrap_or(&0.0);
        let p10 = percentile(&wealths, 0.10);
        let p25 = percentile(&wealths, 0.25);
        let p50 = percentile(&wealths, 0.50);
        let p75 = percentile(&wealths, 0.75);
        let p90 = percentile(&wealths, 0.90);

        let edu_min = *educations.first().unwrap_or(&0.0);
        let edu_max = *educations.last().unwrap_or(&0.0);
        let edu_mean = if !educations.is_empty() {
            educations.iter().sum::<f64>() / educations.len() as f64
        } else {
            0.0
        };
        let edu_p10 = percentile(&educations, 0.10);
        let edu_p25 = percentile(&educations, 0.25);
        let edu_p50 = percentile(&educations, 0.50);
        let edu_p75 = percentile(&educations, 0.75);
        let edu_p90 = percentile(&educations, 0.90);

        let adult_agents = agents
            .iter()
            .filter(|a| a.age >= 18 * 12 && a.alive)
            .count();

        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .expect("Failed to open metrics file");
        writeln!(
        file,
        "{},{:.5},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
        iteration, gini, min, p10, p25, p50, p75, p90, max, total_wealth, adult_agents,
        edu_mean, edu_min, edu_p10, edu_p25, edu_p50, edu_p75, edu_p90, edu_max
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
