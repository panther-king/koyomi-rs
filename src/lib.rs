//! # A library for handling traditional Japanese customs.
//!
//! This library handles various information related to dates based on
//! Japanese unique customs and practices.
//!
//! This library heavily relies on the [`chrono`] for various caluculation and comparisions.
//!
//! ## Overview
//!
//! ### Year
//!
//! In Japan, in addition to Gregorian calendar, the Japanese era system is also used.
//!
//! Using [`JapaneseEra`], it is possible to derive the Japanese era from the Gregorian calendar.
//!
//! Please note that it is not possible to derive Japanese eras before the Meiji era.
//! Prior to the Meiji era, the derivation is based on the lunar calendar, which is not currentry supported.
//!
//! ### Month
//!
//! In Japan, there are unique names for months similar to how _January_ is for first month
//! and _Febrary_ for second months in English.
//!
//! By using [`JapaneseMonth`], it is possible to derive these.
//!
//! ### Weekday
//!
//! Similarly to months, there are unique names for weekday in Japanese.
//!
//! By using [`JapaneseWeekday`], it is possible to derive these.
//!
//! ### Day and holiday
//!
//! The representation of days itself is not specifically supported by this library.
//! However, due to the numerous unique Japanese holidays, this is supported in the library.
//!
//! By using [`JapaneseHoliday`], it is possible to derive these.
//!
//! ### Calendar
//!
//! It supports generating calendars using each of the above,
//! including both common Gregorian dates and Japanese-specific expression.
//!
//! By using [`Koyomi`], you can generate calendars.
mod day;
pub use day::{JapaneseHoliday, JapaneseWeekday};

mod era;
pub use era::JapaneseEra;

mod internal;

mod koyomi;
pub use koyomi::{JapaneseDate, Koyomi};

mod month;
pub use month::JapaneseMonth;

mod year;
pub use year::{HeavenlyStem, JapaneseZodiac, SexagenaryCycle};

pub mod prelude {
    pub use crate::day::{JapaneseHoliday, JapaneseWeekday};
    pub use crate::era::JapaneseEra;
    pub use crate::koyomi::{JapaneseDate, Koyomi};
    pub use crate::month::JapaneseMonth;
    pub use crate::year::{HeavenlyStem, JapaneseZodiac, SexagenaryCycle};
}
