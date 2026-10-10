//! Units in the types. Months, years, kg, metres and km2 are different types,
//! so adding a month count to a year count does not compile.

use std::ops::{Add, Div, Mul, Sub};

macro_rules! unit {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
        pub struct $name(pub f64);
        impl Add for $name { type Output = $name; fn add(self, o: $name) -> $name { $name(self.0 + o.0) } }
        impl Sub for $name { type Output = $name; fn sub(self, o: $name) -> $name { $name(self.0 - o.0) } }
        impl Mul<f64> for $name { type Output = $name; fn mul(self, k: f64) -> $name { $name(self.0 * k) } }
        impl Div<f64> for $name { type Output = $name; fn div(self, k: f64) -> $name { $name(self.0 / k) } }
        impl Div for $name { type Output = f64; fn div(self, o: $name) -> f64 { self.0 / o.0 } }
    };
}

unit!(/// A span of time in months (the engine step).
    Months);
unit!(/// A span of time in years.
    Years);
unit!(/// Mass in kilograms.
    Kg);
unit!(/// Length in metres.
    Metres);
unit!(/// Area in square kilometres.
    Km2);

pub const MONTHS_PER_YEAR: u64 = 12;
/// Mean Gregorian year, 365.2425 days.
pub const SECONDS_PER_YEAR: f64 = 365.2425 * 86400.0;
pub const SECONDS_PER_MONTH: f64 = SECONDS_PER_YEAR / 12.0;

impl Months { pub fn to_years(self) -> Years { Years(self.0 / 12.0) } }
impl Years { pub fn to_months(self) -> Months { Months(self.0 * 12.0) } }
impl Km2 { pub fn to_m2(self) -> f64 { self.0 * 1.0e6 } }

/// The one clock. Whole months since the start, so there is no drift.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Clock {
    pub month_index: u64,
    /// Calendar year of month 0. Negative for years before common era.
    pub start_year: i64,
}

impl Clock {
    pub fn new(start_year: i64) -> Self { Clock { month_index: 0, start_year } }
    pub fn month_of_year(&self) -> u32 { (self.month_index % MONTHS_PER_YEAR) as u32 }
    pub fn year(&self) -> i64 { self.start_year + (self.month_index / MONTHS_PER_YEAR) as i64 }
    pub fn elapsed(&self) -> Years { Months(self.month_index as f64).to_years() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversions_round_trip() {
        assert_eq!(Years(2.0).to_months(), Months(24.0));
        assert_eq!(Months(18.0).to_years(), Years(1.5));
        assert_eq!(Months(6.0) + Months(6.0), Months(12.0));
        assert_eq!(Km2(2.0).to_m2(), 2.0e6);
    }
    #[test]
    fn clock_wraps_months() {
        let mut c = Clock::new(-125000);
        c.month_index = 25;
        assert_eq!(c.month_of_year(), 1);
        assert_eq!(c.year(), -124998);
    }
}
