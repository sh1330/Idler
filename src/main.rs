use eframe::egui;

fn can_afford(cost: f64, total_score: f64) -> bool {
    total_score >= cost
}

fn handleUpgrade(upgrader: &mut Upgrader, total_score: &mut f64) {
    if can_afford(upgrader.cost, *total_score) {
        if upgrader.name == "blues" {
            *total_score -= upgrader.cost;
            upgrader.cost *= upgrader.cost_multi;
            upgrader.count += 1.0;
        } else if upgrader.name == "jacob_collier" {
            *total_score -= upgrader.cost;
            upgrader.cost *= upgrader.cost_multi;
            upgrader.count += 1.0;
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Idler",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    //score
    total_score: f64,
    dmg_per_second: f64,

    //clicker handling
    base_clicky: f64,
    clicker_upgrade_multi: f64,

   
   
    //clicky costs
    base_clicky_cost: f64,
    clicker_multi_cost: f64,

   
    upgraders: Vec<Upgrader>,
    clicky_upgraders: Vec<ClickyUpgraders>,
}

struct Upgrader {
    // each upgrader has a cost, count, and cost_multi
    name: String,
    cost: f64,
    count: f64,
    cost_multi: f64,
}

struct ClickyUpgraders {
    cost: f64,
    count: f64,
    cost_multi: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            //score
            total_score: 0.0,
            dmg_per_second: 0.0,

            //clicker handling
            base_clicky: 1.0,
            clicker_upgrade_multi: 1.0,

            
            
            //clicky costs
            base_clicky_cost: 250.0,
            clicker_multi_cost: 50.0,

           
            upgraders: vec![
                Upgrader {
                    name: "blues".to_string(),
                    cost: 10.0,
                    count: 0.0,
                    cost_multi: 1.15,
                },
                Upgrader {
                    name: "jacob_collier".to_string(),
                    cost: 100.0,
                    count: 0.0,
                    cost_multi: 1.15,
                },
            ],
            clicky_upgraders: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("settings").show_inside(ui, |ui| {
            ui.label("Settings Tab");
        });

        egui::Panel::right("clicker_options").show_inside(ui, |ui| {
            ui.label("Clicker Options");

            ui.label(format!("Base Clicky: {:.2}", self.base_clicky));
            if ui.button("Increase base clicky").clicked() {
                if can_afford(self.base_clicky_cost, self.total_score) {
                    self.base_clicky = self.base_clicky + 1.0;
                    self.total_score -= self.base_clicky_cost;
                }
            }

            ui.label(format!("Cost: {:.2}", self.base_clicky_cost));

            ui.separator();

            ui.label(format!(
                "Clicky Multiplier: {:.2}",
                self.clicker_upgrade_multi
            ));
            if ui.button("Increase clicky multiplier").clicked() {
                if can_afford(self.clicker_multi_cost, self.total_score) {
                    self.clicker_upgrade_multi = self.clicker_upgrade_multi * 1.1;
                    self.total_score -= self.clicker_multi_cost;
                }
            }
            ui.label(format!("Cost: {:.2}", self.clicker_multi_cost));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Idler");

            if ui.button("Increment score").clicked() {
                self.total_score += self.base_clicky * self.clicker_upgrade_multi;
            }

            ui.label(format!("Total Score: {:.0}", self.total_score));

            ui.separator();

            ui.horizontal(|ui| {
                for upgrader in &mut self.upgraders {
                    ui.vertical(|ui| {
                        if ui.button(format!("{} Upgrade", upgrader.name)).clicked() {
                            handleUpgrade(upgrader, &mut self.total_score);
                        }
                        ui.label(format!("{} count: {}", upgrader.name, upgrader.count));
                        ui.label(format!("Cost :{:.0}", upgrader.cost));
                    });
                }
            });
        });
    }
}
