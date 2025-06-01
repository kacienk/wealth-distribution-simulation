use eframe::egui;
use eframe::App;
use egui::debug_text::print;

use crate::environment::Environment;

pub struct SimApp {
    pub env: Environment,
}

impl SimApp {
    pub fn new(env: Environment) -> Self {
        Self { env }
    }
}

impl eframe::App for SimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.env.step(); // Advance simulation

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

            // Draw agents
            for agent in &self.env.agents {
                if !agent.alive {
                    continue;
                }
                let sim_pos = egui::pos2(agent.x as f32, agent.y as f32);
                let screen_pos = to_screen * sim_pos;

                let wealth = agent.wealth.clamp(0.0, 255.0);
                let color = egui::Color32::from_rgb(wealth as u8, 100, 255 - wealth as u8);

                painter.circle_filled(screen_pos, 3.0, color);
            }

            println!(
                "Iter: {}, Cummulative Wealth: {}",
                self.env.iteration,
                self.env.agents.iter().map(|a| a.wealth).sum::<f64>()
            );
        });

        ctx.request_repaint();
    }
}
