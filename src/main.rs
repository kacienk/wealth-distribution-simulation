mod agent;
mod environment;
mod environment_config;
mod gui;
mod metrics;

use crate::environment::Environment;
use crate::environment_config::EnvironmentConfig;
use crate::gui::SimApp;
use crate::metrics::Metrics;

fn main() -> eframe::Result<()> {
    let config = EnvironmentConfig::new(1000, 1000, 50.0, 1.0, 5.0); // interaction radius, max movement, tax rate
    let env = Environment::new(2000, &config); // 300 agents

    let native_options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Wealth Simulation",
        native_options,
        Box::new(|_cc| Box::new(SimApp::new(env))),
    )
}
