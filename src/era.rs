use crate::inner::InnerDate;
use chrono::Datelike;

use self::JapaneseEra::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum JapaneseEra {
    /// 令和
    Reiwa(u8),
    /// 平成
    Heisei(u8),
    /// 昭和
    Showa(u8),
    /// 大正
    Taisho(u8),
    /// 明治
    Meiji(u8),
}

impl JapaneseEra {
    /// Generate from Datelike of chrono.
    /// The supported period extends up to the Meiji era.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseEra;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert_eq!(JapaneseEra::Reiwa(6), JapaneseEra::from_datelike(&date).unwrap());
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Option<Self> {
        let current = InnerDate {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        };

        JapaneseEra::reiwa(&current)
            .or(JapaneseEra::heisei(&current))
            .or(JapaneseEra::showa(&current))
            .or(JapaneseEra::taisho(&current))
            .or(JapaneseEra::meiji(&current))
    }

    /// Convert to string.
    /// The era year will be ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseEra;
    ///
    /// assert_eq!("令和", JapaneseEra::Reiwa(1).to_str());
    /// ```
    pub const fn to_str(&self) -> &str {
        match self {
            Reiwa(_) => "令和",
            Heisei(_) => "平成",
            Showa(_) => "昭和",
            Taisho(_) => "大正",
            Meiji(_) => "明治",
        }
    }

    fn heisei(date: &InnerDate) -> Option<Self> {
        let begin = InnerDate {
            year: 1989,
            month: 1,
            day: 8,
        };
        let until = InnerDate {
            year: 2019,
            month: 4,
            day: 30,
        };

        match &begin <= date && date <= &until {
            false => None,
            true => {
                let base = date.year + 12;
                let year = if base < 2000 {
                    base - 1900
                } else {
                    base - 2000
                };
                Some(Heisei(year as u8))
            }
        }
    }

    fn meiji(date: &InnerDate) -> Option<Self> {
        let begin = InnerDate {
            year: 1868,
            month: 10,
            day: 23,
        };
        let until = InnerDate {
            year: 1912,
            month: 7,
            day: 29,
        };

        match &begin <= date && date <= &until {
            false => None,
            true => {
                let year = date.year + 33 - 1900;
                Some(Meiji(year as u8))
            }
        }
    }

    fn reiwa(date: &InnerDate) -> Option<Self> {
        let begin = InnerDate {
            year: 2019,
            month: 5,
            day: 1,
        };

        match &begin <= date {
            false => None,
            true => {
                let year = date.year - 18 - 2000;
                Some(Reiwa(year as u8))
            }
        }
    }

    fn showa(date: &InnerDate) -> Option<Self> {
        let begin = InnerDate {
            year: 1926,
            month: 12,
            day: 25,
        };
        let until = InnerDate {
            year: 1989,
            month: 1,
            day: 7,
        };

        match &begin <= date && date <= &until {
            false => None,
            true => {
                let year = date.year - 25 - 1900;
                Some(Showa(year as u8))
            }
        }
    }

    fn taisho(date: &InnerDate) -> Option<Self> {
        let begin = InnerDate {
            year: 1912,
            month: 7,
            day: 30,
        };
        let until = InnerDate {
            year: 1926,
            month: 12,
            day: 24,
        };

        match &begin <= date && date <= &until {
            false => None,
            true => {
                let year = date.year - 11 - 1900;
                Some(Taisho(year as u8))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JapaneseEra;
    use super::JapaneseEra::*;

    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    #[case(1868, 10, 23, Meiji(1))]
    #[case(1912, 7, 29, Meiji(45))]
    fn 明治は1868年10月23日から1912年7月29日までである(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: JapaneseEra,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(Some(expect), JapaneseEra::from_datelike(&date));
    }

    #[rstest]
    #[case(1912, 7, 30, Taisho(1))]
    #[case(1926, 12, 24, Taisho(15))]
    fn 大正は1912年7月30日から1926年12月24日までである(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: JapaneseEra,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(Some(expect), JapaneseEra::from_datelike(&date));
    }

    #[rstest]
    #[case(1926, 12, 25, Showa(1))]
    #[case(1989, 1, 7, Showa(64))]
    fn 昭和は1926年12月25日から1989年1月7日までである(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: JapaneseEra,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(Some(expect), JapaneseEra::from_datelike(&date));
    }

    #[rstest]
    #[case(1989, 1, 8, Heisei(1))]
    #[case(2019, 4, 30, Heisei(31))]
    fn 平成は1989年1月8日から2019年4月30日までである(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: JapaneseEra,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(Some(expect), JapaneseEra::from_datelike(&date));
    }

    #[rstest]
    #[case(2019, 5, 1, Reiwa(1))]
    fn 令和は2019年5月1日以降である(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: JapaneseEra,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(Some(expect), JapaneseEra::from_datelike(&date));
    }

    #[rstest]
    fn 明治より前の元号には対応していない() {
        let date = NaiveDate::from_ymd_opt(1868, 10, 22).unwrap();
        assert!(JapaneseEra::from_datelike(&date).is_none());
    }
}
