use core::f64;

use crate::models::{ClickyUpgrader, Upgrader, WordList};

pub fn can_afford(cost: f64, total_score: f64) -> bool {
    total_score >= cost
}

pub fn passive_score_calc(upgraders: &[Upgrader]) -> f64 {
    let mut total_passive = 0.0;

    for upgrader in upgraders {
        total_passive += upgrader.passive_score_ps;
    }

    total_passive
}

pub fn handle_upgrade(upgrader: &mut Upgrader, total_score: &mut f64) {
    if can_afford(upgrader.cost, *total_score) {
        if upgrader.name == "blues" {
            *total_score -= upgrader.cost;
            upgrader.cost *= upgrader.cost_multi;
            upgrader.count += 1.0;
            upgrader.passive_score_ps += 2.0;
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

pub fn handle_clicky_upgrader(clicky_upgrader: &mut ClickyUpgrader, total_score: &mut f64) {
    if can_afford(clicky_upgrader.cost, *total_score) {
        if clicky_upgrader.name == "base" {
            *total_score -= clicky_upgrader.cost;
            clicky_upgrader.cost *= clicky_upgrader.cost_multi;
            clicky_upgrader.count += 1.0;
        } else if clicky_upgrader.name == "multi" {
            *total_score -= clicky_upgrader.cost;
            clicky_upgrader.cost *= clicky_upgrader.cost_multi;
            clicky_upgrader.count *= 1.15;
        }
    }
}

pub fn generate_target_text(word_list: &WordList, length: usize) -> String {
    let mut target_text = String::new();

    for i in 0..length {
        let random_word = word_list.random_select();

        if i > 0 {
            target_text.push(' ');
        }

        target_text.push_str(&random_word);
    }

    target_text
}

pub fn completed_percent(target_text: &String, current_text: &String) -> f64 {
    let target_len = target_text.len();
    let current_len = current_text.len();
    current_len as f64 / target_len as f64
}

pub fn completion_color(completion_percent: f64) -> egui::Color32 {
    //we're at 227, 148, 159
    //we want to be at 163, 255, 135
    //differences 64, 107, 24
    if !(completion_percent > 1.0) {
        let diff_red = 64.0;
        let diff_green = 107.0;
        let diff_blue = 24.0;
        let mut red: u8 = 227;
        let mut green: u8 = 148;
        let mut blue: u8 = 159;

        //so for each color we do color += difference * completion_percent

        red -= (diff_red * completion_percent) as u8;
        green += (diff_green * completion_percent) as u8;
        blue -= (diff_blue * completion_percent) as u8;

        egui::Color32::from_rgb(red, green, blue)
    } else {
        egui::Color32::from_rgb(163, 255, 135)
    }
}
