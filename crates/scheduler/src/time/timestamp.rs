use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Sub};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::time::{Date, DAY_IN_SECS, Time};
use crate::time::ts_components::TSComponents;

pub struct Timestamp {
    inner: u64,
    components: TSComponents,
}

impl Timestamp {
    pub fn now() -> Self {
        let now_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let components = TSComponents::from(now_ts);

        Self {
            inner: now_ts,
            components,
        }
    }

    pub fn new(ts: u64) -> Self {
        let components = TSComponents::from(ts);
        Self {
            inner: ts,
            components,
        }
    }

    pub fn as_timestamp(&self) -> u64 {
        self.inner
    }

    pub fn day_of_week(&self) -> u8 {
        (((self.inner / DAY_IN_SECS) + 4) % 7) as u8
    }

    pub(crate) fn get_year_interval(&self, period: u32) -> u64 {
        let date = Date::today().incr_year(period);
        let mut future = Self::now();
        future.set_date(&date);

        (Duration::from_secs(future.get_components().as_timestamp())
            - Duration::from_secs(self.as_timestamp()))
            .as_secs()
    }

    pub(crate) fn get_month_interval(&self, period: u8) -> u64 {
        let date = Date::today().incr_month(period);
        let mut future = Self::now();
        future.set_date(&date);

        (Duration::from_secs(future.get_components().as_timestamp())
            - Duration::from_secs(self.as_timestamp()))
            .as_secs()
    }

    // TODO: refactor, this might not wrap around, should test
    pub(crate) fn set_time(&mut self, time: &Time) {
        self.components.hour = time.hours;
        self.components.minute = time.minutes;
        if let Some(seconds) = time.seconds {
            self.components.second = seconds;
        }

        self.inner = self.components.as_timestamp();
    }

    // TODO: refactor, this might not wrap around, should test
    pub(crate) fn set_date(&mut self, date: &Date) {
        self.components.year = date.year;
        self.components.month = date.month;
        self.components.second = date.day;

        self.inner = self.components.as_timestamp();
    }

    pub(crate) fn get_components(&mut self) -> &TSComponents {
        &self.components
    }
}

impl Add<u64> for Timestamp {
    type Output = Timestamp;
    fn add(mut self, rhs: u64) -> Self::Output {
        self.inner += rhs;
        self.components = TSComponents::from(self.inner);
        self
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;
    fn add(mut self, rhs: Duration) -> Self::Output {
        self.inner += rhs.as_secs();
        self.components = TSComponents::from(self.inner);
        self
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, rhs: Duration) {
        self.inner += rhs.as_secs();
        self.components = TSComponents::from(self.inner);
    }
}


impl PartialEq<Self> for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialOrd<Self> for Timestamp {
    fn ge(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    fn le(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    fn gt(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    fn lt(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.inner < other.inner {
            return Some(Ordering::Less);
        }
        if self.inner > other.inner {
            return Some(Ordering::Greater);
        }

        None
    }
}

impl Eq for Timestamp {}

impl Ord for Timestamp {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        if self < min {
            return min;
        }

        if self > max {
            return max;
        }

        self
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.cmp(&other) {
            Ordering::Equal | Ordering::Less => other,
            _ => self
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.cmp(&other) {
            Ordering::Equal | Ordering::Greater => other,
            _ => self
        }
    }
}


#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use crate::time::{DAY_IN_SECS, WeekDay, YEAR_IN_SECS};
    use crate::time::timestamp::Timestamp;

    // Bad test, need to specify the days manually each time
    #[test]
    fn test_day_of_week() {
        let days = vec![
            (Timestamp::now(), WeekDay::Friday),
            (
                Timestamp::now() + Duration::from_secs(DAY_IN_SECS * 6),
                WeekDay::Thursday,
            ),
            (
                Timestamp::now() + Duration::from_secs(DAY_IN_SECS * 3),
                WeekDay::Monday,
            ),
            (
                Timestamp::now() + Duration::from_secs(DAY_IN_SECS * 4),
                WeekDay::Tuesday,
            ),
            (
                Timestamp::now() + Duration::from_secs(DAY_IN_SECS * 7),
                WeekDay::Friday,
            ),
            (
                Timestamp::now() + Duration::from_secs(DAY_IN_SECS * 13),
                WeekDay::Thursday,
            ),
            (
                Timestamp::now() + Duration::from_secs(YEAR_IN_SECS),
                WeekDay::Saturday,
            ),
        ];

        for (ts, day) in days {
            assert_eq!(ts.day_of_week(), day as u8)
        }
    }
}
