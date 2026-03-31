use crate::models::{ClickyUpgrader, Upgrader};

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
