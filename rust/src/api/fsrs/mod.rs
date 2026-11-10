use core::f64::consts::E;
use std::ops::Sub;

static FSRS6: &[f64] = &[
    0.212, 1.2931, 2.3065, 8.2956, 6.4133, 0.8334, 3.0194, 0.001, 1.8722,
    0.1666, 0.796, 1.4835, 0.0614, 0.2629, 1.6483, 0.6014, 1.8729, 0.5425,
    0.0912, 0.0658, 0.1542,
];

#[flutter_rust_bridge::frb(sync)]
pub struct FSRS {
    pub concept: Concept,
    pub stability: f64,
    pub difficulty: f64,
    pub time: usize, // more like attempts as the formula used is for same day recall
}

impl FSRS {
    #[inline]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(concept: Concept) -> Self {
        Self { concept, stability: 1.0, difficulty: FSRS6[4], time: 1 }
    }

    #[inline]
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

    #[inline]
    #[flutter_rust_bridge::frb(sync)]
    pub fn retainability(&self) -> f64 {
        let factor = 0.9f64.powf(-1.0 / FSRS6[20]) - 1.0;
        (1.0 + factor * (self.time as f64 / self.stability)).powf(-FSRS6[20])
    }

    #[inline]
    pub fn clear_attempts(&mut self) {
        self.time = 0;
    }

    #[inline]
    fn init_difficulty(grade: Grade) -> f64 {
        FSRS6[4] - E.powf(FSRS6[5] * (grade - 1) as f64) + 1.0
    }
}

pub enum Concept {
    VarDecl,
    FunctionDecl,
    Ifs,
    ElseIfs,
    ForLoop,
    WhileLoop,
    Switch,
}

#[derive(Debug, Copy, Clone)]
pub enum Grade {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

impl Sub<usize> for Grade {
    type Output = isize;

    fn sub(self, rhs: usize) -> Self::Output {
        (self as isize).wrapping_sub_unsigned(rhs)
    }
}
