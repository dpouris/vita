pub const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_IN_LEAP_MONTH: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const EPOCH_YEAR: u32 = 1970;
pub const WEEK_DAYS: u8 = 7;
pub const DAYS_IN_YEAR: u64 = 365;
pub const DAYS_IN_LEAP_YEAR: u64 = 366;
pub const MINUTE_IN_SECS: u64 = 60;
pub const HOUR_IN_SECS: u64 = 60 * MINUTE_IN_SECS;
pub const DAY_IN_SECS: u64 = 24 * HOUR_IN_SECS;
pub const WEEK_IN_SECS: u64 = WEEK_DAYS as u64 * DAY_IN_SECS;
pub const YEAR_IN_SECS: u64 = DAYS_IN_YEAR * DAY_IN_SECS;
pub const LEAP_YEAR_IN_SECS: u64 = DAYS_IN_LEAP_YEAR * DAY_IN_SECS;

// WeekDays
pub const MIDNIGHT: u8 = 0;
pub const MORNING: u8 = 6;
pub const AFTERNOON: u8 = 12;
pub const EVENING: u8 = 18;