// WeekDay starts from Sunday through Saturday (mimics cron):
// The time and date fields are:

// field          allowed values
// -----          --------------
// ...
// day of week    0-6 (0 is Sunday, or use names)

use std::cmp::Ordering;

use crate::time::{DAY_IN_SECS, Step, Time, WEEK_DAYS};
use crate::time::timespan::TimeSpan;
use crate::time::timestamp::Timestamp;

#[derive(Copy, Clone)]
pub enum WeekDay {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl WeekDay {
    // TODO: these methods might have a timezone bug, check it out
    pub fn midnight(self) -> TimeSpan {
        let mut ts = self.at("00:00").unwrap(); // it's safe because we know that the time_str is parsable
        let until = Self::days_until(self as u8);
        ts.start_in.replace(until as u64 * DAY_IN_SECS);
        ts
    }

    pub fn morning(self) -> TimeSpan {
        let mut ts = self.at("06:00").unwrap(); // it's safe because we know that the time_str is parsable
        let until = Self::days_until(self as u8);
        ts.start_in.replace(until as u64 * DAY_IN_SECS);
        ts
    }

    pub fn afternoon(self) -> TimeSpan {
        let mut ts = self.at("12:00").unwrap(); // it's safe because we know that the time_str is parsable
        let until = Self::days_until(self as u8);
        ts.start_in.replace(until as u64 * DAY_IN_SECS);
        ts
    }

    // TODO: theres a bug that if the day that the schedule is supposed to start is exactly one week until execution it wont take into account the time of the day
    // we should consider that and if the hour is still after ts.now() then start it then
    pub fn evening(self) -> TimeSpan {
        let mut ts = self.at("18:00").unwrap(); // it's safe because we know that the time_str is parsable
        let until = Self::days_until(self as u8);
        ts.start_in.replace(until as u64 * DAY_IN_SECS);
        ts
    }

    pub fn at(self, time_str: &str) -> Result<TimeSpan, ()> {
        let time_obj = match Time::from_time_str(time_str) {
            Ok(t) => t,
            Err(msg) => return Err(msg),
        };
        Ok(TimeSpan {
            interval: WEEK_DAYS as u64 * DAY_IN_SECS,
            start_in: None,
            start_at: Some(time_obj),
            step: Step::Weekday,
        })
    }

    fn days_until(day: u8) -> u8 {
        let today = Timestamp::now().day_of_week();
        match today.cmp(&day) {
            Ordering::Less => day - today,
            Ordering::Greater => 7 - today + day,
            Ordering::Equal => 7,
        }
    }
}
