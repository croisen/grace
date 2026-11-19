use chrono::{Duration, Local};
use fsrs::{FSRS, MemoryState};
use rand::random_range;

use crate::api::concept::Concept;
use crate::api::drills::grade::Grade;

pub mod grade;
pub mod java;
pub mod rust;
pub mod sql;

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct DrillQuestion {
    pub concept: Concept,
    pub conditions: Vec<(bool, String)>,
    pub answer: String,
    pub ident_level: usize,
}

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(ignore_all, opaque)]
pub struct DrillCollection {
    pub name: String,
    pub collection: Vec<DrillMemory>,
}

impl DrillCollection {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(name: String, concepts: Vec<Concept>) -> Self {
        let drill = concepts.into_iter().map(DrillMemory::new).collect();
        Self { name, collection: drill }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn sort_by_retainability(&mut self) {
        let now = local_now();
        self.collection.sort_by(|a, b| {
            a.retrievability(now).total_cmp(&b.retrievability(now))
        });
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn sort_by_concept(&mut self) {
        self.collection.sort_by(|a, b| a.concept.cmp(&b.concept));
    }
}

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(ignore_all, opaque)]
pub struct DrillMemory {
    pub concept: Concept,
    pub memory: Option<MemoryState>,
    pub last_review: Option<i64>,
    pub due_date: i64,
}

impl DrillMemory {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(concept: Concept) -> Self {
        let now = local_now();
        Self { concept, memory: None, last_review: None, due_date: now }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn retrievability(&self, now: i64) -> f64 {
        let Some(memory) = &self.memory else {
            return 0.0;
        };

        let elapsed = (now - self.last_review.unwrap_or(now)).max(0) as f64;
        let elapsed_days = elapsed / 86400.0;
        (1.0 + elapsed_days / (9.0 * memory.stability as f64)).powf(-1.0)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn is_due(&self) -> bool {
        self.due_date <= Local::now().timestamp()
    }

    pub fn due_date_in_days(&self) -> i64 {
        self.due_date / 86400
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn review_now(&mut self) {
        self.due_date = local_now();
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn review_after(&mut self, duration: Duration) {
        self.due_date = (Local::now() + duration).timestamp();
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn review(&mut self, grade: Grade) {
        let fsrs = FSRS::default();
        let now = local_now();
        let elapsed = now - self.last_review.unwrap_or(now);
        let days = (elapsed as f64 / 86400.0) as u32;

        let next = fsrs.next_states(self.memory, 0.9, days).unwrap();
        let grade = match grade {
            Grade::Again => next.again,
            Grade::Hard => next.hard,
            Grade::Good => next.good,
            Grade::Easy => next.easy,
        };

        self.memory = Some(grade.memory);
        self.due_date = now + (grade.interval as i64 * 86400);
        self.last_review = Some(now);
    }
}

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

fn generic_operator() -> &'static str {
    static OPS: &[&str] = &["<", "<=", ">=", ">", "==", "!="];
    OPS[random_range(0..OPS.len())]
}
