use rand::prelude::IndexedRandom;

pub struct MyApp {
    pub last_update_time: std::time::Instant,
    pub per_click_totals: f64,

    pub total_score: f64,
    pub dmg_per_second: f64,

    pub upgraders: Vec<Upgrader>,
    pub clicky_upgraders: Vec<ClickyUpgrader>,

    pub jobs: Vec<Job>,
    pub job_count: f64,
    pub jobs_pileup_limit: f64,

    pub typing_job_start: std::time::Instant,
    pub word_list: WordList,
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
    pub completion_percentage: f64,
}

impl Job {
    pub fn new(target_text: String) -> Self {
        Self {
            target_text,
            text_input: String::new(),
            finished: false,
            completion_percentage: 0.0,
        }
    }
}

pub struct WordList {
    pub words: Vec<String>,
}

impl WordList {
    pub fn new() -> Self {
        Self {
            words: vec![
                "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not",
                "on", "with", "he", "as", "you", "do", "at", "this", "but", "his", "by", "from",
                "they", "we", "say", "her", "she", "or", "an", "will", "my", "one", "all", "would",
                "there", "their", "what", "so", "up", "out", "if", "about", "who", "get", "which",
                "go", "me", "when", "make", "can", "like", "time", "no", "just", "him", "know",
                "take", "people", "into", "year", "your", "good", "some", "could", "them", "see",
                "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
                "also", "back", "after", "use", "two", "how", "our", "work", "first", "well",
                "way", "even", "new", "want", "because", "any", "these", "give", "day", "most",
                "us",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        }
    }

    pub fn random_select(&self) -> String {
        let mut rng = rand::rng();

        self.words
            .choose(&mut rng)
            .cloned()
            .unwrap_or_else(|| "rust".to_string())
    }
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
                Upgrader {
                    name: "Tyler_Toney".to_string(),
                    cost: 1000000.0,
                    count: 0.0,
                    cost_multi: 1.05,
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
                Job::new("the quick brown fox jumped over the lazy dog".to_string()),
                Job::new("the way to the finish is the long path home".to_string()),
            ],
            job_count: 2.0,
            jobs_pileup_limit: 10.0,

            typing_job_start: std::time::Instant::now(),
            word_list: WordList::new(),
        }
    }
}
