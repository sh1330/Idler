mod logic;
mod models;

use eframe::egui;

use crate::logic::{handle_clicky_upgrader, handle_upgrade, passive_score_calc};
use crate::models::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Idler",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(1.0 / 60.0));

        let now = std::time::Instant::now();
        let seconds_passed = now.duration_since(self.last_update_time).as_secs_f64();
        self.last_update_time = now;

        self.dmg_per_second = passive_score_calc(&self.upgraders);
        self.total_score += self.dmg_per_second * seconds_passed;
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("settings").show_inside(ui, |ui| {
            ui.label("Settings Tab");
        });

        egui::Panel::right("clicker_options").show_inside(ui, |ui| {
            ui.heading("Clicker");

            ui.label(format!(
                "Base Clicky: {:.2}",
                self.clicky_upgraders[0].count
            ));
            if ui.button("Increase base clicky").clicked() {
                handle_clicky_upgrader(&mut self.clicky_upgraders[0], &mut self.total_score);
            }

            ui.label(format!("Cost: {:.2}", self.clicky_upgraders[0].cost));

            ui.separator();

            ui.label(format!(
                "Clicky Multiplier: {:.2}",
                self.clicky_upgraders[1].count
            ));
            if ui.button("Increase clicky multiplier").clicked() {
                handle_clicky_upgrader(&mut self.clicky_upgraders[1], &mut self.total_score);
            }
            ui.label(format!("Cost: {:.2}", self.clicky_upgraders[1].cost));

            ui.separator();

            self.per_click_totals = self.clicky_upgraders[0].count * self.clicky_upgraders[1].count;

            ui.label("Per Click Stats");
            ui.label(format!("{:.2}", self.per_click_totals));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Idler");

            if ui.button("Increment score").clicked() {
                self.total_score += self.per_click_totals;
            }

            ui.label(format!("Total Score: {:.2}", self.total_score));
            ui.label(format!("Total Passive: {:.2}", self.dmg_per_second));

            ui.separator();

            ui.horizontal(|ui| {
                for upgrader in &mut self.upgraders {
                    ui.vertical(|ui| {
                        if ui.button(format!("{} Upgrade", upgrader.name)).clicked() {
                            handle_upgrade(upgrader, &mut self.total_score);
                        }
                        ui.label(format!("{} count: {}", upgrader.name, upgrader.count));
                        ui.label(format!("Cost: {:.0}", upgrader.cost));
                    });
                }
            });
        });
    }
}
