use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    display_hours: i32,
    display_minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:02}:{:02}", self.display_hours, self.display_minutes)
    }
}

impl Clock {
    const ONE_HOUR: i32 = 60;
    
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (normalized_hours, normalized_minutes, minutes) = Clock::normalize_time(hours * Clock::ONE_HOUR + minutes);

        Clock {
            display_hours: normalized_hours,
            display_minutes: normalized_minutes,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (normalized_hours, normalized_minutes, new_minutes) = Clock::normalize_time(self.minutes + minutes);

        Clock {
            display_hours: normalized_hours,
            display_minutes: normalized_minutes,
            minutes: new_minutes,
        }
   }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.display_hours, self.display_minutes)
    }

    fn normalize_time(minutes: i32) -> (i32, i32, i32) {
        const ONE_DAY_IN_MIN: i32 = 1440;
        let new_minutes: i32;

        if minutes < 0 {
            new_minutes = ONE_DAY_IN_MIN - (-minutes % ONE_DAY_IN_MIN);
        } else {
            new_minutes = minutes % ONE_DAY_IN_MIN;
        }

        let normalized_hours = new_minutes / Clock::ONE_HOUR;
        let normalized_minutes = new_minutes % Clock::ONE_HOUR;

        (normalized_hours, normalized_minutes, new_minutes)
    }
}
