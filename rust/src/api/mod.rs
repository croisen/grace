use chrono::Local;
use fsrs::FSRS;
pub use fsrs::MemoryState;
use rand::random_range;

use crate::api::grade::Grade;

pub mod concept;
pub mod diagnostics;
mod edit;
pub mod grade;

pub mod java;
pub mod rust;
pub mod sql;

static DESIRED_RETENTION: f32 = 0.9;

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(opaque)]
pub struct DrillCollection {
    pub id: usize,
    pub name: String,
    pub collection: Vec<DrillMemory>,
}

impl DrillCollection {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(id: usize, name: String, collection: Vec<DrillMemory>) -> Self {
        Self { id, name, collection }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn new_with_default(
        name: String,
        lang: DrillLanguage,
        list: Vec<String>,
    ) -> Self {
        let drill = list
            .into_iter()
            .map(|c| DrillMemory::new_with_default(lang, c))
            .collect();

        Self { id: 0, name, collection: drill }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn due_concept(&self) -> Option<String> {
        let now = local_now();
        self.collection
            .iter()
            .filter(|s| s.is_due())
            .min_by(|a, b| {
                a.retrievability(now).total_cmp(&b.retrievability(now))
            })
            .map(|drill| drill.concept.clone())
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
        for drill in &mut self.collection {
            drill.col_id = id;
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn review_drill(&mut self, concept: String, grade: Grade) {
        if let Some(drill) =
            self.collection.iter_mut().find(|d| d.concept == concept)
        {
            drill.review(grade);
        }
    }
}

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct DrillMemory {
    pub col_id: usize, // collection_id
    pub lang: DrillLanguage,
    pub concept: String,
    pub memory: Option<MemoryState>,
    pub last_review: Option<i64>,
    pub due_date: i64,
}

impl DrillMemory {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(
        id: usize,
        lang: DrillLanguage,
        concept: String,
        memory: Option<MemoryState>,
        last_review: Option<i64>,
        due_date: i64,
    ) -> Self {
        Self { col_id: id, lang, concept, memory, last_review, due_date }
    }

    pub fn new_with_default(lang: DrillLanguage, concept: String) -> Self {
        Self {
            col_id: 0,
            lang,
            concept,
            memory: None,
            last_review: None,
            due_date: local_now(),
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn retrievability(&self, now: i64) -> f32 {
        let fsrs = FSRS::default();
        let elapsed = now - self.last_review.unwrap_or(now);
        let days = elapsed as f32 / 86400.0;
        self.memory.map(|m| fsrs.current_retrievability(m, days)).unwrap_or(0.0)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn due_date_str(&self) -> String {
        let now = local_now();
        let elapsed = self.due_date - now;
        if elapsed.is_negative() {
            self.duration("Overdue by", elapsed)
        } else {
            self.duration("Due in", elapsed)
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn is_due(&self) -> bool {
        self.due_date <= Local::now().timestamp()
    }

    fn review(&mut self, grade: Grade) {
        self._review(grade);
    }

    fn review_if_due(&mut self, grade: Grade) {
        if self.is_due() {
            self.review(grade);
        }
    }

    fn _review(&mut self, grade: Grade) {
        let fsrs = FSRS::default();
        let now = local_now();
        let elapsed = now - self.last_review.unwrap_or(now);
        let days = elapsed as f32 / 86400.0;

        let next = fsrs
            .next_states_with_elapsed_days(self.memory, DESIRED_RETENTION, days)
            .unwrap();

        let next = match grade {
            Grade::Again => next.again,
            Grade::Hard => next.hard,
            Grade::Good => next.good,
            Grade::Easy => next.easy,
        };

        self.memory = Some(next.memory);
        self.due_date = now + (next.interval as i64 * 86400);
        self.last_review = Some(now);
    }

    fn duration(&self, prefix: &str, seconds: i64) -> String {
        let seconds = seconds.abs();
        let (val, unit) = if seconds >= 86400 {
            (seconds / 86400, "day")
        } else if seconds >= 3600 {
            (seconds / 3600, "hour")
        } else if seconds >= 60 {
            (seconds / 60, "minute")
        } else {
            return "Due now".into();
        };

        format!(
            "{prefix} {val} {}",
            if val == 1 { unit.into() } else { format!("{unit}s") }
        )
    }
}

#[flutter_rust_bridge::frb(mirror(MemoryState))]
pub struct _MemoryState {
    pub stability: f32,
    pub difficulty: f32,
    pub stability_fast: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum DrillLanguage {
    JAVA,
    RUST,
    SQL,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn local_now() -> i64 {
    Local::now().timestamp()
}

// there are two to prevent the same name to be put out
fn generic_varname1() -> &'static str {
    static VARS1: &[&str] = &[
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf",
        "hotel", "india", "juliett", "kilo", "lima", "mike",
    ];

    VARS1[random_range(0..VARS1.len())]
}

fn generic_varname2() -> &'static str {
    static VARS2: &[&str] = &[
        "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango",
        "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
    ];
    VARS2[random_range(0..VARS2.len())]
}

fn generic_iterator() -> &'static str {
    static ITER: &[&str] =
        &["iter", "i", "j", "k", "e", "idx", "index", "n", "count"];

    ITER[random_range(0..ITER.len())]
}

fn generic_operator() -> &'static str {
    static OPS: &[&str] = &["<", "<=", ">=", ">", "==", "!="];
    OPS[random_range(0..OPS.len())]
}
