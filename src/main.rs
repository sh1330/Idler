
use eframe::egui;

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
    
    //upgraders
    blues_upgrade_count: f64,
    jacob_collier_count: f64,

    //costs
    
    //clicky costs
    base_clicky_cost: f64,
    clicker_multi_cost: f64,
    

    //upgrader costs
    blues_upgrade_cost: f64,
    jacob_collier_cost: f64,

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
            
            //Upgraders
            blues_upgrade_count: 0.0,
            jacob_collier_count: 0.0,

            //costs
            
            //clicky costs
            base_clicky_cost: f64,
            clicker_multi_cost: f64,

            //upgrader costs
            blues_upgrade_cost: 10,
            jacob_collier_cost: 100.0,
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
                    self.base_clicky = self.base_clicky + 1.0;
            }

            ui.label(format!("Clicky Multiplier: {:.2}", self.clicker_upgrade_multi));
            if ui.button("Increase clicky multiplier").clicked() {
                self.clicker_upgrade_multi = self.clicker_upgrade_multi * 1.1;
            }
        });

        



        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Idler");

            if ui.button("Increment score").clicked() {
                self.total_score += self.base_clicky * self.clicker_upgrade_multi;
            }

            ui.label(format!("Total Score: {:.0}", self.total_score));

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.button("Blues Upgrade").clicked() {
                        self.blues_upgrade_count += 1.0;
                    }
                    ui.label(format!("blues count: {}", self.blues_upgrade_count));
                });

                ui.vertical(|ui| {
                    if ui.button("Jacob Collier Upgrade").clicked() {
                        self.jacob_collier_count += 1.0;
                    }
                    ui.label(format!("jacob collier count: {}", self.jacob_collier_count));
                });
            });
        });
    }
}
