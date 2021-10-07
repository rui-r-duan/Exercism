// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut success_rate: f64 = 1.0;
    if speed >= 1 && speed <= 4 {
        success_rate = 1.0
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9
    } else if speed == 9 || speed == 10 {
        success_rate = 0.77
    }
    let cars: f64 = (speed as u32 * 221) as f64;

    cars * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let mut success_rate: f64 = 1.0;
    if speed >= 1 && speed <= 4 {
        success_rate = 1.0
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9
    } else if speed == 9 || speed == 10 {
        success_rate = 0.77
    }
    let cars: f64 = (speed as u32 * 221) as f64 / 60.0;

    (cars * success_rate) as u32
}
