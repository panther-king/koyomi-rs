use crate::internal::JAPANESE_WEEKDAY;
use chrono::{Datelike, NaiveDate, Weekday};

use self::JapaneseHoliday::*;
use self::JapaneseWeekday::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JapaneseHoliday {
    /// 秋分の日
    AutumnalEquinoxDay,
    /// こどもの日
    ChildrensDay,
    /// 成人の日
    ComingOfAgeDay,
    /// 憲法記念日
    ConstitutionDay,
    /// 文化の日
    CultureDay,
    /// 天皇誕生日
    EmperorsBirthday,
    /// 即位礼正殿の儀(平成天皇)
    EnthronmentCeremonyOfEmperorHeisei,
    /// 即位礼正殿の儀(令和天皇)
    EnthronmentCeremonyOfEmperorReiwa,
    /// 令和天皇即位
    EnthronmentOfEmperorReiwa,
    /// みどりの日
    GreenDay,
    /// 勤労感謝の日
    LaborThanksgivingDay,
    /// 海の日
    MarineDay,
    /// 山の日
    MountainDay,
    /// 昭和天皇大喪の礼
    MouringCeremonyOfEmperorShowa,
    /// 建国記念の日
    NationalFoundationDay,
    /// 元日
    NewYearsDay,
    /// 体育の日
    PhysicalEducationDay,
    /// 敬老の日
    RespectForTheAgeDay,
    /// 昭和の日
    ShowaDay,
    /// スポーツの日
    SportsDay,
    /// 振替休日
    SubstituteDay,
    /// 春分の日
    VernalEquinoxDay,
    /// 明仁親王の結婚の儀
    WeddingCeremonyOfPrinceAkihito,
    /// 徳仁親王の結婚の儀
    WeddingCeremonyOfPrinceNaruhito,
}

impl JapaneseHoliday {
    /// [秋分の日](https://ja.wikipedia.org/wiki/秋分の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 9, 22).unwrap();
    /// assert!(JapaneseHoliday::autumnal_equinox_day(&date).is_some());
    /// ```
    pub fn autumnal_equinox_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month()) {
            (1949.., 9) if autumnal_equinox_day(date) => Some(AutumnalEquinoxDay),
            _ => None,
        }
    }

    /// [こどもの日](https://ja.wikipedia.org/wiki/こどもの日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 5).unwrap();
    /// assert!(JapaneseHoliday::childrens_day(&date).is_some());
    /// ```
    pub fn childrens_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1948.., 5, 5) => Some(ChildrensDay),
            _ => None,
        }
    }

    /// [成人の日](https://ja.wikipedia.org/wiki/成人の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
    /// assert!(JapaneseHoliday::coming_of_age_day(&date).is_some());
    /// ```
    pub fn coming_of_age_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1949..=1999, 1, 15) => Some(ComingOfAgeDay),
            (2000.., 1, _) if happy_monday_second(date) => Some(ComingOfAgeDay),
            _ => None,
        }
    }

    /// [憲法記念日](https://ja.wikipedia.org/wiki/憲法記念日_(日本))
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 3).unwrap();
    /// assert!(JapaneseHoliday::constitution_day(&date).is_some());
    /// ```
    pub fn constitution_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1948.., 5, 3) => Some(ConstitutionDay),
            _ => None,
        }
    }

    /// [文化の日](https://ja.wikipedia.org/wiki/文化の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 11, 3).unwrap();
    /// assert!(JapaneseHoliday::culture_day(&date).is_some());
    /// ```
    pub fn culture_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1948.., 11, 3) => Some(CultureDay),
            _ => None,
        }
    }

    /// [天皇誕生日](https://ja.wikipedia.org/wiki/天皇誕生日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 2, 23).unwrap();
    /// assert!(JapaneseHoliday::emperors_birthday(&date).is_some());
    /// ```
    pub fn emperors_birthday<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1949..=1988, 4, 29) => Some(EmperorsBirthday),
            (1989..=2018, 12, 23) => Some(EmperorsBirthday),
            (2020.., 2, 23) => Some(EmperorsBirthday),
            _ => None,
        }
    }

    /// [みどりの日](https://ja.wikipedia.org/wiki/みどりの日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 4).unwrap();
    /// assert!(JapaneseHoliday::green_day(&date).is_some());
    /// ```
    pub fn green_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1989..=2006, 4, 29) => Some(GreenDay),
            (2007.., 5, 4) => Some(GreenDay),
            _ => None,
        }
    }

    /// [国民の祝日](https://ja.wikipedia.org/wiki/国民の祝日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert!(JapaneseHoliday::holiday(&date).is_some());
    /// ```
    pub fn holiday<T: Datelike>(date: &T) -> Option<Self> {
        JapaneseHoliday::holiday_without_substitute(date)
            .or(JapaneseHoliday::substitute_holiday(date))
    }

    /// [皇室慶弔行事に伴う休日](https://ja.wikipedia.org/wiki/皇室慶弔行事に伴う休日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2019, 5, 1).unwrap();
    /// assert!(JapaneseHoliday::imperial_ceremony_day(&date).is_some());
    /// ```
    pub fn imperial_ceremony_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1959, 4, 10) => Some(WeddingCeremonyOfPrinceAkihito),
            (1989, 2, 24) => Some(MouringCeremonyOfEmperorShowa),
            (1990, 11, 12) => Some(EnthronmentCeremonyOfEmperorHeisei),
            (1993, 6, 9) => Some(WeddingCeremonyOfPrinceNaruhito),
            (2019, 5, 1) => Some(EnthronmentOfEmperorReiwa),
            (2019, 10, 22) => Some(EnthronmentCeremonyOfEmperorReiwa),
            _ => None,
        }
    }

    /// [勤労感謝の日](https://ja.wikipedia.org/wiki/勤労感謝の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();
    /// assert!(JapaneseHoliday::labor_thanksgiving_day(&date).is_some());
    /// ```
    pub fn labor_thanksgiving_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1948.., 11, 23) => Some(LaborThanksgivingDay),
            _ => None,
        }
    }

    /// [海の日](https://ja.wikipedia.org/wiki/海の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
    /// assert!(JapaneseHoliday::marine_day(&date).is_some());
    /// ```
    pub fn marine_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (2020, 7, 23) => Some(MarineDay),
            (2021, 7, 22) => Some(MarineDay),
            (1996..=2002, 7, 20) => Some(MarineDay),
            (2003.., 7, _) if happy_monday_third(date) => Some(MarineDay),
            _ => None,
        }
    }

    /// [山の日](https://ja.wikipedia.org/wiki/山の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 8, 11).unwrap();
    /// assert!(JapaneseHoliday::mountain_day(&date).is_some());
    /// ```
    pub fn mountain_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (2020, 8, 10) => Some(MountainDay),
            (2021, 8, 8) => Some(MountainDay),
            (2016.., 8, 11) => Some(MountainDay),
            _ => None,
        }
    }

    /// [建国記念の日](https://ja.wikipedia.org/wiki/建国記念の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 2, 11).unwrap();
    /// assert!(JapaneseHoliday::national_foundation_day(&date).is_some());
    /// ```
    pub fn national_foundation_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1967.., 2, 11) => Some(NationalFoundationDay),
            _ => None,
        }
    }

    /// [元日](https://ja.wikipedia.org/wiki/元日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert!(JapaneseHoliday::new_years_day(&date).is_some());
    /// ```
    pub fn new_years_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1949.., 1, 1) => Some(NewYearsDay),
            _ => None,
        }
    }

    /// [体育の日](https://ja.wikipedia.org/wiki/スポーツの日_(日本))
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2019, 10, 14).unwrap();
    /// assert!(JapaneseHoliday::physical_education_day(&date).is_some());
    /// ```
    pub fn physical_education_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1966..=1999, 10, 10) => Some(PhysicalEducationDay),
            (2000..=2019, 10, _) if happy_monday_second(date) => Some(PhysicalEducationDay),
            _ => None,
        }
    }

    /// [敬老の日](https://ja.wikipedia.org/wiki/敬老の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 9, 16).unwrap();
    /// assert!(JapaneseHoliday::respect_for_the_age_day(&date).is_some());
    /// ```
    pub fn respect_for_the_age_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (1966..=2002, 9, 15) => Some(RespectForTheAgeDay),
            (2003.., 9, _) if happy_monday_third(date) => Some(RespectForTheAgeDay),
            _ => None,
        }
    }

    /// [昭和の日](https://ja.wikipedia.org/wiki/昭和の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 4, 29).unwrap();
    /// assert!(JapaneseHoliday::showa_day(&date).is_some());
    /// ```
    pub fn showa_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (2007.., 4, 29) => Some(ShowaDay),
            _ => None,
        }
    }

    /// [スポーツの日](https://ja.wikipedia.org/wiki/スポーツの日_(日本))
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 14).unwrap();
    /// assert!(JapaneseHoliday::sports_day(&date).is_some());
    /// ```
    pub fn sports_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month(), date.day()) {
            (2020, 7, 24) => Some(SportsDay),
            (2021, 7, 23) => Some(SportsDay),
            (2020.., 10, _) if happy_monday_second(date) => Some(SportsDay),
            _ => None,
        }
    }

    /// [振替休日](https://ja.wikipedia.org/wiki/振替休日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 2, 12).unwrap();
    /// assert!(JapaneseHoliday::substitute_holiday(&date).is_some());
    /// ```
    pub fn substitute_holiday<T: Datelike>(date: &T) -> Option<Self> {
        let enforced = NaiveDate::from_ymd_opt(1973, 4, 30).unwrap();
        let today = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();

        if today < enforced || date.year() <= 1973 {
            None
        } else {
            today
                .pred_opt()
                .and_then(|d| match JapaneseHoliday::holiday_without_substitute(&d) {
                    None => None,
                    Some(_) if d.weekday() == Weekday::Sun => Some(SubstituteDay),
                    _ => JapaneseHoliday::substitute_holiday(&d),
                })
        }
    }

    /// Returns the holiday name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseHoliday;
    ///
    /// let holiday = JapaneseHoliday::NewYearsDay;
    /// assert_eq!("元日", holiday.to_str());
    /// ```
    pub const fn to_str(&self) -> &'static str {
        match self {
            AutumnalEquinoxDay => "秋分の日",
            ChildrensDay => "こどもの日",
            ComingOfAgeDay => "成人の日",
            ConstitutionDay => "憲法記念日",
            CultureDay => "文化の日",
            EmperorsBirthday => "天皇誕生日",
            EnthronmentCeremonyOfEmperorHeisei => "即位礼正殿の儀",
            EnthronmentCeremonyOfEmperorReiwa => "即位礼正殿の儀",
            EnthronmentOfEmperorReiwa => "天皇即位",
            GreenDay => "みどりの日",
            LaborThanksgivingDay => "勤労感謝の日",
            MarineDay => "海の日",
            MountainDay => "山の日",
            MouringCeremonyOfEmperorShowa => "昭和天皇大喪の礼",
            NationalFoundationDay => "建国記念の日",
            NewYearsDay => "元日",
            PhysicalEducationDay => "体育の日",
            RespectForTheAgeDay => "敬老の日",
            ShowaDay => "昭和の日",
            SportsDay => "スポーツの日",
            SubstituteDay => "振替休日",
            VernalEquinoxDay => "春分の日",
            WeddingCeremonyOfPrinceAkihito => "明仁親王の結婚の儀",
            WeddingCeremonyOfPrinceNaruhito => "徳仁親王の結婚の儀",
        }
    }

    /// [春分の日](https://ja.wikipedia.org/wiki/春分の日)
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseHoliday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
    /// assert!(JapaneseHoliday::vernal_equinox_day(&date).is_some());
    /// ```
    pub fn vernal_equinox_day<T: Datelike>(date: &T) -> Option<Self> {
        match (date.year(), date.month()) {
            (1949.., 3) if vernal_equinox_day(date) => Some(VernalEquinoxDay),
            _ => None,
        }
    }

    fn holiday_without_substitute<T: Datelike>(date: &T) -> Option<Self> {
        JapaneseHoliday::autumnal_equinox_day(date)
            .or(JapaneseHoliday::childrens_day(date))
            .or(JapaneseHoliday::coming_of_age_day(date))
            .or(JapaneseHoliday::constitution_day(date))
            .or(JapaneseHoliday::culture_day(date))
            .or(JapaneseHoliday::emperors_birthday(date))
            .or(JapaneseHoliday::green_day(date))
            .or(JapaneseHoliday::imperial_ceremony_day(date))
            .or(JapaneseHoliday::labor_thanksgiving_day(date))
            .or(JapaneseHoliday::marine_day(date))
            .or(JapaneseHoliday::mountain_day(date))
            .or(JapaneseHoliday::national_foundation_day(date))
            .or(JapaneseHoliday::new_years_day(date))
            .or(JapaneseHoliday::physical_education_day(date))
            .or(JapaneseHoliday::respect_for_the_age_day(date))
            .or(JapaneseHoliday::showa_day(date))
            .or(JapaneseHoliday::sports_day(date))
            .or(JapaneseHoliday::vernal_equinox_day(date))
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum JapaneseWeekday {
    /// 月
    Getsu,
    /// 火
    Ka,
    /// 水
    Sui,
    /// 木
    Moku,
    /// 金
    Kin,
    /// 土
    Do,
    /// 日
    Nichi,
}

impl JapaneseWeekday {
    /// Generate from Datelike of chrono.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseWeekday;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert_eq!(JapaneseWeekday::Getsu, JapaneseWeekday::from_datelike(&date));
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Self {
        let index = date.weekday().number_from_monday();
        JapaneseWeekday::from_usize(index as usize).unwrap()
    }

    /// Generate from string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseWeekday;
    ///
    /// assert!(JapaneseWeekday::from_str("金").is_some());
    /// ```
    pub fn from_str(name: &str) -> Option<Self> {
        JAPANESE_WEEKDAY
            .iter()
            .position(|&w| w == name)
            .and_then(|i| JapaneseWeekday::from_usize(i + 1))
    }

    /// Generate from unsigned integer.
    /// The index starts from `1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseWeekday;
    ///
    /// assert!(JapaneseWeekday::from_usize(7).is_some());
    /// ```
    pub const fn from_usize(num: usize) -> Option<Self> {
        match num {
            1 => Some(Getsu),
            2 => Some(Ka),
            3 => Some(Sui),
            4 => Some(Moku),
            5 => Some(Kin),
            6 => Some(Do),
            7 => Some(Nichi),
            _ => None,
        }
    }

    /// Convert to string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseWeekday;
    ///
    /// assert_eq!("水", JapaneseWeekday::Sui.to_str());
    /// ```
    pub const fn to_str(&self) -> &'static str {
        JAPANESE_WEEKDAY[self.to_usize() - 1]
    }

    /// Convert to unsigned integer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseWeekday;
    ///
    /// assert_eq!(4, JapaneseWeekday::Moku.to_usize());
    /// ```
    pub const fn to_usize(&self) -> usize {
        match self {
            Getsu => 1,
            Ka => 2,
            Sui => 3,
            Moku => 4,
            Kin => 5,
            Do => 6,
            Nichi => 7,
        }
    }
}

fn autumnal_equinox_day<T: Datelike>(date: &T) -> bool {
    date.day() == equinox_day(23.2488, date.year())
}

fn vernal_equinox_day<T: Datelike>(date: &T) -> bool {
    date.day() == equinox_day(20.8431, date.year())
}

fn equinox_day(equinox: f64, year: i32) -> u32 {
    let x = (year - 1980) as f64;
    let y = ((0.242194_f64 * x) + equinox).floor() as i32;
    let z = (x / 4.0_f64).floor() as i32;

    (y - z).abs() as u32
}

fn happy_monday_second<T: Datelike>(date: &T) -> bool {
    match (date.weekday(), date.day()) {
        (Weekday::Mon, 8..=14) => true,
        _ => false,
    }
}

fn happy_monday_third<T: Datelike>(date: &T) -> bool {
    match (date.weekday(), date.day()) {
        (Weekday::Mon, 15..=21) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests_new_years_day {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後1月1日は元日である() {
        let date = NaiveDate::from_ymd_opt(1949, 1, 1).unwrap();
        assert_eq!(Some(NewYearsDay), JapaneseHoliday::new_years_day(&date));
    }

    #[rstest]
    fn 祝日法の施行以前は1月1日であっても元日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 1, 1).unwrap();
        assert_ne!(Some(NewYearsDay), JapaneseHoliday::new_years_day(&date));
    }
}

#[cfg(test)]
mod tests_coming_of_age_day {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後1月15日は成人の日である() {
        let date = NaiveDate::from_ymd_opt(1949, 1, 15).unwrap();
        assert_eq!(
            Some(ComingOfAgeDay),
            JapaneseHoliday::coming_of_age_day(&date)
        );
    }

    #[rstest]
    fn 祝日法の施行以前は1月15日であっても成人の日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 1, 15).unwrap();
        assert_ne!(
            Some(ComingOfAgeDay),
            JapaneseHoliday::coming_of_age_day(&date)
        );
    }

    #[rstest]
    fn 祝日法の改正後は1月の第2月曜が成人の日である() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 10).unwrap();
        assert_eq!(
            Some(ComingOfAgeDay),
            JapaneseHoliday::coming_of_age_day(&date)
        );
    }

    #[rstest]
    fn 祝日法の改正後は1月15日であっても第2月曜でなければ成人の日ではない() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 15).unwrap();
        assert_ne!(
            Some(ComingOfAgeDay),
            JapaneseHoliday::coming_of_age_day(&date)
        );
    }
}

#[cfg(test)]
mod tests_national_foundation_day {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の改正後は2月15日が建国記念の日である() {
        let date = NaiveDate::from_ymd_opt(1967, 2, 11).unwrap();
        assert_eq!(
            Some(NationalFoundationDay),
            JapaneseHoliday::national_foundation_day(&date)
        );
    }

    #[rstest]
    fn 祝日法の改正以前は2月15日であっても建国記念の日ではない() {
        let date = NaiveDate::from_ymd_opt(1966, 2, 15).unwrap();
        assert_ne!(
            Some(NationalFoundationDay),
            JapaneseHoliday::national_foundation_day(&date)
        );
    }
}

#[cfg(test)]
mod tests_emperors_birthday {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1949)]
    #[case(1988)]
    fn 祝日法の施行後で昭和天皇在位時は4月29日が天皇誕生日になる(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 4, 29).unwrap();
        assert_eq!(
            Some(EmperorsBirthday),
            JapaneseHoliday::emperors_birthday(&date)
        );
    }

    #[rstest]
    #[case(1989)]
    #[case(2018)]
    fn 祝日法の施行後で平成天皇在位時は12月23日が天皇誕生日になる(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 12, 23).unwrap();
        assert_eq!(
            Some(EmperorsBirthday),
            JapaneseHoliday::emperors_birthday(&date)
        );
    }

    #[rstest]
    #[case(2020)]
    fn 祝日法の施行後で令和天皇在位時は2月23日が天皇誕生日になる(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 2, 23).unwrap();
        assert_eq!(
            Some(EmperorsBirthday),
            JapaneseHoliday::emperors_birthday(&date)
        );
    }

    #[rstest]
    fn 祝日法の施行前は昭和天皇在位時の4月29日でも天皇誕生日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 4, 29).unwrap();
        assert_ne!(
            Some(EmperorsBirthday),
            JapaneseHoliday::emperors_birthday(&date)
        );
    }

    #[rstest]
    #[case(2, 23)]
    #[case(12, 23)]
    fn 天皇退位の2019年は天皇誕生日が存在しない(#[case] m: u32, #[case] d: u32) {
        let date = NaiveDate::from_ymd_opt(2019, m, d).unwrap();
        assert_ne!(
            Some(EmperorsBirthday),
            JapaneseHoliday::emperors_birthday(&date)
        );
    }
}

#[cfg(test)]
mod vernal_equinox_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の春分日は春分の日である() {
        let date = NaiveDate::from_ymd_opt(1949, 3, 21).unwrap();
        assert_eq!(
            Some(VernalEquinoxDay),
            JapaneseHoliday::vernal_equinox_day(&date)
        );
    }

    #[rstest]
    fn 祝日法施行以前は春分日であっても春分の日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 3, 21).unwrap();
        assert_ne!(
            Some(VernalEquinoxDay),
            JapaneseHoliday::vernal_equinox_day(&date)
        );
    }
}

#[cfg(test)]
mod green_days_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1989)]
    #[case(2006)]
    fn 昭和天皇崩御から昭和の日制定まで4月29日はみどりの日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 4, 29).unwrap();
        assert_eq!(Some(GreenDay), JapaneseHoliday::green_day(&date));
    }

    #[rstest]
    fn 昭和の日制定後は5月4日がみどりの日である() {
        let date = NaiveDate::from_ymd_opt(2007, 5, 4).unwrap();
        assert_eq!(Some(GreenDay), JapaneseHoliday::green_day(&date));
    }

    #[rstest]
    fn 昭和天皇在位時の4月29日はみどりの日ではない() {
        let date = NaiveDate::from_ymd_opt(1988, 4, 29).unwrap();
        assert_ne!(Some(GreenDay), JapaneseHoliday::green_day(&date));
    }
}

#[cfg(test)]
mod showa_days_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 昭和の日が制定された2007年以降は4月29日が昭和の日である() {
        let date = NaiveDate::from_ymd_opt(2007, 4, 29).unwrap();
        assert_eq!(Some(ShowaDay), JapaneseHoliday::showa_day(&date));
    }

    #[rstest]
    fn 昭和の日制定以前の4月29日は昭和の日ではない() {
        let date = NaiveDate::from_ymd_opt(2006, 4, 29).unwrap();
        assert_ne!(Some(ShowaDay), JapaneseHoliday::showa_day(&date));
    }
}

#[cfg(test)]
mod constitution_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後5月3日は憲法記念日である() {
        let date = NaiveDate::from_ymd_opt(1948, 5, 3).unwrap();
        assert_eq!(
            Some(ConstitutionDay),
            JapaneseHoliday::constitution_day(&date)
        );
    }

    #[rstest]
    fn 祝日法施行以前の5月3日は憲法記念日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 5, 3).unwrap();
        assert_ne!(
            Some(ConstitutionDay),
            JapaneseHoliday::constitution_day(&date)
        );
    }
}

#[cfg(test)]
mod childrens_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後5月5日はこどもの日である() {
        let date = NaiveDate::from_ymd_opt(1948, 5, 5).unwrap();
        assert_eq!(Some(ChildrensDay), JapaneseHoliday::childrens_day(&date));
    }

    #[rstest]
    fn 祝日法施行以前の5月5日はこどもの日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 5, 5).unwrap();
        assert_ne!(Some(ChildrensDay), JapaneseHoliday::childrens_day(&date));
    }
}

#[cfg(test)]
mod marin_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1996)]
    #[case(2002)]
    fn 制定年の1996年からハッピーマンデー導入の2002年まで7月20日は海の日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 7, 20).unwrap();
        assert_eq!(Some(MarineDay), JapaneseHoliday::marine_day(&date));
    }

    #[rstest]
    fn 制定以前の7月20日は海の日ではない() {
        let date = NaiveDate::from_ymd_opt(1995, 7, 20).unwrap();
        assert_ne!(Some(MarineDay), JapaneseHoliday::marine_day(&date));
    }

    #[rstest]
    #[case(2003, 7, 21)]
    #[case(2019, 7, 15)]
    fn ハッピーマンデー導入後は7月の第3月曜が海の日である(
        #[case] y: i32,
        #[case] m: u32,
        #[case] d: u32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, m, d).unwrap();
        assert_eq!(Some(MarineDay), JapaneseHoliday::marine_day(&date));
    }

    #[rstest]
    fn 東京五輪の特措法に基づき2020年は7月23日が海の日である() {
        let date = NaiveDate::from_ymd_opt(2020, 7, 23).unwrap();
        assert_eq!(Some(MarineDay), JapaneseHoliday::marine_day(&date));
    }

    #[rstest]
    fn 東京オリンピック開催年の2021年は開会式前日の7月22日が海の日である() {
        let date = NaiveDate::from_ymd_opt(2021, 7, 22).unwrap();
        assert_eq!(Some(MarineDay), JapaneseHoliday::marine_day(&date));
    }

    #[rstest]
    fn 東京オリンピック以降は7月の第3月曜が海の日である() {
        let date = NaiveDate::from_ymd_opt(2022, 7, 18).unwrap();
        assert_eq!(Some(MarineDay), JapaneseHoliday::marine_day(&date));
    }
}

#[cfg(test)]
mod mountain_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(2016)]
    #[case(2019)]
    fn 制定年の2016年から2019年まで8月11日は山の日である(#[case] y: i32) {
        let date = NaiveDate::from_ymd_opt(y, 8, 11).unwrap();
        assert_eq!(Some(MountainDay), JapaneseHoliday::mountain_day(&date));
    }

    #[rstest]
    fn 制定以前の8月11日は山の日ではない() {
        let date = NaiveDate::from_ymd_opt(2015, 8, 11).unwrap();
        assert_ne!(Some(MountainDay), JapaneseHoliday::mountain_day(&date));
    }

    #[rstest]
    fn 東京五輪の特措法に基づき2020年は8月10日が山の日である() {
        let date = NaiveDate::from_ymd_opt(2020, 8, 10).unwrap();
        assert_eq!(Some(MountainDay), JapaneseHoliday::mountain_day(&date));
    }

    #[rstest]
    fn 東京オリンピック開催年の2021年は閉会式翌日の8月8日が山の日である() {
        let date = NaiveDate::from_ymd_opt(2021, 8, 8).unwrap();
        assert_eq!(Some(MountainDay), JapaneseHoliday::mountain_day(&date));
    }

    #[rstest]
    fn 東京オリンピック以降は8月11日が山の日である() {
        let date = NaiveDate::from_ymd_opt(2022, 8, 11).unwrap();
        assert_eq!(Some(MountainDay), JapaneseHoliday::mountain_day(&date));
    }
}

#[cfg(test)]
mod respect_for_the_age_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1966)]
    #[case(2002)]
    fn 制定年の1966年からハッピーマンデー導入の2002年まで9月15日は敬老の日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 9, 15).unwrap();
        assert_eq!(
            Some(RespectForTheAgeDay),
            JapaneseHoliday::respect_for_the_age_day(&date)
        );
    }

    #[rstest]
    fn 制定以前の9月15日は敬老の日ではない() {
        let date = NaiveDate::from_ymd_opt(1965, 9, 15).unwrap();
        assert_ne!(
            Some(RespectForTheAgeDay),
            JapaneseHoliday::respect_for_the_age_day(&date)
        );
    }

    #[rstest]
    fn ハッピーマンデー導入後は9月の第3月曜が敬老の日である() {
        let date = NaiveDate::from_ymd_opt(2022, 9, 19).unwrap();
        assert_eq!(
            Some(RespectForTheAgeDay),
            JapaneseHoliday::respect_for_the_age_day(&date)
        );
    }
}

#[cfg(test)]
mod autumnal_equinox_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の秋分日は秋分の日である() {
        let date = NaiveDate::from_ymd_opt(1949, 9, 23).unwrap();
        assert_eq!(
            Some(AutumnalEquinoxDay),
            JapaneseHoliday::autumnal_equinox_day(&date)
        );
    }

    #[rstest]
    fn 祝日法施行以前は秋分日であっても秋分の日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 9, 23).unwrap();
        assert_ne!(
            Some(AutumnalEquinoxDay),
            JapaneseHoliday::autumnal_equinox_day(&date)
        );
    }
}

#[cfg(test)]
mod physical_education_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1966)]
    #[case(1999)]
    fn 制定年の1966年からハッピーマンデー導入の1999年まで10月10日は体育の日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 10, 10).unwrap();
        assert_eq!(
            Some(PhysicalEducationDay),
            JapaneseHoliday::physical_education_day(&date)
        );
    }

    #[rstest]
    fn 制定以前の10月10日は体育の日ではない() {
        let date = NaiveDate::from_ymd_opt(1965, 10, 10).unwrap();
        assert_ne!(
            Some(PhysicalEducationDay),
            JapaneseHoliday::physical_education_day(&date)
        );
    }

    #[rstest]
    fn ハッピーマンデー導入後は10月の第2月曜が体育の日である() {
        let date = NaiveDate::from_ymd_opt(2000, 10, 9).unwrap();
        assert_eq!(
            Some(PhysicalEducationDay),
            JapaneseHoliday::physical_education_day(&date)
        );
    }
}

#[cfg(test)]
mod sports_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 東京五輪の特措法に基づき2020年は7月24日がスポーツの日である() {
        let date = NaiveDate::from_ymd_opt(2020, 7, 24).unwrap();
        assert_eq!(Some(SportsDay), JapaneseHoliday::sports_day(&date));
    }

    #[rstest]
    fn 東京オリンピック開催年の2021年は7月23日がスポーツの日である() {
        let date = NaiveDate::from_ymd_opt(2021, 7, 23).unwrap();
        assert_eq!(Some(SportsDay), JapaneseHoliday::sports_day(&date));
    }

    #[rstest]
    fn 東京オリンピック以降は10月の第2月曜がスポーツの日である() {
        let date = NaiveDate::from_ymd_opt(2022, 10, 10).unwrap();
        assert_eq!(Some(SportsDay), JapaneseHoliday::sports_day(&date));
    }

    #[rstest]
    fn 制定以前の10月第2月曜はスポーツの日ではない() {
        let date = NaiveDate::from_ymd_opt(2019, 10, 14).unwrap();
        assert_ne!(Some(SportsDay), JapaneseHoliday::sports_day(&date));
    }
}

#[cfg(test)]
mod culture_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の11月3日は文化の日である() {
        let date = NaiveDate::from_ymd_opt(1948, 11, 3).unwrap();
        assert_eq!(Some(CultureDay), JapaneseHoliday::culture_day(&date));
    }

    #[rstest]
    fn 祝日法の施行以前は11月3日であっても文化の日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 11, 3).unwrap();
        assert_ne!(Some(CultureDay), JapaneseHoliday::culture_day(&date));
    }
}

#[cfg(test)]
mod labor_thanksgiving_day_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の11月23日は勤労感謝の日である() {
        let date = NaiveDate::from_ymd_opt(1948, 11, 23).unwrap();
        assert_eq!(
            Some(LaborThanksgivingDay),
            JapaneseHoliday::labor_thanksgiving_day(&date)
        );
    }

    #[rstest]
    fn 祝日法の施行以前は11月23日であっても勤労感謝の日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 11, 23).unwrap();
        assert_ne!(
            Some(LaborThanksgivingDay),
            JapaneseHoliday::labor_thanksgiving_day(&date)
        );
    }
}

#[cfg(test)]
mod imperial_ceremony_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 明仁親王の結婚の儀が行われた1959年4月10日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1959, 4, 10).unwrap();
        assert_eq!(
            Some(WeddingCeremonyOfPrinceAkihito),
            JapaneseHoliday::imperial_ceremony_day(&date)
        );
    }

    #[rstest]
    fn 昭和天皇大喪の礼が行われた1989年2月24日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1989, 2, 24).unwrap();
        assert_eq!(
            Some(MouringCeremonyOfEmperorShowa),
            JapaneseHoliday::imperial_ceremony_day(&date)
        );
    }

    #[rstest]
    fn 平成天皇の即位礼正殿の儀が行われた1990年11月12日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1990, 11, 12).unwrap();
        assert_eq!(
            Some(EnthronmentCeremonyOfEmperorHeisei),
            JapaneseHoliday::imperial_ceremony_day(&date)
        );
    }

    #[rstest]
    fn 徳仁親王の結婚の儀が行われた1993年6月9日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1993, 6, 9).unwrap();
        assert_eq!(
            Some(WeddingCeremonyOfPrinceNaruhito),
            JapaneseHoliday::imperial_ceremony_day(&date)
        );
    }

    #[rstest]
    fn 令和天皇が即位した2019年5月1日は祝日である() {
        let date = NaiveDate::from_ymd_opt(2019, 5, 1).unwrap();
        assert_eq!(
            Some(EnthronmentOfEmperorReiwa),
            JapaneseHoliday::imperial_ceremony_day(&date)
        );
    }

    #[rstest]
    fn 令和天皇の即位礼正殿の儀が行われた2019年10月22日は祝日である() {
        let date = NaiveDate::from_ymd_opt(2019, 10, 22).unwrap();
        assert_eq!(
            Some(EnthronmentCeremonyOfEmperorReiwa),
            JapaneseHoliday::imperial_ceremony_day(&date)
        );
    }
}

#[cfg(test)]
mod substitute_holiday_tests {
    use super::JapaneseHoliday;
    use super::JapaneseHoliday::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 日曜が祝日の場合は次の月曜が振替休日である() {
        let date = NaiveDate::from_ymd_opt(2021, 8, 9).unwrap();
        assert_eq!(
            Some(SubstituteDay),
            JapaneseHoliday::substitute_holiday(&date)
        );
    }

    #[rstest]
    fn 日曜の次に祝日が連続する場合は祝日の次の平日が振替休日である() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 6).unwrap();
        assert_eq!(
            Some(SubstituteDay),
            JapaneseHoliday::substitute_holiday(&date)
        );
    }

    #[rstest]
    fn 祝日法の改正前は日曜が祝日であっても次の月曜は振替休日ではない() {
        let date = NaiveDate::from_ymd_opt(1973, 2, 12).unwrap();
        assert_ne!(
            Some(SubstituteDay),
            JapaneseHoliday::substitute_holiday(&date)
        );
    }
}

#[cfg(test)]
mod japanese_weekday_tests {
    use super::JapaneseWeekday;
    use super::JapaneseWeekday::*;
    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    fn 年月日から変換できる() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(Getsu, JapaneseWeekday::from_datelike(&date));
    }

    #[rstest]
    #[case("月", Getsu)]
    #[case("火", Ka)]
    #[case("水", Sui)]
    #[case("木", Moku)]
    #[case("金", Kin)]
    #[case("土", Do)]
    #[case("日", Nichi)]
    fn 曜日の文字列から変換できる(
        #[case] name: &str,
        #[case] expect: JapaneseWeekday,
    ) {
        assert_eq!(Some(expect), JapaneseWeekday::from_str(name));
    }

    #[rstest]
    fn 曜日の文字列でなければ変換できない() {
        assert!(JapaneseWeekday::from_str("").is_none());
    }

    #[rstest]
    #[case(1, Getsu)]
    #[case(2, Ka)]
    #[case(3, Sui)]
    #[case(4, Moku)]
    #[case(5, Kin)]
    #[case(6, Do)]
    #[case(7, Nichi)]
    fn 曜日の順番から変換できる(#[case] num: usize, #[case] expect: JapaneseWeekday) {
        assert_eq!(Some(expect), JapaneseWeekday::from_usize(num));
    }

    #[rstest]
    fn 曜日の順番範囲内でなければ変換できない() {
        assert!(JapaneseWeekday::from_usize(0).is_none());
    }

    #[rstest]
    #[case(Getsu, "月")]
    #[case(Ka, "火")]
    #[case(Sui, "水")]
    #[case(Moku, "木")]
    #[case(Kin, "金")]
    #[case(Do, "土")]
    #[case(Nichi, "日")]
    fn 曜日の文字列に変換できる(
        #[case] weekday: JapaneseWeekday,
        #[case] expect: &str,
    ) {
        assert_eq!(expect, weekday.to_str());
    }

    #[rstest]
    #[case(Getsu, 1)]
    #[case(Ka, 2)]
    #[case(Sui, 3)]
    #[case(Moku, 4)]
    #[case(Kin, 5)]
    #[case(Do, 6)]
    #[case(Nichi, 7)]
    fn 曜日の順番に変換できる(#[case] weekday: JapaneseWeekday, #[case] expect: usize) {
        assert_eq!(expect, weekday.to_usize());
    }
}
