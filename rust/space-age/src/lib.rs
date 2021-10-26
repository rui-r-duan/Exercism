#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        const SECONDS_EARTH_YEAR: f64 = 31_557_600.0;
        Self {
            years: s as f64 / SECONDS_EARTH_YEAR,
        }
    }
}

pub trait Planet {
    const EARTH_YEAR_RATIO: f64;
    fn years_during(d: &Duration) -> f64 {
        d.years / Self::EARTH_YEAR_RATIO
    }
}

#[macro_export]
macro_rules! defplanet {
    ($planet_name:ident, $earth_year_ratio:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            const EARTH_YEAR_RATIO: f64 = $earth_year_ratio;
        }
    };
}

defplanet!(Mercury, 0.2408467);
defplanet!(Venus, 0.61519726);
defplanet!(Earth, 1.0);
defplanet!(Mars, 1.8808158);
defplanet!(Jupiter, 11.862615);
defplanet!(Saturn, 29.447498);
defplanet!(Uranus, 84.016846);
defplanet!(Neptune, 164.79132);
