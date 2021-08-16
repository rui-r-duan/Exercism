use std::fmt;
const MINUTES_PER_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let modulo = total_minutes.rem_euclid(MINUTES_PER_DAY);

        Clock { minutes: modulo }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let t = self.minutes + minutes;
        let modulo = t.rem_euclid(MINUTES_PER_DAY);

        Clock { minutes: modulo }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / 60,
            self.minutes.rem_euclid(60)
        )
    }
}
