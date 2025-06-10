use eframe::egui;

use crate::{environment::Environment, metrics::Metrics};

pub struct SimApp {
    pub env: Environment,
    pub max_iter: Option<usize>,
    pub metrics: Metrics,
    pub logging_enabled: bool,
}

impl SimApp {
    pub fn new(
        env: Environment,
        filepath: Option<&str>,
        logging_enabled: bool,
    ) -> Self {
        let metrics = Metrics::new(filepath.unwrap_or("visualisation/metrics.csv"));
        let num_iterations = env.config.num_iterations;
        Self {
            env,
            max_iter: Some(num_iterations),
            metrics,
            logging_enabled,
        }
    }
}

impl eframe::App for SimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.max_iter.is_some() && self.env.iteration >= self.max_iter.unwrap() {
            ctx.request_repaint();
            return;
        }
        self.env.step();
        if self.logging_enabled {
            self.metrics.log(self.env.iteration, &self.env.agents);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            let (response, painter) = ui.allocate_painter(available, egui::Sense::hover());

            let (_min_x, max_x, _min_y, max_y) = self.env.bounds();

            // Define simulation area
            let sim_rect = egui::Rect::from_min_max(
                egui::pos2(0.0, 0.0),
                egui::pos2(max_x as f32, max_y as f32),
            );

            // Define screen area to draw to
            let screen_rect = response.rect;

            // Build transform: simulation → screen
            let to_screen = egui::emath::RectTransform::from_to(sim_rect, screen_rect);

            let min_wealth = self
                .env
                .agents
                .iter()
                .map(|a| a.wealth)
                .fold(f64::INFINITY, f64::min);
            let max_wealth = self
                .env
                .agents
                .iter()
                .map(|a| a.wealth)
                .fold(f64::NEG_INFINITY, f64::max);

            // Draw agents
            for agent in &self.env.agents {
                if !agent.alive {
                    continue;
                }
                let sim_pos = egui::pos2(agent.x as f32, agent.y as f32);
                let screen_pos = to_screen * sim_pos;

                // Normalize wealth to a color value
                // Assuming wealth is in the range [min_wealth, max_wealth]
                if min_wealth == max_wealth {
                    continue; // Avoid division by zero
                }
                let wealth = if min_wealth == max_wealth {
                    128 // Neutral color if all agents have the same wealth
                } else {
                    ((agent.wealth - min_wealth) / (max_wealth - min_wealth) * 255.0)
                        .clamp(0.0, 255.0) as u8
                };
                let color = egui::Color32::from_rgb(wealth as u8, 100, 255 - wealth as u8);

                painter.circle_filled(screen_pos, 3.0, color);
            }

            println!(
                "Iter: {}, Cummulative Wealth: {}, min Wealth: {}, max Wealth: {}",
                self.env.iteration,
                self.env.agents.iter().map(|a| a.wealth).sum::<f64>(),
                min_wealth,
                max_wealth
            );
        });

        ctx.request_repaint();
    }
}
