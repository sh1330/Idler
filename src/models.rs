pub struct MyApp {
    pub last_update_time: std::time::Instant,
    pub per_click_totals: f64,

    pub total_score: f64,
    pub dmg_per_second: f64,

    pub upgraders: Vec<Upgrader>,
    pub clicky_upgraders: Vec<ClickyUpgrader>,

    pub jobs: Vec<Job>,
    pub job_count: f64,
}

pub struct Upgrader {
    pub name: String,
    pub cost: f64,
    pub count: f64,
    pub cost_multi: f64,
    pub passive_score_ps: f64,
}

pub struct ClickyUpgrader {
    pub name: String,
    pub cost: f64,
    pub count: f64,
    pub cost_multi: f64,
}

pub struct Job {
    pub target_text: String,
    pub text_input: String,
    pub finished: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            last_update_time: std::time::Instant::now(),
            per_click_totals: 0.0,

            total_score: 0.0,
            dmg_per_second: 0.0,

            upgraders: vec![
                Upgrader {
                    name: "blues".to_string(),
                    cost: 10.0,
                    count: 0.0,
                    cost_multi: 1.05,
                    passive_score_ps: 0.0,
                },
                Upgrader {
                    name: "jacob_collier".to_string(),
                    cost: 100.0,
                    count: 0.0,
                    cost_multi: 1.05,
                    passive_score_ps: 0.0,
                },
                Upgrader {
                    name: "cowboy_chordist".to_string(),
                    cost: 1000.0,
                    count: 0.0,
                    cost_multi: 1.5,
                    passive_score_ps: 0.0,
                },
            ],
            clicky_upgraders: vec![
                ClickyUpgrader {
                    name: "base".to_string(),
                    cost: 250.0,
                    count: 1.0,
                    cost_multi: 1.15,
                },
                ClickyUpgrader {
                    name: "multi".to_string(),
                    cost: 50.0,
                    count: 1.0,
                    cost_multi: 1.3,
                },
            ],
            jobs: vec![
                Job {
                    target_text: "the quick brown fox jumped over the lazy dog".to_string(),
                    text_input: "".to_string(),
                    finished: false,
                },
                Job {
                    target_text: "the way to the finished is the long path home".to_string(),
                    text_input: "".to_string(),
                    finished: false,
                },
            ],
            job_count: 2.0,
        }
    }
}
