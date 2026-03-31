use eframe::egui;

//check if they can afford the upgrade
fn can_afford(cost: f64, total_score: f64) -> bool {
    total_score >= cost
}

//calculate the passive score
fn passive_score_calc(upgraders: &[Upgrader]) -> f64 {
    let mut total_passive = 0.0;

    for upgrader in upgraders {
        total_passive += upgrader.passive_score_ps;
    }

    total_passive
}

fn apply_passive_score(upgraders: &[Upgrader], total_score: &mut f64, seconds_passed: f64) {
    let passive_per_second = passive_score_calc(upgraders);
    *total_score += passive_per_second * seconds_passed;
}

fn handleUpgrade(upgrader: &mut Upgrader, total_score: &mut f64) {
    if can_afford(upgrader.cost, *total_score) {
        //if blues upgrade this way
        if upgrader.name == "blues" {
            *total_score -= upgrader.cost;
            upgrader.cost *= upgrader.cost_multi;
            upgrader.count += 1.0;
            upgrader.passive_score_ps += 2.0;
        
        //if jacob collier upgrade this way
        } else if upgrader.name == "jacob_collier" {
            *total_score -= upgrader.cost;
            upgrader.cost *= upgrader.cost_multi;
            upgrader.count += 1.0;
            upgrader.passive_score_ps += 20.0;
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
    //state stuff
    last_update_time: std::time::Instant,

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
    passive_score_ps: f64,
}

struct ClickyUpgraders {
    cost: f64,
    count: f64,
    cost_multi: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            //status
            last_update_time: std::time::Instant::now(),

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
                    passive_score_ps: 0.0,
                },
                Upgrader {
                    name: "jacob_collier".to_string(),
                    cost: 100.0,
                    count: 0.0,
                    cost_multi: 1.15,
                    passive_score_ps: 0.0,
                },
            ],
            clicky_upgraders: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(100));
        let now = std::time::Instant::now();
        let seconds_passed = now.duration_since(self.last_update_time).as_secs_f64();
        self.last_update_time = now;

        apply_passive_score(&self.upgraders, &mut self.total_score, seconds_passed);
        self.dmg_per_second = passive_score_calc(&self.upgraders);


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
            ui.label(format!("Total Passive: {:.0}", passive_score_calc(&self.upgraders)));

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
