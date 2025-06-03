mod agent;
mod environment;
mod environment_config;
mod gui;
mod metrics;

use crate::environment::Environment;
use crate::environment_config::EnvironmentConfig;
use crate::gui::SimApp;

fn main() -> eframe::Result<()> {
    let config = EnvironmentConfig::new(
        1000, 1000, 1000, 50.0, 0.3, 15.0, 0.0, 30.0, 10.0, 80.0, 90.0, 0.02,
    );
    let env = Environment::new(&config);

    let native_options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Wealth Simulation",
        native_options,
        Box::new(|_cc| {
            let max_iter = 5000;
            Box::new(SimApp::new(
                env,
                Some(max_iter),
                Some("visualisation/metrics.csv"),
                true,
            ))
        }),
    )
}
