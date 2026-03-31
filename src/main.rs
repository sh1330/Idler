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
            upgrader.passive_score_ps += 10.0;
        } else if upgrader.name == "cowboy_chordist" {
            *total_score -= upgrader.cost;
            upgrader.cost *= upgrader.cost_multi;
            upgrader.count += 1.0;
            upgrader.passive_score_ps += 1000.0;
        }
    }
}

fn handleClickyUpgraders(clicky_upgraders: &mut ClickyUpgraders, total_score: &mut f64) {
    if can_afford(clicky_upgraders.cost, *total_score) {
        if clicky_upgraders.name == "base" {
            *total_score -= clicky_upgraders.cost;
            clicky_upgraders.cost *= clicky_upgraders.cost_multi;
            clicky_upgraders.count += 1.0;
        } else if clicky_upgraders.name == "multi" {
            *total_score -= clicky_upgraders.cost;
            clicky_upgraders.cost *= clicky_upgraders.cost_multi;
            clicky_upgraders.count *= 1.15;
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
    per_click_totals: f64,

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
    name: String,
    cost: f64,
    count: f64,
    cost_multi: f64
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            //status
            last_update_time: std::time::Instant::now(),
            per_click_totals: 0.0,

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
                Upgrader {
                    name: "cowboy_chordist".to_string(),
                    cost: 10000.0,
                    count: 0.0,
                    cost_multi: 1.5,
                    passive_score_ps: 0.0,
                },
            ],
            clicky_upgraders: vec![
                ClickyUpgraders {
                    name: "base".to_string(),
                    cost: 250.0,
                    count: 1.0,
                    cost_multi: 1.15,
                },
                ClickyUpgraders {

                    name: "multi".to_string(),
                    cost: 50.0,
                    count: 1.0,
                    cost_multi: 1.3,

                }
            ]
        }
    }
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

            ui.label(format!("Base Clicky: {:.2}", self.clicky_upgraders[0].count));
            if ui.button("Increase base clicky").clicked() {
                handleClickyUpgraders(&mut self.clicky_upgraders[0], &mut self.total_score);
            }

            ui.label(format!("Cost: {:.2}", self.clicky_upgraders[0].cost));

            ui.separator();

            ui.label(format!("Clicky Multiplier: {:.2}", self.clicky_upgraders[1].count));
            if ui.button("Increase clicky multiplier").clicked() {
                handleClickyUpgraders(&mut self.clicky_upgraders[1], &mut self.total_score);
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
