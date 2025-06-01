mod agent;
mod environment;
mod environment_config;
mod metrics;

use crate::environment::Environment;
use crate::environment_config::EnvironmentConfig;
use crate::metrics::Metrics;

fn main() {
    let config = EnvironmentConfig::new(1000, 1000, 50.0, 1.0, 5.0); // interaction radius, max movement, tax rate
    let mut world = Environment::new(100, &config); // 100 agents
    let mut metrics = Metrics::new();

    for step in 0..1000 {
        println!("Step {}", step);
        world.step();
        metrics.record(&world.agents);
    }

    metrics.report();
}
