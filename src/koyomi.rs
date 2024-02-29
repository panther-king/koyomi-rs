use std::iter::Iterator;

use crate::day::{JapaneseHoliday, JapaneseWeekday};
use crate::era::JapaneseEra;
use crate::month::JapaneseMonth;
use crate::year::{HeavenlyStem, JapaneseZodiac, SexagenaryCycle};

use chrono::{Datelike, Local, Months, NaiveDate};

/// Japanese date
///
/// It includes dates with Japanese-specific definitions as well.
/// You can generate it independently, but it's also intended to be used as a [`Koyomi`] item.
#[derive(Debug, Eq, PartialEq)]
pub struct JapaneseDate {
    day: u32,
    era: Option<JapaneseEra>,
    heavenly_stem: HeavenlyStem,
    holiday: Option<JapaneseHoliday>,
    month: JapaneseMonth,
    month_number: u32,
    sexagenary_cycle: SexagenaryCycle,
    weekday: JapaneseWeekday,
    western_year: i32,
    zodiac: JapaneseZodiac,
}

impl JapaneseDate {
    /// Generate from Datelike of chrono.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert!(japanese_date.is_holiday());
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Self {
        Self {
            day: date.day(),
            era: JapaneseEra::from_datelike(date),
            heavenly_stem: HeavenlyStem::from_datelike(date),
            holiday: JapaneseHoliday::holiday(date),
            month: JapaneseMonth::from_datelike(date),
            month_number: date.month(),
            sexagenary_cycle: SexagenaryCycle::from_datelike(date),
            weekday: JapaneseWeekday::from_datelike(date),
            western_year: date.year(),
            zodiac: JapaneseZodiac::from_datelike(date),
        }
    }

    /// Returns the day.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(29, japanese_date.day());
    /// ```
    pub const fn day(&self) -> u32 {
        self.day
    }

    /// Returns the `JapaneseEra`.
    /// Note: Supported Reiwa, Heisei, Showa, Taisho and Meiji.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, JapaneseEra};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(JapaneseEra::Reiwa(6), japanese_date.era().unwrap());
    /// ```
    pub const fn era(&self) -> Option<JapaneseEra> {
        self.era
    }

    /// Returns the holiday or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 2, 23).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// // Reiwa emperor's birthday
    /// assert!(japanese_date.is_holiday());
    /// ```
    pub const fn is_holiday(&self) -> bool {
        self.holiday.is_some()
    }

    /// Returns the name of Japanese era.
    /// Note: Supported Reiwa, Heisei, Showa, Taisho and Meiji.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("令和", japanese_date.era_name().unwrap());
    /// ```
    pub fn era_name(&self) -> Option<&'static str> {
        self.era.map(|e| e.name())
    }

    /// Returns the `HeavenlyStem`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, HeavenlyStem};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(HeavenlyStem::Kinoe, japanese_date.heavenly_stem());
    /// ```
    pub const fn heavenly_stem(&self) -> HeavenlyStem {
        self.heavenly_stem
    }

    /// Returns the name of heavenly stem.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("甲", japanese_date.heavenly_stem_name());
    /// ```
    pub const fn heavenly_stem_name(&self) -> &'static str {
        self.heavenly_stem.name()
    }

    /// Returns the `JapaneseHoliday`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, JapaneseHoliday};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(JapaneseHoliday::NewYearsDay, japanese_date.holiday().unwrap());
    /// ```
    pub const fn holiday(&self) -> Option<JapaneseHoliday> {
        self.holiday
    }

    /// Returns the name of Japanese holiday.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("元日", japanese_date.holiday_name().unwrap());
    /// ```
    pub fn holiday_name(&self) -> Option<&'static str> {
        self.holiday.map(|h| h.name())
    }

    /// Returns the `JapaneseMonth`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, JapaneseMonth};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(JapaneseMonth::Kisaragi, japanese_date.month());
    /// ```
    pub const fn month(&self) -> JapaneseMonth {
        self.month
    }

    /// Returns the name of Japanese month.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("如月", japanese_date.month_name());
    /// ```
    pub const fn month_name(&self) -> &'static str {
        self.month.name()
    }

    /// Returns the month number between 1 and 12.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(2, japanese_date.month_number());
    /// ```
    pub const fn month_number(&self) -> u32 {
        self.month_number
    }

    /// Returns the `SexagenaryCycle`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, SexagenaryCycle};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(SexagenaryCycle::KinoeTatsu, japanese_date.sexagenary_cycle());
    /// ```
    pub const fn sexagenary_cycle(&self) -> SexagenaryCycle {
        self.sexagenary_cycle
    }

    /// Returns the name of sexagenary cycle.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("甲辰", japanese_date.sexagenary_cycle_name());
    /// ```
    pub const fn sexagenary_cycle_name(&self) -> &'static str {
        self.sexagenary_cycle.name()
    }

    /// Returns the `JapaneseWeekday`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, JapaneseWeekday};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(); // Tue
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(JapaneseWeekday::Ka, japanese_date.weekday());
    /// ```
    pub const fn weekday(&self) -> JapaneseWeekday {
        self.weekday
    }

    /// Returns the name of Japanese weekday.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Mon
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("月", japanese_date.weekday_name());
    /// ```
    pub const fn weekday_name(&self) -> &'static str {
        self.weekday.name()
    }

    /// Returns the western year.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(2024, japanese_date.western_year());
    /// ```
    pub const fn western_year(&self) -> i32 {
        self.western_year
    }

    /// Returns the `JapaneseZodiac`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use koyomi_rs::{JapaneseDate, JapaneseZodiac};
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!(JapaneseZodiac::Tatsu, japanese_date.zodiac());
    /// ```
    pub const fn zodiac(&self) -> JapaneseZodiac {
        self.zodiac
    }

    /// Returns the name of Japanese zodiac.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use koyomi_rs::JapaneseDate;
    ///
    /// let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let japanese_date = JapaneseDate::from_datelike(&chrono_date);
    ///
    /// assert_eq!("辰", japanese_date.zodiac_name());
    /// ```
    pub const fn zodiac_name(&self) -> &'static str {
        self.zodiac.name()
    }
}

/// Japanese calendar
///
/// A calendar that includes Japanese-specific definitions ([`JapaneseDate`]).
#[derive(Debug)]
pub struct Koyomi {
    current: NaiveDate,
    until: NaiveDate,
}

impl Koyomi {
    /// Generate a calendar for the specified period.
    ///
    /// # Exampla
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::Koyomi;
    ///
    /// let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let until = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    /// let koyomi = Koyomi::between(&from, &until);
    ///
    /// // 2024 is leap year.
    /// assert_eq!(366, koyomi.count());
    /// ```
    pub fn between<T: Datelike>(from: &T, until: &T) -> Self {
        Self {
            current: NaiveDate::from_ymd_opt(from.year(), from.month(), from.day()).unwrap(),
            until: NaiveDate::from_ymd_opt(until.year(), until.month(), until.day()).unwrap(),
        }
    }

    /// Generate a calenddar from month and year of Datelike.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::Koyomi;
    ///
    /// let current = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
    /// let koyomi = Koyomi::current_month(&current);
    ///
    /// // 2024 is leap year.
    /// assert_eq!(29, koyomi.count());
    /// ```
    pub fn current_month<T: Datelike>(date: &T) -> Self {
        Self::month_of(date.year(), date.month()).unwrap()
    }

    /// Generate a calendar from year of Datelike.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi_rs::Koyomi;
    ///
    /// let current = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    /// let koyomi = Koyomi::current_year(&current);
    ///
    /// // 2024 is leap year.
    /// assert_eq!(366, koyomi.count());
    /// ```
    pub fn current_year<T: Datelike>(date: &T) -> Self {
        Self::year_of(date.year())
    }

    /// Generate a calendar from now until specified date.
    /// Note: Now date is based on `chrono::Local` not `chrono::Utc`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::{Days, Local, NaiveDate};
    /// use koyomi_rs::Koyomi;
    ///
    /// let until = Local::now().checked_add_days(Days::new(10)).unwrap();
    /// let koyomi = Koyomi::from_now_until(&until);
    ///
    /// // Includes today.
    /// assert_eq!(11, koyomi.count());
    /// ```
    pub fn from_now_until<T: Datelike>(until: &T) -> Self {
        Self {
            current: Local::now().date_naive(),
            until: NaiveDate::from_ymd_opt(until.year(), until.month(), until.day()).unwrap(),
        }
    }

    /// Generate a calendar for the specified year and month.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi_rs::Koyomi;
    ///
    /// assert!(Koyomi::month_of(2024, 1).is_some());
    /// assert!(Koyomi::month_of(2024, 13).is_none());
    /// ```
    pub fn month_of(year: i32, month: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, 1)
            .map(|d| (d, d + Months::new(1)))
            .map(|(c, n)| (c, n.pred_opt().unwrap()))
            .map(|(c, n)| Self {
                current: c,
                until: n,
            })
    }

    /// Generate a calendar from specified date until now.
    /// Note: Now date is based on `chrono::Local` not `chrono::Utc`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::{Days, Local, NaiveDate};
    /// use koyomi_rs::Koyomi;
    ///
    /// let from = Local::now().checked_sub_days(Days::new(10)).unwrap();
    /// let koyomi = Koyomi::until_now_from(&from);
    ///
    /// // Includes today.
    /// assert_eq!(11, koyomi.count());
    /// ```
    pub fn until_now_from<T: Datelike>(from: &T) -> Self {
        Self {
            current: NaiveDate::from_ymd_opt(from.year(), from.month(), from.day()).unwrap(),
            until: Local::now().date_naive(),
        }
    }

    /// Generate a calendar for the specified year.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi_rs::Koyomi;
    ///
    /// // 2024 is leap year.
    /// assert_eq!(366, Koyomi::year_of(2024).count());
    /// ```
    pub fn year_of(year: i32) -> Self {
        Self {
            current: NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
            until: NaiveDate::from_ymd_opt(year, 12, 31).unwrap(),
        }
    }
}

impl Iterator for Koyomi {
    type Item = JapaneseDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.until < self.current {
            None
        } else {
            let current = self.current;
            self.current = current.succ_opt()?;
            Some(JapaneseDate::from_datelike(&current))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Days, Local, NaiveDate};
    use rstest::rstest;

    #[rstest]
    fn 指定した期間のカレンダーを生成できる() {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let until = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        let mut koyomi = Koyomi::between(&from, &until);
        let first = koyomi.nth(0).unwrap();
        let last = koyomi.last().unwrap();

        assert_eq!(
            (2024, 1, 1),
            (first.western_year(), first.month_number(), first.day())
        );
        assert_eq!(
            (2024, 1, 31),
            (last.western_year(), last.month_number(), last.day())
        );
    }

    #[rstest]
    fn 現在から指定した年月日までのカレンダーを生成できる() {
        let now = Local::now().date_naive();
        let until = now.checked_add_days(Days::new(10)).unwrap();

        let mut koyomi = Koyomi::from_now_until(&until);
        let first = koyomi.nth(0).unwrap();
        let last = koyomi.last().unwrap();

        assert_eq!(
            (now.year(), now.month(), now.day()),
            (first.western_year(), first.month_number(), first.day())
        );
        assert_eq!(
            (until.year(), until.month(), until.day()),
            (last.western_year(), last.month_number(), last.day())
        );
    }

    #[rstest]
    fn 指定した年月日から現在までのカレンダーを生成できる() {
        let now = Local::now().date_naive();
        let from = now.checked_sub_days(Days::new(10)).unwrap();

        let mut koyomi = Koyomi::until_now_from(&from);
        let first = koyomi.nth(0).unwrap();
        let last = koyomi.last().unwrap();

        assert_eq!(
            (from.year(), from.month(), from.day()),
            (first.western_year(), first.month_number(), first.day())
        );
        assert_eq!(
            (now.year(), now.month(), now.day()),
            (last.western_year(), last.month_number(), last.day())
        );
    }

    #[rstest]
    fn 指定した年のカレンダーを生成できる() {
        let koyomi = Koyomi::year_of(2024);

        // 2024 is leap year.
        assert_eq!(366, koyomi.count());
    }

    #[rstest]
    fn 指定した年月のカレンダーを生成できる() {
        let mut koyomi = Koyomi::month_of(2025, 2).unwrap();
        let first = koyomi.nth(0).unwrap();
        let last = koyomi.last().unwrap();

        assert_eq!(
            (2025, 2, 1),
            (first.western_year(), first.month_number(), first.day())
        );
        assert_eq!(
            (2025, 2, 28),
            (last.western_year(), last.month_number(), last.day())
        );
    }
}
