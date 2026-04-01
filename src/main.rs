mod logic;
mod models;

use eframe::egui;

use crate::logic::{
    completed_percent, completion_color, generate_target_text, handle_clicky_upgrader,
    handle_upgrade, passive_score_calc,
};
use crate::models::{Job, MyApp};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Idler",
        options,
        Box::new(|cc| {
            cc.egui_ctx.style_mut(|style| {
                style.visuals.override_text_color = Some(egui::Color32::WHITE);
                style.visuals.panel_fill = egui::Color32::from_rgb(15, 15, 15);
            });

            Ok(Box::new(MyApp::default()))
        }),
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

        while now.duration_since(self.typing_job_start).as_secs_f64() >= 10.0 {
            let target_text = generate_target_text(&self.word_list, 10);
            self.jobs.push(Job::new(target_text));
            self.job_count += 1.0;
            self.typing_job_start += std::time::Duration::from_secs(15);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //SETTINGS
        //
        //
        //
        egui::Panel::left("settings").show_inside(ui, |ui| {
            ui.label("Settings Tab");
        });

        //Clicker Options
        //
        //
        //
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

        //Central Idler Content
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Idler");

            if ui.button("Increment score").clicked() {
                self.total_score += self.per_click_totals;
            }

            ui.label(format!("Total Score: {:.2}", self.total_score));
            ui.label(format!("Total Passive: {:.2}", self.dmg_per_second));

            ui.separator();

            ui.heading("Get lessons!");
            ui.horizontal(|ui| {
                for upgrader in &mut self.upgraders {
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(117, 35, 35))
                        .corner_radius(15)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                if ui.button(format!("{} Upgrade", upgrader.name)).clicked() {
                                    handle_upgrade(upgrader, &mut self.total_score);
                                }
                                ui.label(format!("{} count: {}", upgrader.name, upgrader.count));
                                ui.label(format!("Cost: {:.0}", upgrader.cost));
                            });
                        });
                }
            });
            ui.separator();
            //the story is this section of the game
            //is when he has to go to work
            //but then eventually he'll graduate to a rockstar or something maybe
            ui.label(
                egui::RichText::new("Day Job")
                    .size(24.0)
                    .strong()
                    .color(egui::Color32::LIGHT_BLUE),
            );
            ui.separator();
            for job in &mut self.jobs {
                if job.finished == false {
                    job.completion_percentage =
                        completed_percent(&job.target_text, &job.text_input);

                    egui::Frame::group(ui.style())
                        .corner_radius(15)
                        .show(ui, |ui| {
                            ui.label("Target");
                            ui.heading(
                                egui::RichText::new(format!("{}", job.target_text))
                                    .size(24.0)
                                    .color(completion_color(job.completion_percentage))
                                    .strong(),
                            );
                            ui.text_edit_singleline(&mut job.text_input);
                            if job.text_input == job.target_text {
                                job.finished = true;
                                self.job_count -= 1.0;
                            } else {
                                job.finished = false;
                            }
                            ui.label(format!("Completed Percent:{}", job.completion_percentage));
                        });
                    ui.separator();
                }
            }
            if self.job_count == 0.0 {
                ui.heading("Well Done! No more jobs to complete");
                ui.label(format!("The job count is: {}", self.job_count));
            }

            ui.heading(format!(
                "Job pile up that gets you fired: {}",
                self.jobs_pileup_limit
            ));
            if self.job_count >= self.jobs_pileup_limit {
                ui.heading("YOU'RE FIRED");
            } else {
                ui.heading("The boss thinks you're doing an acceptable job");
            }
        });
    }
}
