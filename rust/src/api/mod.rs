use chrono::Local;
use fsrs::{FSRS, MemoryState};
use rand::random_range;

use crate::api::grade::Grade;

pub mod concept;
pub mod diagnostics;
pub mod grade;

pub mod java;
pub mod rust;
pub mod sql;

pub static DESIRED_RETENTION: f32 = 0.9;

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(ignore_all, opaque)]
pub struct DrillCollection {
    pub name: String,
    pub collection: Vec<DrillMemory>,
}

impl DrillCollection {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(name: String, list: Vec<(DrillLanguage, String)>) -> Self {
        let drill =
            list.into_iter().map(|(l, c)| DrillMemory::new(l, c)).collect();

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

#[derive(Debug, Clone)]
#[flutter_rust_bridge::frb(ignore_all, opaque)]
pub struct DrillMemory {
    pub lang: DrillLanguage,
    pub concept: String,
    pub memory: Option<MemoryState>,
    pub last_review: Option<i64>,
    pub due_date: i64,
}

impl DrillMemory {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(lang: DrillLanguage, concept: String) -> Self {
        let now = local_now();
        Self { lang, concept, memory: None, last_review: None, due_date: now }
    }

    pub fn lang(&self) -> DrillLanguage {
        self.lang
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn retrievability(&self, now: i64) -> f32 {
        let fsrs = FSRS::default();
        let elapsed = now - self.last_review.unwrap_or(now);
        let days = elapsed as f32 / 86400.0;
        self.memory.map(|m| fsrs.current_retrievability(m, days)).unwrap_or(0.0)
    }

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

    #[flutter_rust_bridge::frb(sync)]
    pub fn review(&mut self, grade: Grade) {
        self._review(grade);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn review_if_due(&mut self, grade: Grade) {
        if self.is_due() {
            self._review(grade);
        }
    }

    #[inline]
    fn _review(&mut self, grade: Grade) {
        let fsrs = FSRS::default();
        let now = local_now();
        let elapsed = now - self.last_review.unwrap_or(now);
        let days = elapsed as f32 / 86400.0;

        let next = fsrs
            .next_states_with_elapsed_days(self.memory, DESIRED_RETENTION, days)
            .unwrap();

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
