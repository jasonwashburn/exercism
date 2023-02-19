use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let hours = self.hours;
        let minutes = self.minutes;
        write!(f, "{}", format!("{hours:02}:{minutes:02}"))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
impl Eq for Clock {}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: hours % 24,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: self.minutes + minutes,
            ..*self
        }
    }
}
