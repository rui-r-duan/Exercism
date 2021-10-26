// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    age_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            age_years: s as f64 / 31557600 as f64,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 0.2408467;

        d.age_years / R
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 0.61519726;

        d.age_years / R
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.age_years
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 1.8808158;

        d.age_years / R
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 11.862615;

        d.age_years / R
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 29.447498;

        d.age_years / R
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 84.016846;

        d.age_years / R
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        const R: f64 = 164.79132;

        d.age_years / R
    }
}
