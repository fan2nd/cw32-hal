//! Validated Gregorian calendar values for the hardware's 2000–2099 epoch.

/// Day of the week. The CW32 register encoding is Sunday = 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

/// A calendar value cannot be represented by this driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateTimeError {
    InvalidYear,
    InvalidMonth,
    InvalidDay,
    InvalidDayOfWeek,
    InvalidHour,
    InvalidMinute,
    InvalidSecond,
    InvalidBcd,
}

/// A validated 24-hour calendar value, at whole-second resolution.
///
/// The two-digit hardware year is interpreted as 2000–2099. There is no time
/// zone, leap-second or century tracking in hardware or in this value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    day_of_week: DayOfWeek,
    hour: u8,
    minute: u8,
    second: u8,
}
impl DateTime {
    /// Validate a Gregorian date/time and calculate its weekday.
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, DateTimeError> {
        if !(2000..=2099).contains(&year) {
            return Err(DateTimeError::InvalidYear);
        }
        let days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if year % 4 == 0 => 29,
            2 => 28,
            _ => return Err(DateTimeError::InvalidMonth),
        };
        if day == 0 || day > days {
            return Err(DateTimeError::InvalidDay);
        }
        if hour > 23 {
            return Err(DateTimeError::InvalidHour);
        }
        if minute > 59 {
            return Err(DateTimeError::InvalidMinute);
        }
        if second > 59 {
            return Err(DateTimeError::InvalidSecond);
        }
        // 2000-01-01 was Saturday; count complete years/months in this epoch.
        let y = u32::from(year - 2000);
        let mut elapsed = y * 365 + (y + 3) / 4;
        const BEFORE_MONTH: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        elapsed += u32::from(BEFORE_MONTH[usize::from(month - 1)]) + u32::from(day - 1);
        if month > 2 && year % 4 == 0 {
            elapsed += 1;
        }
        let day_of_week = weekday(((elapsed + 6) % 7) as u8)?;
        Ok(Self {
            year,
            month,
            day,
            day_of_week,
            hour,
            minute,
            second,
        })
    }
    /// Embassy-style constructor with an explicit, checked weekday.
    pub fn from(
        year: u16,
        month: u8,
        day: u8,
        day_of_week: DayOfWeek,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, DateTimeError> {
        let value = Self::new(year, month, day, hour, minute, second)?;
        if value.day_of_week != day_of_week {
            return Err(DateTimeError::InvalidDayOfWeek);
        }
        Ok(value)
    }
    pub const fn year(&self) -> u16 {
        self.year
    }
    pub const fn month(&self) -> u8 {
        self.month
    }
    pub const fn day(&self) -> u8 {
        self.day
    }
    pub const fn day_of_week(&self) -> DayOfWeek {
        self.day_of_week
    }
    pub const fn hour(&self) -> u8 {
        self.hour
    }
    pub const fn minute(&self) -> u8 {
        self.minute
    }
    pub const fn second(&self) -> u8 {
        self.second
    }

    pub(super) fn date_bits(self) -> u32 {
        (self.day_of_week as u32) << 24
            | u32::from(bcd((self.year - 2000) as u8)) << 16
            | u32::from(bcd(self.month)) << 8
            | u32::from(bcd(self.day))
    }
    pub(super) fn time_bits(self) -> u32 {
        u32::from(bcd(self.hour)) << 16
            | u32::from(bcd(self.minute)) << 8
            | u32::from(bcd(self.second))
    }
    pub(super) fn from_bits(date: u32, time: u32) -> Result<Self, DateTimeError> {
        Self::from(
            2000 + u16::from(unbcd((date >> 16) as u8)?),
            unbcd(((date >> 8) & 0x1f) as u8)?,
            unbcd((date & 0x3f) as u8)?,
            weekday(((date >> 24) & 7) as u8)?,
            unbcd(((time >> 16) & 0x3f) as u8)?,
            unbcd(((time >> 8) & 0x7f) as u8)?,
            unbcd((time & 0x7f) as u8)?,
        )
    }
}
fn weekday(value: u8) -> Result<DayOfWeek, DateTimeError> {
    match value {
        0 => Ok(DayOfWeek::Sunday),
        1 => Ok(DayOfWeek::Monday),
        2 => Ok(DayOfWeek::Tuesday),
        3 => Ok(DayOfWeek::Wednesday),
        4 => Ok(DayOfWeek::Thursday),
        5 => Ok(DayOfWeek::Friday),
        6 => Ok(DayOfWeek::Saturday),
        _ => Err(DateTimeError::InvalidDayOfWeek),
    }
}
const fn bcd(value: u8) -> u8 {
    ((value / 10) << 4) | (value % 10)
}
fn unbcd(value: u8) -> Result<u8, DateTimeError> {
    if value & 15 > 9 || value >> 4 > 9 {
        return Err(DateTimeError::InvalidBcd);
    }
    Ok((value >> 4) * 10 + (value & 15))
}
