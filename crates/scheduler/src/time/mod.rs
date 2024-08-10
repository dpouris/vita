pub use consts::*;
pub use timespan::*;
pub use weekday::*;

use crate::time::timestamp::Timestamp;

mod consts;
mod timespan;
pub mod timestamp;
mod weekday;

mod ts_components;
mod ts_components_legacy;

/// Time is represented in the 24hour clock e.g. 13:43
pub(crate) struct Time {
    hours: u8,
    minutes: u8,
    seconds: Option<u8>,
}

pub(crate) struct Date {
    year: u32,
    month: u8,
    day: u8,
}

impl Time {
    pub(crate) fn from_time_str(time_str: &str) -> Result<Time, ()> {
        let [Some(hours), Some(minutes)] = time_str
            .split(':')
            .map(|t: &str| t.parse::<u8>().ok())
            .collect::<Vec<Option<u8>>>()[..]
        else {
            return Err(());
        };

        Ok(Time {
            hours,
            minutes,
            seconds: None,
        })
    }
}

impl Date {
    pub(crate) fn today() -> Self {
        let mut today = Timestamp::now();
        let components = today.get_components();
        Self {
            day: components.day,
            month: components.month,
            year: components.year,
        }
    }

    pub(crate) fn incr_year(mut self, year: u32) -> Self {
        self.year += year;
        self
    }
    pub(crate) fn incr_month(mut self, month: u8) -> Self {
        self.month += month;
        self
    }
    pub(crate) fn incr_day(mut self, day: u8) -> Self {
        self.day += day;
        self
    }
}
