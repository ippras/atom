#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

pub macro uncertain {
    ($start:literal..$end:literal) => {{
        let value = ($start + $end) / 2.0;
        let left = value - $start;
        let right = $end - value;
        let uncertainty = if left > right { left } else { right };
        Uncertain::new(value, uncertainty)
    }},
    ($value:literal, $uncertainty:literal) => {
        Uncertain::new($value, $uncertainty)
    },
    ($value:literal) => {
        Uncertain::new($value, 0.0)
    }
}

/// Uncertain
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Uncertain {
    pub value: f64,
    pub uncertainty: f64,
}

impl Uncertain {
    pub const fn new(value: f64, uncertainty: f64) -> Self {
        Self { value, uncertainty }
    }

    pub const fn start(&self) -> f64 {
        self.value - self.uncertainty
    }

    pub const fn end(&self) -> f64 {
        self.value + self.uncertainty
    }
}

impl Add for Uncertain {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            uncertainty: self.uncertainty + rhs.uncertainty,
        }
    }
}

impl Add<f64> for Uncertain {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value + rhs,
            uncertainty: self.uncertainty,
        }
    }
}

impl Div for Uncertain {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
            uncertainty: self.uncertainty + rhs.uncertainty,
        }
    }
}

impl Div<f64> for Uncertain {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value / rhs,
            uncertainty: self.uncertainty,
        }
    }
}

impl Display for Uncertain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " ± {}", self.uncertainty)?;
        Ok(())
    }
}

impl Eq for Uncertain {}

impl Mul for Uncertain {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            uncertainty: self.uncertainty + rhs.uncertainty,
        }
    }
}

impl Mul<f64> for Uncertain {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value * rhs,
            uncertainty: self.uncertainty,
        }
    }
}

impl Ord for Uncertain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.total_cmp(&other.value)
    }
}

impl Sub for Uncertain {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            uncertainty: self.uncertainty + rhs.uncertainty,
        }
    }
}

impl Sub<f64> for Uncertain {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value - rhs,
            uncertainty: self.uncertainty,
        }
    }
}
