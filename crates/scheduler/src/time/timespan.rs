use std::cmp::Ordering;
use std::ops::{Add, Mul};
use std::time::Duration;

use crate::time::{DAY_IN_SECS, HOUR_IN_SECS, MINUTE_IN_SECS, Time, WEEK_IN_SECS};
use crate::time::timestamp::Timestamp;

pub(crate) enum Step {
    Year(u32),
    Month(u8),
    Weekday,
    Second,
}

pub struct TimeSpan {
    /// Interval represents the time in seconds that needs to elapse before the event occurs
    pub(crate) interval: u64,
    /// start_at specifies the actual time the schedule needs to start every instance
    pub(crate) start_at: Option<Time>,
    /// start_in specifies how much time in seconds needs to elapse, before its scheduled execution interval
    pub(crate) start_in: Option<u64>,
    pub(crate) step: Step,
}

impl TimeSpan {
    pub(crate) fn next_run_on(&mut self) -> Timestamp {
        let start_in = self.start_in.take();
        let mut next_timestamp = Timestamp::now();
        match &self.step {
            Step::Month(period) => {
                next_timestamp += Duration::from_secs(next_timestamp.get_month_interval(*period));
            }
            Step::Year(period) => {
                next_timestamp += Duration::from_secs(next_timestamp.get_year_interval(*period));
            }
            _ => {
                next_timestamp += Duration::from_secs(start_in.unwrap_or(self.interval));
            }
        };
        if let Some(ref start_at) = self.start_at {
            next_timestamp.set_time(start_at);
        }

        next_timestamp
    }

    // TODO: this needs refactoring. We need to take more things into account instead of just incrementing the self.interval by "span"
    pub fn and(mut self, span: TimeSpan) -> Self {
        self.interval += span.interval;
        self
    }

    // TODO: we have the same timezone problem as with the .midnight etc methods on Weekday
    /// `at` is usually called as:
    /// ```
    /// use scheduler::{Scheduler, time::AsTimeSpan};
    /// let mut scheduler = Scheduler::with_tz("UTC+3").unwrap();
    ///
    /// scheduler
    ///     .every(1.day().at("10:00")?.am().unwrap())
    ///     .perform(|| println!("hello!"));
    /// ```
    pub fn at(mut self, time_str: &'static str) -> Result<Self, ()> {
        // Default is pm, call .am()
        let time_obj = match Time::from_time_str(time_str) {
            Ok(t) => t,
            Err(msg) => return Err(msg),
        };
        self.start_at.replace(time_obj);
        Ok(self)
    }

    // TODO: major refactoring here.
    // We need to take into account timezones and the fact that the SystemTime::now() call returns a timestamp referring to the timezone of the computer, not GMT
    // Everything should, by default, be GMT time and when timezones are implemented, then we can go from there
    pub fn am(mut self) -> Result<Self, ()> {
        if let Some(ref mut time) = self.start_at.as_mut() {
            match (time.hours + 12).cmp(&24) {
                Ordering::Equal => {
                    time.hours = 0;
                }
                Ordering::Less => {
                    time.hours += 12;
                }
                Ordering::Greater => return Err(()),
            }
        }
        return Ok(self);
    }

    /// # Examples:
    /// ```
    /// use scheduler::{Scheduler, time::AsTimeSpan};
    /// let mut scheduler = Scheduler::with_tz("UTC+3").unwrap();
    ///
    /// scheduler
    ///     .every(1.month().on_the(28)) // example: Current month is December, this will next run on January 28th
    ///     .perform(|| println!("hello!"));
    /// ```
    pub fn on_the(mut self, month_day: u8) -> Self {
        unimplemented!()
    }
}

impl Add for TimeSpan {
    type Output = TimeSpan;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.interval += rhs.interval;
        self
    }
}

pub trait AsTimeSpan
where
    Self: Mul<u64>,
{
    fn second(&self) -> TimeSpan;

    fn minute(&self) -> TimeSpan;

    fn hour(&self) -> TimeSpan;

    fn day(&self) -> TimeSpan;

    fn week(&self) -> TimeSpan;

    fn month(&self) -> TimeSpan;

    fn year(&self) -> TimeSpan;
}

impl AsTimeSpan for u64 {
    fn second(&self) -> TimeSpan {
        TimeSpan {
            interval: *self,
            start_at: None,
            start_in: None,
            step: Step::Second,
        }
    }

    fn minute(&self) -> TimeSpan {
        TimeSpan {
            interval: *self * MINUTE_IN_SECS,
            start_at: None,
            start_in: None,
            step: Step::Second,
        }
    }

    fn hour(&self) -> TimeSpan {
        TimeSpan {
            interval: *self * HOUR_IN_SECS,
            start_at: None,
            start_in: None,
            step: Step::Second,
        }
    }

    fn day(&self) -> TimeSpan {
        TimeSpan {
            interval: *self * DAY_IN_SECS,
            start_at: None,
            start_in: None,
            step: Step::Second,
        }
    }

    fn week(&self) -> TimeSpan {
        TimeSpan {
            interval: *self * WEEK_IN_SECS,
            start_at: None,
            start_in: None,
            step: Step::Second,
        }
    }

    //TODO: .month and .year need major refactor im not sure about their behaviour
    fn month(&self) -> TimeSpan {
        // let interval = Timestamp::now().get_month_interval(*self as u8);
        TimeSpan {
            interval: 0,
            start_at: None,
            start_in: None,
            step: Step::Month(*self as u8),
        }
    }

    fn year(&self) -> TimeSpan {
        // let interval = Timestamp::now().get_year_interval(*self as u32);
        TimeSpan {
            interval: 0,
            start_at: None,
            start_in: None,
            step: Step::Year(*self as u32),
        }
    }
}
