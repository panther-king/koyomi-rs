use crate::internal::{InternalDate, HEAVENLY_STEMS, JAPANESE_ZODIAC, SEXAGENARY_CYCLE};
use chrono::Datelike;

use self::HeavenlyStem::*;
use self::JapaneseZodiac::*;
use self::SexagenaryCycle::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HeavenlyStem {
    /// 甲
    Kinoe,
    /// 乙
    Kinoto,
    /// 丙
    Hinoe,
    /// 丁
    Hinoto,
    /// 戊
    Tsuchinoe,
    /// 己
    Tsuchinoto,
    /// 庚
    Kanoe,
    /// 辛
    Kanoto,
    /// 壬
    Mizunoe,
    /// 癸
    Mizunoto,
}

impl HeavenlyStem {
    /// Generate from Datelike of chrono.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::HeavenlyStem;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert_eq!(HeavenlyStem::Kinoe, HeavenlyStem::from_datelike(&date));
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Self {
        let current = InternalDate {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        };

        // @refs http://tadopika.net/fate/tendaycount.html
        let year = if current.year % 2 == 0 { 0 } else { 5 };
        let month = match current.month {
            2 | 6 | 7 => 0,
            8 => 1,
            9 | 10 => 2,
            11 | 12 => 3,
            3 => 8,
            1 | 4 | 5 => 9,
            _ => unreachable!(),
        };
        let day = current.day as i32;
        let leap4 = match current.month {
            1 | 2 => (current.year - 1) / 4,
            3..=12 => current.year / 4,
            _ => unreachable!(),
        };
        let leap100 = match current.month {
            1 | 2 => (current.year - 1) / 100,
            3..=12 => current.year / 100,
            _ => unreachable!(),
        };
        let leap400 = leap4 / 100;
        let total = (year + leap4 - leap100 + leap400 + month + day) as usize;
        let index = total % 10;

        HeavenlyStem::from_usize(index + 1).unwrap()
    }

    /// Generate from string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::HeavenlyStem;
    ///
    /// assert!(HeavenlyStem::from_str("甲").is_some());
    /// ```
    pub fn from_str(name: &str) -> Option<Self> {
        HEAVENLY_STEMS
            .iter()
            .position(|&x| x == name)
            .and_then(|i| HeavenlyStem::from_usize(i + 1))
    }

    /// Generate from unsigned integer.
    /// The index starts from `1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::HeavenlyStem;
    ///
    /// assert!(HeavenlyStem::from_usize(1).is_some());
    /// ```
    pub const fn from_usize(num: usize) -> Option<Self> {
        match num {
            1 => Some(Kinoe),
            2 => Some(Kinoto),
            3 => Some(Hinoe),
            4 => Some(Hinoto),
            5 => Some(Tsuchinoe),
            6 => Some(Tsuchinoto),
            7 => Some(Kanoe),
            8 => Some(Kanoto),
            9 => Some(Mizunoe),
            10 => Some(Mizunoto),
            _ => None,
        }
    }

    /// Convert to string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::HeavenlyStem;
    ///
    /// assert_eq!("甲", HeavenlyStem::Kinoe.to_str());
    /// ```
    pub const fn to_str(&self) -> &str {
        HEAVENLY_STEMS[self.to_usize() - 1]
    }

    /// Convert to unsigned integer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::HeavenlyStem;
    ///
    /// assert_eq!(10, HeavenlyStem::Mizunoto.to_usize());
    /// ```
    pub const fn to_usize(&self) -> usize {
        match self {
            Kinoe => 1,
            Kinoto => 2,
            Hinoe => 3,
            Hinoto => 4,
            Tsuchinoe => 5,
            Tsuchinoto => 6,
            Kanoe => 7,
            Kanoto => 8,
            Mizunoe => 9,
            Mizunoto => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum JapaneseZodiac {
    /// 子
    Ne,
    /// 丑
    Ushi,
    /// 寅
    Tora,
    /// 卯
    Wu,
    /// 辰
    Tatsu,
    /// 巳
    Mi,
    /// 午
    Uma,
    /// 未
    Hitsuji,
    /// 申
    Saru,
    /// 酉
    Tori,
    /// 戌
    Inu,
    /// 亥
    Yi,
}

impl JapaneseZodiac {
    /// Generate from Datelike of chrono.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::JapaneseZodiac;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert_eq!(JapaneseZodiac::Tatsu, JapaneseZodiac::from_datelike(&date));
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Self {
        // @refs https://kanshiqsei.com/ad-year-change-jyunishi-of-year
        match (date.year() + 9) % 12 {
            0 => JapaneseZodiac::from_usize(12).unwrap(),
            n => JapaneseZodiac::from_usize(n as usize).unwrap(),
        }
    }

    /// Generate from string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseZodiac;
    ///
    /// assert!(JapaneseZodiac::from_str("寅").is_some());
    /// ```
    pub fn from_str(name: &str) -> Option<Self> {
        JAPANESE_ZODIAC
            .iter()
            .position(|&x| x == name)
            .and_then(|i| JapaneseZodiac::from_usize(i + 1))
    }

    /// Generate from unsigned integer.
    /// The index starts from `1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseZodiac;
    ///
    /// assert!(JapaneseZodiac::from_usize(1).is_some());
    /// ```
    pub const fn from_usize(num: usize) -> Option<Self> {
        match num {
            1 => Some(Ne),
            2 => Some(Ushi),
            3 => Some(Tora),
            4 => Some(Wu),
            5 => Some(Tatsu),
            6 => Some(Mi),
            7 => Some(Uma),
            8 => Some(Hitsuji),
            9 => Some(Saru),
            10 => Some(Tori),
            11 => Some(Inu),
            12 => Some(Yi),
            _ => None,
        }
    }

    /// Convert to string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseZodiac;
    ///
    /// assert_eq!("丑", JapaneseZodiac::Ushi.to_str());
    /// ```
    pub const fn to_str(&self) -> &str {
        JAPANESE_ZODIAC[self.to_usize() - 1]
    }

    /// Convert to unsigned integer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::JapaneseZodiac;
    ///
    /// assert_eq!(12, JapaneseZodiac::Yi.to_usize());
    /// ```
    pub const fn to_usize(&self) -> usize {
        match self {
            Ne => 1,
            Ushi => 2,
            Tora => 3,
            Wu => 4,
            Tatsu => 5,
            Mi => 6,
            Uma => 7,
            Hitsuji => 8,
            Saru => 9,
            Tori => 10,
            Inu => 11,
            Yi => 12,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SexagenaryCycle {
    /// 甲子
    KinoeNe,
    /// 乙丑
    KinotoUshi,
    /// 丙寅
    HinoeTora,
    /// 丁卯
    HinotoWu,
    /// 戊辰
    TsuchinoeTatsu,
    /// 己巳
    TsuchinotoMi,
    /// 庚午
    KanoeUma,
    /// 辛未
    KanotoHitsuji,
    /// 壬申
    MizunoeSaru,
    /// 癸酉
    MizunotoTori,
    /// 甲戌
    KinoeInu,
    /// 乙亥
    KinotoYi,
    /// 丙子
    HinoeNe,
    /// 丁丑
    HinotoUshi,
    /// 戊寅
    TsuchinoeTora,
    /// 己卯
    TsuchinotoWu,
    /// 庚辰
    KanoeTatsu,
    /// 辛巳
    KanotoMi,
    /// 壬午
    MizunoeUma,
    /// 癸未
    MizunotoHitsuji,
    /// 甲申
    KinoeSaru,
    /// 乙酉
    KinotoTori,
    /// 丙戌
    HinoeInu,
    /// 丁亥
    HinotoYi,
    /// 戊子
    TsuchinoeNe,
    /// 己丑
    TsuchinotoUshi,
    /// 庚寅
    KanoeTora,
    /// 辛卯
    KanotoWu,
    /// 壬辰
    MizunoeTatsu,
    /// 癸巳
    MizunotoMi,
    /// 甲午
    KinoeUma,
    /// 乙未
    KinotoHitsuji,
    /// 丙申
    HinoeSaru,
    /// 丁酉
    HinotoTori,
    /// 戊戌
    TsuchinoeInu,
    /// 己亥
    TsuchinotoYi,
    /// 庚子
    KanoeNe,
    /// 辛丑
    KanotoUshi,
    /// 壬寅
    MizunoeTora,
    /// 癸卯
    MizunotoWu,
    /// 甲辰
    KinoeTatsu,
    /// 乙巳
    KinotoMi,
    /// 丙午
    HinoeUma,
    /// 丁未
    HinotoHitsuji,
    /// 戊申
    TsuchinoeSaru,
    /// 己酉
    TsuchinotoTori,
    /// 庚戌
    KanoeInu,
    /// 辛亥
    KanotoYi,
    /// 壬子
    MizunoeNe,
    /// 癸丑
    MizunotoUshi,
    /// 甲寅
    KinoeTora,
    /// 乙卯
    KinotoWu,
    /// 丙辰
    HinoeTatsu,
    /// 丁巳
    HinotoMi,
    /// 戊午
    TsuchinoeUma,
    /// 己未
    TsuchinotoHitsuji,
    /// 庚申
    KanoeSaru,
    /// 辛酉
    KanotoTori,
    /// 壬戌
    MizunoeInu,
    /// 癸亥
    MizunotoYi,
}

impl SexagenaryCycle {
    /// Generate from Datelike of chrono.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use koyomi::SexagenaryCycle;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// assert_eq!(SexagenaryCycle::KinoeTatsu, SexagenaryCycle::from_datelike(&date));
    /// ```
    pub fn from_datelike<T: Datelike>(date: &T) -> Self {
        let hs = HeavenlyStem::from_datelike(date);
        let jz = JapaneseZodiac::from_datelike(date);
        let sc = format!("{}{}", hs.to_str(), jz.to_str());

        SexagenaryCycle::from_str(&sc).unwrap()
    }

    /// Generate from string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::SexagenaryCycle;
    ///
    /// assert!(SexagenaryCycle::from_str("丙午").is_some());
    /// ```
    pub fn from_str(name: &str) -> Option<Self> {
        SEXAGENARY_CYCLE
            .iter()
            .position(|&x| x == name)
            .and_then(|i| SexagenaryCycle::from_usize(i + 1))
    }

    /// Convert to string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use koyomi::SexagenaryCycle;
    ///
    /// assert_eq!("戊辰", SexagenaryCycle::TsuchinoeTatsu.to_str());
    /// ```
    pub const fn to_str(&self) -> &str {
        SEXAGENARY_CYCLE[self.to_usize() - 1]
    }

    const fn from_usize(num: usize) -> Option<Self> {
        match num {
            1 => Some(KinoeNe),
            2 => Some(KinotoUshi),
            3 => Some(HinoeTora),
            4 => Some(HinotoWu),
            5 => Some(TsuchinoeTatsu),
            6 => Some(TsuchinotoMi),
            7 => Some(KanoeUma),
            8 => Some(KanotoHitsuji),
            9 => Some(MizunoeSaru),
            10 => Some(MizunotoTori),
            11 => Some(KinoeInu),
            12 => Some(KinotoYi),
            13 => Some(HinoeNe),
            14 => Some(HinotoUshi),
            15 => Some(TsuchinoeTora),
            16 => Some(TsuchinotoWu),
            17 => Some(KanoeTatsu),
            18 => Some(KanotoMi),
            19 => Some(MizunoeUma),
            20 => Some(MizunotoHitsuji),
            21 => Some(KinoeSaru),
            22 => Some(KinotoTori),
            23 => Some(HinoeInu),
            24 => Some(HinotoYi),
            25 => Some(TsuchinoeNe),
            26 => Some(TsuchinotoUshi),
            27 => Some(KanoeTora),
            28 => Some(KanotoWu),
            29 => Some(MizunoeTatsu),
            30 => Some(MizunotoMi),
            31 => Some(KinoeUma),
            32 => Some(KinotoHitsuji),
            33 => Some(HinoeSaru),
            34 => Some(HinotoTori),
            35 => Some(TsuchinoeInu),
            36 => Some(TsuchinotoYi),
            37 => Some(KanoeNe),
            38 => Some(KanotoUshi),
            39 => Some(MizunoeTora),
            40 => Some(MizunotoWu),
            41 => Some(KinoeTatsu),
            42 => Some(KinotoMi),
            43 => Some(HinoeUma),
            44 => Some(HinotoHitsuji),
            45 => Some(TsuchinoeSaru),
            46 => Some(TsuchinotoTori),
            47 => Some(KanoeInu),
            48 => Some(KanotoYi),
            49 => Some(MizunoeNe),
            50 => Some(MizunotoUshi),
            51 => Some(KinoeTora),
            52 => Some(KinotoWu),
            53 => Some(HinoeTatsu),
            54 => Some(HinotoMi),
            55 => Some(TsuchinoeUma),
            56 => Some(TsuchinotoHitsuji),
            57 => Some(KanoeSaru),
            58 => Some(KanotoTori),
            59 => Some(MizunoeInu),
            60 => Some(MizunotoYi),
            _ => None,
        }
    }

    const fn to_usize(&self) -> usize {
        match self {
            KinoeNe => 1,
            KinotoUshi => 2,
            HinoeTora => 3,
            HinotoWu => 4,
            TsuchinoeTatsu => 5,
            TsuchinotoMi => 6,
            KanoeUma => 7,
            KanotoHitsuji => 8,
            MizunoeSaru => 9,
            MizunotoTori => 10,
            KinoeInu => 11,
            KinotoYi => 12,
            HinoeNe => 13,
            HinotoUshi => 14,
            TsuchinoeTora => 15,
            TsuchinotoWu => 16,
            KanoeTatsu => 17,
            KanotoMi => 18,
            MizunoeUma => 19,
            MizunotoHitsuji => 20,
            KinoeSaru => 21,
            KinotoTori => 22,
            HinoeInu => 23,
            HinotoYi => 24,
            TsuchinoeNe => 25,
            TsuchinotoUshi => 26,
            KanoeTora => 27,
            KanotoWu => 28,
            MizunoeTatsu => 29,
            MizunotoMi => 30,
            KinoeUma => 31,
            KinotoHitsuji => 32,
            HinoeSaru => 33,
            HinotoTori => 34,
            TsuchinoeInu => 35,
            TsuchinotoYi => 36,
            KanoeNe => 37,
            KanotoUshi => 38,
            MizunoeTora => 39,
            MizunotoWu => 40,
            KinoeTatsu => 41,
            KinotoMi => 42,
            HinoeUma => 43,
            HinotoHitsuji => 44,
            TsuchinoeSaru => 45,
            TsuchinotoTori => 46,
            KanoeInu => 47,
            KanotoYi => 48,
            MizunoeNe => 49,
            MizunotoUshi => 50,
            KinoeTora => 51,
            KinotoWu => 52,
            HinoeTatsu => 53,
            HinotoMi => 54,
            TsuchinoeUma => 55,
            TsuchinotoHitsuji => 56,
            KanoeSaru => 57,
            KanotoTori => 58,
            MizunoeInu => 59,
            MizunotoYi => 60,
        }
    }
}

#[cfg(test)]
mod tests_heavenly_stem {
    use super::HeavenlyStem;
    use super::HeavenlyStem::*;

    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    fn 年月日から変換できる() {
        let date = NaiveDate::from_ymd_opt(2012, 1, 1).unwrap();
        assert_eq!(HeavenlyStem::Kanoto, HeavenlyStem::from_datelike(&date));
    }

    #[rstest]
    #[case("甲", Kinoe)]
    #[case("乙", Kinoto)]
    #[case("丙", Hinoe)]
    #[case("丁", Hinoto)]
    #[case("戊", Tsuchinoe)]
    #[case("己", Tsuchinoto)]
    #[case("庚", Kanoe)]
    #[case("辛", Kanoto)]
    #[case("壬", Mizunoe)]
    #[case("癸", Mizunoto)]
    fn 十干の文字から変換できる(#[case] name: &str, #[case] expect: HeavenlyStem) {
        assert_eq!(Some(expect), HeavenlyStem::from_str(&name));
    }

    #[rstest]
    fn 十干の文字でなければ変換できない() {
        assert!(HeavenlyStem::from_str("").is_none());
    }

    #[rstest]
    #[case(1, Kinoe)]
    #[case(2, Kinoto)]
    #[case(3, Hinoe)]
    #[case(4, Hinoto)]
    #[case(5, Tsuchinoe)]
    #[case(6, Tsuchinoto)]
    #[case(7, Kanoe)]
    #[case(8, Kanoto)]
    #[case(9, Mizunoe)]
    #[case(10, Mizunoto)]
    fn 十干の順番から変換できる(#[case] num: usize, #[case] expect: HeavenlyStem) {
        assert_eq!(Some(expect), HeavenlyStem::from_usize(num));
    }

    #[rstest]
    fn 十干の順番範囲内でなければ変換できない() {
        assert!(HeavenlyStem::from_usize(11).is_none());
    }

    #[rstest]
    #[case(Kinoe, "甲")]
    #[case(Kinoto, "乙")]
    #[case(Hinoe, "丙")]
    #[case(Hinoto, "丁")]
    #[case(Tsuchinoe, "戊")]
    #[case(Tsuchinoto, "己")]
    #[case(Kanoe, "庚")]
    #[case(Kanoto, "辛")]
    #[case(Mizunoe, "壬")]
    #[case(Mizunoto, "癸")]
    fn 十干の文字に変換できる(#[case] stem: HeavenlyStem, #[case] expect: &str) {
        assert_eq!(expect, stem.to_str());
    }

    #[rstest]
    #[case(Kinoe, 1)]
    #[case(Kinoto, 2)]
    #[case(Hinoe, 3)]
    #[case(Hinoto, 4)]
    #[case(Tsuchinoe, 5)]
    #[case(Tsuchinoto, 6)]
    #[case(Kanoe, 7)]
    #[case(Kanoto, 8)]
    #[case(Mizunoe, 9)]
    #[case(Mizunoto, 10)]
    fn 十干の順番に変換できる(#[case] stem: HeavenlyStem, #[case] expect: usize) {
        assert_eq!(expect, stem.to_usize());
    }
}

#[cfg(test)]
mod tests_japanese_zodiac {
    use super::JapaneseZodiac;
    use super::JapaneseZodiac::*;

    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    #[case(2020, Ne)]
    #[case(2021, Ushi)]
    #[case(2022, Tora)]
    #[case(2023, Wu)]
    #[case(2024, Tatsu)]
    #[case(2025, Mi)]
    #[case(2026, Uma)]
    #[case(2027, Hitsuji)]
    #[case(2028, Saru)]
    #[case(2029, Tori)]
    #[case(2030, Inu)]
    #[case(2031, Yi)]
    fn 西暦から十二支を導出できる(#[case] year: i32, #[case] expect: JapaneseZodiac) {
        let date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        assert_eq!(expect, JapaneseZodiac::from_datelike(&date));
    }

    #[rstest]
    #[case("子", Ne)]
    #[case("丑", Ushi)]
    #[case("寅", Tora)]
    #[case("卯", Wu)]
    #[case("辰", Tatsu)]
    #[case("巳", Mi)]
    #[case("午", Uma)]
    #[case("未", Hitsuji)]
    #[case("申", Saru)]
    #[case("酉", Tori)]
    #[case("戌", Inu)]
    #[case("亥", Yi)]
    fn 十二支の文字から変換できる(#[case] name: &str, #[case] expect: JapaneseZodiac) {
        assert_eq!(Some(expect), JapaneseZodiac::from_str(name));
    }

    #[rstest]
    fn 十二支の文字でなければ変換できない() {
        assert!(JapaneseZodiac::from_str("").is_none());
    }

    #[rstest]
    #[case(1, Ne)]
    #[case(2, Ushi)]
    #[case(3, Tora)]
    #[case(4, Wu)]
    #[case(5, Tatsu)]
    #[case(6, Mi)]
    #[case(7, Uma)]
    #[case(8, Hitsuji)]
    #[case(9, Saru)]
    #[case(10, Tori)]
    #[case(11, Inu)]
    #[case(12, Yi)]
    fn 十二支の順番から変換できる(#[case] num: usize, #[case] expect: JapaneseZodiac) {
        assert_eq!(Some(expect), JapaneseZodiac::from_usize(num));
    }

    #[rstest]
    fn 十二支の順番範囲内でなければ変換できない() {
        assert!(JapaneseZodiac::from_usize(13).is_none());
    }

    #[rstest]
    #[case(Ne, "子")]
    #[case(Ushi, "丑")]
    #[case(Tora, "寅")]
    #[case(Wu, "卯")]
    #[case(Tatsu, "辰")]
    #[case(Mi, "巳")]
    #[case(Uma, "午")]
    #[case(Hitsuji, "未")]
    #[case(Saru, "申")]
    #[case(Tori, "酉")]
    #[case(Inu, "戌")]
    #[case(Yi, "亥")]
    fn 十二支の文字に変換できる(#[case] zodiac: JapaneseZodiac, #[case] expect: &str) {
        assert_eq!(expect, zodiac.to_str());
    }

    #[rstest]
    #[case(Ne, 1)]
    #[case(Ushi, 2)]
    #[case(Tora, 3)]
    #[case(Wu, 4)]
    #[case(Tatsu, 5)]
    #[case(Mi, 6)]
    #[case(Uma, 7)]
    #[case(Hitsuji, 8)]
    #[case(Saru, 9)]
    #[case(Tori, 10)]
    #[case(Inu, 11)]
    #[case(Yi, 12)]
    fn 十二支の順番に変換できる(#[case] zodiac: JapaneseZodiac, #[case] expect: usize) {
        assert_eq!(expect, zodiac.to_usize());
    }
}

#[cfg(test)]
mod tests_sexagenary_cycle {
    use super::SexagenaryCycle;
    use super::SexagenaryCycle::*;

    use chrono::NaiveDate;
    use rstest::rstest;

    #[rstest]
    fn 年月日から変換できる() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(
            SexagenaryCycle::KinoeTatsu,
            SexagenaryCycle::from_datelike(&date)
        );
    }

    #[rstest]
    #[case("甲子", KinoeNe)]
    #[case("乙丑", KinotoUshi)]
    #[case("丙寅", HinoeTora)]
    #[case("丁卯", HinotoWu)]
    #[case("戊辰", TsuchinoeTatsu)]
    #[case("己巳", TsuchinotoMi)]
    #[case("庚午", KanoeUma)]
    #[case("辛未", KanotoHitsuji)]
    #[case("壬申", MizunoeSaru)]
    #[case("癸酉", MizunotoTori)]
    #[case("甲戌", KinoeInu)]
    #[case("乙亥", KinotoYi)]
    #[case("丙子", HinoeNe)]
    #[case("丁丑", HinotoUshi)]
    #[case("戊寅", TsuchinoeTora)]
    #[case("己卯", TsuchinotoWu)]
    #[case("庚辰", KanoeTatsu)]
    #[case("辛巳", KanotoMi)]
    #[case("壬午", MizunoeUma)]
    #[case("癸未", MizunotoHitsuji)]
    #[case("甲申", KinoeSaru)]
    #[case("乙酉", KinotoTori)]
    #[case("丙戌", HinoeInu)]
    #[case("丁亥", HinotoYi)]
    #[case("戊子", TsuchinoeNe)]
    #[case("己丑", TsuchinotoUshi)]
    #[case("庚寅", KanoeTora)]
    #[case("辛卯", KanotoWu)]
    #[case("壬辰", MizunoeTatsu)]
    #[case("癸巳", MizunotoMi)]
    #[case("甲午", KinoeUma)]
    #[case("乙未", KinotoHitsuji)]
    #[case("丙申", HinoeSaru)]
    #[case("丁酉", HinotoTori)]
    #[case("戊戌", TsuchinoeInu)]
    #[case("己亥", TsuchinotoYi)]
    #[case("庚子", KanoeNe)]
    #[case("辛丑", KanotoUshi)]
    #[case("壬寅", MizunoeTora)]
    #[case("癸卯", MizunotoWu)]
    #[case("甲辰", KinoeTatsu)]
    #[case("乙巳", KinotoMi)]
    #[case("丙午", HinoeUma)]
    #[case("丁未", HinotoHitsuji)]
    #[case("戊申", TsuchinoeSaru)]
    #[case("己酉", TsuchinotoTori)]
    #[case("庚戌", KanoeInu)]
    #[case("辛亥", KanotoYi)]
    #[case("壬子", MizunoeNe)]
    #[case("癸丑", MizunotoUshi)]
    #[case("甲寅", KinoeTora)]
    #[case("乙卯", KinotoWu)]
    #[case("丙辰", HinoeTatsu)]
    #[case("丁巳", HinotoMi)]
    #[case("戊午", TsuchinoeUma)]
    #[case("己未", TsuchinotoHitsuji)]
    #[case("庚申", KanoeSaru)]
    #[case("辛酉", KanotoTori)]
    #[case("壬戌", MizunoeInu)]
    #[case("癸亥", MizunotoYi)]
    fn 六十干支の文字から変換できる(
        #[case] name: &str,
        #[case] expect: SexagenaryCycle,
    ) {
        assert_eq!(Some(expect), SexagenaryCycle::from_str(&name));
    }

    #[rstest]
    fn 六十干支の文字でなければ変換できない() {
        assert!(SexagenaryCycle::from_str("").is_none());
    }

    #[rstest]
    #[case(KinoeNe, "甲子")]
    #[case(KinotoUshi, "乙丑")]
    #[case(HinoeTora, "丙寅")]
    #[case(HinotoWu, "丁卯")]
    #[case(TsuchinoeTatsu, "戊辰")]
    #[case(TsuchinotoMi, "己巳")]
    #[case(KanoeUma, "庚午")]
    #[case(KanotoHitsuji, "辛未")]
    #[case(MizunoeSaru, "壬申")]
    #[case(MizunotoTori, "癸酉")]
    #[case(KinoeInu, "甲戌")]
    #[case(KinotoYi, "乙亥")]
    #[case(HinoeNe, "丙子")]
    #[case(HinotoUshi, "丁丑")]
    #[case(TsuchinoeTora, "戊寅")]
    #[case(TsuchinotoWu, "己卯")]
    #[case(KanoeTatsu, "庚辰")]
    #[case(KanotoMi, "辛巳")]
    #[case(MizunoeUma, "壬午")]
    #[case(MizunotoHitsuji, "癸未")]
    #[case(KinoeSaru, "甲申")]
    #[case(KinotoTori, "乙酉")]
    #[case(HinoeInu, "丙戌")]
    #[case(HinotoYi, "丁亥")]
    #[case(TsuchinoeNe, "戊子")]
    #[case(TsuchinotoUshi, "己丑")]
    #[case(KanoeTora, "庚寅")]
    #[case(KanotoWu, "辛卯")]
    #[case(MizunoeTatsu, "壬辰")]
    #[case(MizunotoMi, "癸巳")]
    #[case(KinoeUma, "甲午")]
    #[case(KinotoHitsuji, "乙未")]
    #[case(HinoeSaru, "丙申")]
    #[case(HinotoTori, "丁酉")]
    #[case(TsuchinoeInu, "戊戌")]
    #[case(TsuchinotoYi, "己亥")]
    #[case(KanoeNe, "庚子")]
    #[case(KanotoUshi, "辛丑")]
    #[case(MizunoeTora, "壬寅")]
    #[case(MizunotoWu, "癸卯")]
    #[case(KinoeTatsu, "甲辰")]
    #[case(KinotoMi, "乙巳")]
    #[case(HinoeUma, "丙午")]
    #[case(HinotoHitsuji, "丁未")]
    #[case(TsuchinoeSaru, "戊申")]
    #[case(TsuchinotoTori, "己酉")]
    #[case(KanoeInu, "庚戌")]
    #[case(KanotoYi, "辛亥")]
    #[case(MizunoeNe, "壬子")]
    #[case(MizunotoUshi, "癸丑")]
    #[case(KinoeTora, "甲寅")]
    #[case(KinotoWu, "乙卯")]
    #[case(HinoeTatsu, "丙辰")]
    #[case(HinotoMi, "丁巳")]
    #[case(TsuchinoeUma, "戊午")]
    #[case(TsuchinotoHitsuji, "己未")]
    #[case(KanoeSaru, "庚申")]
    #[case(KanotoTori, "辛酉")]
    #[case(MizunoeInu, "壬戌")]
    #[case(MizunotoYi, "癸亥")]
    fn 六十干支の文字に変換できる(
        #[case] cycle: SexagenaryCycle,
        #[case] expect: &str,
    ) {
        assert_eq!(expect, cycle.to_str());
    }
}
