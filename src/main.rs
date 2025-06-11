mod agent;
mod environment;
mod environment_config;
mod gui;
mod metrics;

use std::env;
use std::path::Path;

use crate::environment::Environment;
use crate::environment_config::{
    AgeAndDeath, Education, EnvironmentConfig, IncomeAndConsumption, Transaction,
};
use crate::gui::SimApp;

fn create_default_config() {
    let age_and_death = AgeAndDeath::new(30.0, 10.0, 80.0, 90.0, 0.02);
    let education = Education::new(4.0, 10.0, 4.0, 2.0, 0.005, 0.05, 10.0);
    let income_and_consumption = IncomeAndConsumption::new(0.05, 2.0, 10.0, 0.2);
    let transaction = Transaction::new(0.3, 1.0, 0.001, 0.05, 0.05);
    let wealth = environment_config::Wealth::new(10.0, 100.0, 0.1, 0.3);
    let config = EnvironmentConfig::new(
        5000,
        1000,
        1000,
        1000,
        50.0,
        15.0,
        age_and_death,
        education,
        income_and_consumption,
        transaction,
        wealth,
    );
    config.save_to_file("config/default.json");
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = env::args().collect();

    create_default_config();

    let config_path = if args.len() > 1 {
        println!("Loading config from: {}", args[1]);
        args[1].as_str()
    } else {
        println!("No config file provided, using default.");
        "config/default.json"
    };
    let config_filename = Path::new(config_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("default");

    let config = EnvironmentConfig::load_from_file(config_path);
    let env = Environment::new(&config);

    let native_options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };

    let metrics_filepath = format!("visualisation/metrics_{}.csv", config_filename);

    eframe::run_native(
        "Wealth Simulation",
        native_options,
        Box::new(move |_cc| Box::new(SimApp::new(env, Some(&metrics_filepath), true))),
    )
}
