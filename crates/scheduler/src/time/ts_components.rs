use crate::time::{DAY_IN_SECS, DAYS_IN_LEAP_MONTH, DAYS_IN_MONTH, EPOCH_YEAR, HOUR_IN_SECS, LEAP_YEAR_IN_SECS, MINUTE_IN_SECS, YEAR_IN_SECS};

pub(crate) struct TSComponents {
    pub(crate) year: u32,
    pub(crate) month: u8,
    pub(crate) day: u8,
    pub(crate) hour: u8,
    pub(crate) minute: u8,
    pub(crate) second: u8,
}

impl TSComponents {
    pub fn as_timestamp(&self) -> u64 {
        let mut timestamp = 0;
        for year in EPOCH_YEAR..self.year {
            timestamp += Self::is_leap(year).then_some(LEAP_YEAR_IN_SECS).unwrap_or(YEAR_IN_SECS);
        }
        for month in 1..self.month {
            timestamp += DAY_IN_SECS * Self::days_in_month(self.year, month);
        }
        timestamp += DAY_IN_SECS * (self.day as u64 - 1);
        timestamp += HOUR_IN_SECS * self.hour as u64;
        timestamp += MINUTE_IN_SECS * self.minute as u64;
        timestamp += self.second as u64;

        timestamp
    }

    fn extract_year(ts: &mut u64) -> u32 {
        let mut year = EPOCH_YEAR as u64;
        let mut seconds_in_year: u64;

        let cycles = *ts / (400 * YEAR_IN_SECS + 97 * DAY_IN_SECS); // number of complete 400 year cycles
        year += cycles * 400;
        *ts %= 400 * YEAR_IN_SECS + 97 * DAY_IN_SECS;


        let centuries = *ts / (100 * YEAR_IN_SECS + 24 * DAY_IN_SECS); // number of complete 100 year cycles
        year += centuries * 100;
        *ts %= 100 * YEAR_IN_SECS + 24 * DAY_IN_SECS;

        let quadrennial = *ts / (4 * YEAR_IN_SECS + DAY_IN_SECS); // number of complete 4 year cycles
        year += quadrennial * 4;
        *ts %= 4 * YEAR_IN_SECS + DAY_IN_SECS;

        // add the remaining years
        while {
            seconds_in_year = Self::is_leap(year as u32).then_some(LEAP_YEAR_IN_SECS).unwrap_or(YEAR_IN_SECS);
            *ts >= seconds_in_year
        } {
            *ts -= seconds_in_year;
            year += 1;
        }

        year as u32
    }

    fn extract_month_and_day(mut ts: u64, year: u32) -> (u8, u8) {
        let days_in_month = Self::is_leap(year).then_some(DAYS_IN_LEAP_MONTH).unwrap_or(DAYS_IN_MONTH);
        let mut month = 0;
        while ts >= days_in_month[month] * DAY_IN_SECS {
            ts -= days_in_month[month] * DAY_IN_SECS;
            month += 1;
        }
        ((month + 1) as u8, (ts / DAY_IN_SECS + 1) as u8)
    }

    fn time_from_secs(ts: u64) -> (u8, u8, u8) {
        (
            ((ts % DAY_IN_SECS) / HOUR_IN_SECS) as u8,
            ((ts % DAY_IN_SECS) % HOUR_IN_SECS / MINUTE_IN_SECS) as u8,
            ((ts % DAY_IN_SECS) % HOUR_IN_SECS % MINUTE_IN_SECS) as u8,
        )
    }

    fn is_leap(year: u32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    fn days_in_month(year: u32, month: u8) -> u64 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => Self::is_leap(year).then_some(29).unwrap_or(28),
            _ => 0
        }
    }
}


impl From<u64> for TSComponents {
    fn from(mut value: u64) -> Self {
        let year = Self::extract_year(&mut value); // should consider the case the value is negative
        let (month, day) = Self::extract_month_and_day(value, year);
        let time = Self::time_from_secs(value);

        TSComponents {
            day,
            month,
            year,
            second: time.2,
            minute: time.1,
            hour: time.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rand::Rng;

    use crate::time::ts_components::TSComponents;

    macro_rules! gen_timestamps {
        ($($ts:expr, ($year:expr, $month:expr, $day:expr)),+) => {
            [
                $(
                ($ts, TSComponents::test($year, $month, $day))
                ),+
            ]
        };
    }

    impl TSComponents {
        pub(crate) fn test(year: u32, month: u8, day: u8) -> Self {
            Self {
                year,
                month,
                day,
                hour: 0,
                minute: 0,
                second: 0,
            }
        }
    }

    fn gen_timestamps() -> Vec<(u64, (u32, u32, u32))> {
        let mut rng = rand::thread_rng();
        let mut timestamps: Vec<(u64, (u32, u32, u32))> = Vec::with_capacity(100);
        for _ in 0..1000 {
            let year: i32 = rng.gen_range(1970..2038);
            let month: u32 = rng.gen_range(1..13);
            let day: u32 = rng.gen_range(1..29);

            let date_string = format!("{:04}-{:02}-{:02} 00:00:00", year, month, day);
            let date = chrono::NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S")
                .expect("Failed to parse date");
            let timestamp = date.and_utc().timestamp();

            timestamps.push((timestamp as u64, (year as u32, month, day)));
        }

        timestamps
    }

    #[test]
    fn test_timestamp() {
        let start = Instant::now();
        let ts = TSComponents::from(253389174348);
        println!("Took: {}ns", start.elapsed().as_nanos());

        let timestamps = gen_timestamps();
        let start = Instant::now();
        for (timestamp, expected) in timestamps {
            let ts = TSComponents::from(timestamp);
            assert_eq!(ts.year, expected.0);
            assert_eq!(ts.month as u32, expected.1);
            assert_eq!(ts.day as u32, expected.2);
        }
        println!("Took: {}μs", start.elapsed().as_micros());
    }
}