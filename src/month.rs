use crate::internal::{InnerDate, JAPANESE_MONTHS};
use chrono::Datelike;

use self::JapaneseMonth::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum JapaneseMonth {
    /// 睦月
    Mutsuki,
    /// 如月
    Kisaragi,
    /// 弥生
    Yayoi,
    /// 卯月
    Uzuki,
    /// 皐月
    Satsuki,
    /// 水無月
    Minazuki,
    /// 文月
    Fumizuki,
    /// 葉月
    Hazuki,
    /// 長月
    Nagatsuki,
    /// 神無月
    Kannazuki,
    /// 霜月
    Shimotsuki,
    /// 師走
    Shiwasu,
}

impl JapaneseMonth {
    /// Generate from Datelike of chrono.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseMonth;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert_eq!(JapaneseMonth::Mutsuki, JapaneseMonth::from_datelike(&date));
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Self {
        let current = InnerDate {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        };

        JapaneseMonth::from_usize(current.month as usize).unwrap()
    }

    /// Generate from string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseMonth;
    ///
    /// assert!(JapaneseMonth::from_str("師走").is_some());
    /// ```
    pub fn from_str(name: &str) -> Option<Self> {
        JAPANESE_MONTHS
            .iter()
            .position(|&x| x == name)
            .and_then(|i| JapaneseMonth::from_usize(i + 1))
    }

    /// Generate from unsigned integer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseMonth;
    ///
    /// assert!(JapaneseMonth::from_usize(1).is_some());
    /// ```
    pub const fn from_usize(num: usize) -> Option<Self> {
        match num {
            1 => Some(Mutsuki),
            2 => Some(Kisaragi),
            3 => Some(Yayoi),
            4 => Some(Uzuki),
            5 => Some(Satsuki),
            6 => Some(Minazuki),
            7 => Some(Fumizuki),
            8 => Some(Hazuki),
            9 => Some(Nagatsuki),
            10 => Some(Kannazuki),
            11 => Some(Shimotsuki),
            12 => Some(Shiwasu),
            _ => None,
        }
    }

    /// Convert to string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseMonth;
    ///
    /// assert_eq!("弥生", JapaneseMonth::Yayoi.to_str());
    /// ```
    pub const fn to_str(&self) -> &str {
        JAPANESE_MONTHS[self.to_usize() - 1]
    }

    /// Convert to unsigned integer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseMonth;
    ///
    /// assert_eq!(10, JapaneseMonth::Kannazuki.to_usize());
    /// ```
    pub const fn to_usize(&self) -> usize {
        match self {
            Mutsuki => 1,
            Kisaragi => 2,
            Yayoi => 3,
            Uzuki => 4,
            Satsuki => 5,
            Minazuki => 6,
            Fumizuki => 7,
            Hazuki => 8,
            Nagatsuki => 9,
            Kannazuki => 10,
            Shimotsuki => 11,
            Shiwasu => 12,
        }
    }
}

#[cfg(test)]
mod tests_japanese_month {
    use super::JapaneseMonth;
    use super::JapaneseMonth::*;

    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    fn 年月日から変換できる() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(Mutsuki, JapaneseMonth::from_datelike(&date));
    }

    #[rstest]
    #[case("睦月", Mutsuki)]
    #[case("如月", Kisaragi)]
    #[case("弥生", Yayoi)]
    #[case("卯月", Uzuki)]
    #[case("皐月", Satsuki)]
    #[case("水無月", Minazuki)]
    #[case("文月", Fumizuki)]
    #[case("葉月", Hazuki)]
    #[case("長月", Nagatsuki)]
    #[case("神無月", Kannazuki)]
    #[case("霜月", Shimotsuki)]
    #[case("師走", Shiwasu)]
    fn 月の文字から変換できる(#[case] name: &str, #[case] expect: JapaneseMonth) {
        assert_eq!(Some(expect), JapaneseMonth::from_str(name));
    }

    #[rstest]
    fn 月の文字でなければ変換できない() {
        assert!(JapaneseMonth::from_str("").is_none());
    }

    #[rstest]
    #[case(1, Mutsuki)]
    #[case(2, Kisaragi)]
    #[case(3, Yayoi)]
    #[case(4, Uzuki)]
    #[case(5, Satsuki)]
    #[case(6, Minazuki)]
    #[case(7, Fumizuki)]
    #[case(8, Hazuki)]
    #[case(9, Nagatsuki)]
    #[case(10, Kannazuki)]
    #[case(11, Shimotsuki)]
    #[case(12, Shiwasu)]
    fn 月の数字から変換できる(#[case] num: usize, #[case] expect: JapaneseMonth) {
        assert_eq!(Some(expect), JapaneseMonth::from_usize(num));
    }

    #[rstest]
    fn 月の数字でなければ変換できない() {
        assert!(JapaneseMonth::from_usize(0).is_none());
    }

    #[rstest]
    #[case(Mutsuki, "睦月")]
    #[case(Kisaragi, "如月")]
    #[case(Yayoi, "弥生")]
    #[case(Uzuki, "卯月")]
    #[case(Satsuki, "皐月")]
    #[case(Minazuki, "水無月")]
    #[case(Fumizuki, "文月")]
    #[case(Hazuki, "葉月")]
    #[case(Nagatsuki, "長月")]
    #[case(Kannazuki, "神無月")]
    #[case(Shimotsuki, "霜月")]
    #[case(Shiwasu, "師走")]
    fn 月の文字に変換できる(#[case] month: JapaneseMonth, #[case] expect: &str) {
        assert_eq!(expect, month.to_str());
    }

    #[rstest]
    #[case(Mutsuki, 1)]
    #[case(Kisaragi, 2)]
    #[case(Yayoi, 3)]
    #[case(Uzuki, 4)]
    #[case(Satsuki, 5)]
    #[case(Minazuki, 6)]
    #[case(Fumizuki, 7)]
    #[case(Hazuki, 8)]
    #[case(Nagatsuki, 9)]
    #[case(Kannazuki, 10)]
    #[case(Shimotsuki, 11)]
    #[case(Shiwasu, 12)]
    fn 月の数字に変換できる(#[case] month: JapaneseMonth, #[case] expect: usize) {
        assert_eq!(expect, month.to_usize());
    }
}
