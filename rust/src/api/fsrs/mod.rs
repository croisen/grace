use core::f64::consts::E;

pub mod grade;

use crate::api::concept::Concept;
use crate::api::fsrs::grade::Grade;

static FSRS6: &[f64] = &[
    0.212, 1.2931, 2.3065, 8.2956, 6.4133, 0.8334, 3.0194, 0.001, 1.8722,
    0.1666, 0.796, 1.4835, 0.0614, 0.2629, 1.6483, 0.6014, 1.8729, 0.5425,
    0.0912, 0.0658, 0.1542,
];

#[flutter_rust_bridge::frb(opaque)]
pub struct Fsrs {
    pub concept: Concept,
    pub stability: f64,
    pub difficulty: f64,
    pub time: usize,
}

impl Fsrs {
    #[flutter_rust_bridge::frb(sync)]
    pub fn from_grade(concept: Concept) -> Self {
        Self { concept, stability: 1.0, difficulty: FSRS6[4], time: 1 }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn review(&mut self, grade: Grade) {
        self.stability = self.stability
            * E.powf(FSRS6[17] * ((grade - 3) as f64 + FSRS6[18]))
            * self.stability.powf(-FSRS6[19]);

        let change = -FSRS6[6] * (grade - 3) as f64;
        let dampened =
            self.difficulty + change * ((10.0 - self.difficulty) / 9.0);
        let mean_reversion = FSRS6[7] * Self::init_difficulty(Grade::Easy)
            + (1.0 - FSRS6[7]) * dampened;
        self.difficulty = mean_reversion.max(1.0);
        // self.time += 1;
    }

    // #[flutter_rust_bridge::frb(sync)]
    // pub fn difficulty(&self) -> f64 {
    //     self.difficulty
    // }

    #[flutter_rust_bridge::frb(sync)]
    pub fn retainability(&self) -> f64 {
        let factor = 0.9f64.powf(-1.0 / FSRS6[20]) - 1.0;
        (1.0 + factor * (self.time as f64 / self.stability)).powf(-FSRS6[20])
    }

    #[inline]
    fn init_difficulty(grade: Grade) -> f64 {
        FSRS6[4] - E.powf(FSRS6[5] * (grade - 1) as f64) + 1.0
    }
}
