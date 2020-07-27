#![allow(dead_code)]

use std::time::Duration;

pub const FLICK: f64 = 1.0 / FLICKS_PER_SEC as f64;
pub const FLICKS_PER_NANO: f64 = FLICKS_PER_SEC as f64 / 1000000000 as f64;
pub const FLICKS_PER_MICRO: f64 = FLICKS_PER_SEC as f64 / 1000000 as f64;
pub const FLICKS_PER_MILLI: f64 = FLICKS_PER_SEC as f64 / 1000 as f64;
pub const FLICKS_PER_SEC: u128 = 705600000;
pub const FLICKS_PER_MIN: u128 = FLICKS_PER_SEC * 60;
pub const FLICKS_PER_HOUR: u128 = FLICKS_PER_MIN * 60;

pub fn from_flicks_lossy(flicks: f64) -> Duration {
    return Duration::from_nanos((flicks / FLICKS_PER_NANO) as u64);
}

pub fn to_flicks_lossy(dur: Duration) -> f64 {
    return dur.as_nanos() as f64 * FLICKS_PER_NANO;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_values() {
        println!("{}", FLICK);
    }
    #[test]
    fn check_to_flicks_lossy_1_sec() {
        assert_eq!(FLICKS_PER_SEC as f64, to_flicks_lossy(std::time::Duration::from_secs(1)));
    }
    #[test]
    fn check_to_flicks_lossy_1_milli() {
        assert_eq!(FLICKS_PER_MILLI, to_flicks_lossy(std::time::Duration::from_millis(1)));
    }
    #[test]
    fn check_to_flicks_lossy_1_micro() {
        assert_eq!(FLICKS_PER_MICRO, to_flicks_lossy(std::time::Duration::from_micros(1)));
    }
    #[test]
    fn check_to_flicks_lossy_1_nano() {
        assert_eq!(FLICKS_PER_NANO, to_flicks_lossy(std::time::Duration::from_nanos(1)));
    }
}
