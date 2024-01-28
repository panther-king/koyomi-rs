use self::Holiday::*;
use chrono::{Datelike, NaiveDate};
use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub enum Holiday {
    AutumnalEquinoxDay,
    ChildrensDay,
    ComingOfAgeDay,
    ConstitutionDay,
    CultureDay,
    EmperorsBirthday,
    GreenDay,
    ImperialCeremony(String),
    LaborThanksgivingDay,
    MarineDay,
    MountainDay,
    NationalFoundationDay,
    NewYearsDay,
    PhysicalEducationDay,
    RespectForTheAgeDay,
    ShowaDay,
    SportsDay,
    SubstituteDay,
    VernalEquinoxDay,
}

impl Holiday {
    pub fn name(&self) -> &str {
        match self {
            AutumnalEquinoxDay => "秋分の日",
            ChildrensDay => "こどもの日",
            ComingOfAgeDay => "成人の日",
            ConstitutionDay => "憲法記念日",
            CultureDay => "文化の日",
            EmperorsBirthday => "天皇誕生日",
            GreenDay => "みどりの日",
            ImperialCeremony(ceremony) => ceremony,
            LaborThanksgivingDay => "勤労感謝の日",
            MarineDay => "海の日",
            MountainDay => "山の日",
            NationalFoundationDay => "建国記念の日",
            NewYearsDay => "元日",
            PhysicalEducationDay => "体育の日",
            RespectForTheAgeDay => "敬老の日",
            ShowaDay => "昭和の日",
            SportsDay => "スポーツの日",
            SubstituteDay => "振替休日",
            VernalEquinoxDay => "春分の日",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Month {
    January,
    Februray,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn name(&self) -> &str {
        match self {
            Month::January => "睦月",
            Month::Februray => "如月",
            Month::March => "弥生",
            Month::April => "卯月",
            Month::May => "皐月",
            Month::June => "水無月",
            Month::July => "文月",
            Month::August => "葉月",
            Month::September => "長月",
            Month::October => "神無月",
            Month::November => "霜月",
            Month::December => "師走",
        }
    }
}

#[derive(Debug)]
pub struct Koyomi {
    year: i32,
    month: u32,
    day: u32,
    weekday: u32,
}

impl Koyomi {
    pub fn autumnal_equinox_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1949..,
                month: 9,
                ..
            } if autumnal_equinox_day(&self) => Some(AutumnalEquinoxDay),
            _ => None,
        }
    }

    pub fn childrens_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1948..,
                month: 5,
                day: 5,
                ..
            } => Some(ChildrensDay),
            _ => None,
        }
    }

    pub fn coming_of_age_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1949..=1999,
                month: 1,
                day: 15,
                ..
            } => Some(ComingOfAgeDay),
            Self {
                year: 2000..,
                month: 1,
                ..
            } if happy_monday_second(&self) => Some(ComingOfAgeDay),
            _ => None,
        }
    }

    pub fn constitution_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1948..,
                month: 5,
                day: 3,
                ..
            } => Some(ConstitutionDay),
            _ => None,
        }
    }

    pub fn culture_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1948..,
                month: 11,
                day: 3,
                ..
            } => Some(CultureDay),
            _ => None,
        }
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn emperors_birthday(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1949..=1988,
                month: 4,
                day: 29,
                ..
            } => Some(EmperorsBirthday),
            Self {
                year: 1989..=2018,
                month: 12,
                day: 23,
                ..
            } => Some(EmperorsBirthday),
            Self {
                year: 2020..,
                month: 2,
                day: 23,
                ..
            } => Some(EmperorsBirthday),
            _ => None,
        }
    }

    pub fn green_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1989..=2006,
                month: 4,
                day: 29,
                ..
            } => Some(GreenDay),
            Self {
                year: 2007..,
                month: 5,
                day: 4,
                ..
            } => Some(GreenDay),
            _ => None,
        }
    }

    pub fn holiday(&self) -> Option<Holiday> {
        self.holiday_without_substitute()
            .or(self.substitute_holiday())
    }

    pub fn holiday_without_substitute(&self) -> Option<Holiday> {
        self.autumnal_equinox_day()
            .or(self.childrens_day())
            .or(self.coming_of_age_day())
            .or(self.constitution_day())
            .or(self.culture_day())
            .or(self.emperors_birthday())
            .or(self.green_day())
            .or(self.imperial_ceremony())
            .or(self.labor_thanksgiving_day())
            .or(self.marine_day())
            .or(self.mountain_day())
            .or(self.national_foundation_day())
            .or(self.new_years_day())
            .or(self.physical_education_day())
            .or(self.respect_for_the_age_day())
            .or(self.showa_day())
            .or(self.sports_day())
            .or(self.vernal_equinox_day())
    }

    pub fn imperial_ceremony(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1959,
                month: 4,
                day: 10,
                ..
            } => Some(ImperialCeremony("明仁親王の結婚の儀".to_string())),
            Self {
                year: 1989,
                month: 2,
                day: 24,
                ..
            } => Some(ImperialCeremony("昭和天皇大喪の礼".to_string())),
            Self {
                year: 1990,
                month: 11,
                day: 12,
                ..
            } => Some(ImperialCeremony("即位礼正殿の儀".to_string())),
            Self {
                year: 1993,
                month: 6,
                day: 9,
                ..
            } => Some(ImperialCeremony("徳仁親王の結婚の儀".to_string())),
            Self {
                year: 2019,
                month: 5,
                day: 1,
                ..
            } => Some(ImperialCeremony("令和天皇即位".to_string())),
            Self {
                year: 2019,
                month: 10,
                day: 22,
                ..
            } => Some(ImperialCeremony("即位礼正殿の儀".to_string())),
            _ => None,
        }
    }

    pub fn labor_thanksgiving_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1948..,
                month: 11,
                day: 23,
                ..
            } => Some(LaborThanksgivingDay),
            _ => None,
        }
    }

    pub fn marine_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 2020,
                month: 7,
                day: 23,
                ..
            } => Some(MarineDay),
            Self {
                year: 2021,
                month: 7,
                day: 22,
                ..
            } => Some(MarineDay),
            Self {
                year: 1996..=2002,
                month: 7,
                day: 20,
                ..
            } => Some(MarineDay),
            Self {
                year: 2003..,
                month: 7,
                ..
            } if happy_monday_third(&self) => Some(MarineDay),
            _ => None,
        }
    }

    pub fn mountain_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 2020,
                month: 8,
                day: 10,
                ..
            } => Some(MountainDay),
            Self {
                year: 2021,
                month: 8,
                day: 8,
                ..
            } => Some(MountainDay),
            Self {
                year: 2016..,
                month: 8,
                day: 11,
                ..
            } => Some(MountainDay),
            _ => None,
        }
    }

    pub fn month(&self) -> Month {
        match self.month {
            1 => Month::January,
            2 => Month::Februray,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => unreachable!(),
        }
    }

    pub fn national_foundation_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1967..,
                month: 2,
                day: 11,
                ..
            } => Some(NationalFoundationDay),
            _ => None,
        }
    }

    pub fn new_years_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1949..,
                month: 1,
                day: 1,
                ..
            } => Some(NewYearsDay),
            _ => None,
        }
    }

    pub fn physical_education_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1966..=1999,
                month: 10,
                day: 10,
                ..
            } => Some(PhysicalEducationDay),
            Self {
                year: 2000..=2019,
                month: 10,
                ..
            } if happy_monday_second(&self) => Some(PhysicalEducationDay),
            _ => None,
        }
    }

    pub fn respect_for_the_age_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1966..=2002,
                month: 9,
                day: 15,
                ..
            } => Some(RespectForTheAgeDay),
            Self {
                year: 2003..,
                month: 9,
                ..
            } if happy_monday_third(&self) => Some(RespectForTheAgeDay),
            _ => None,
        }
    }

    pub fn showa_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 2007..,
                month: 4,
                day: 29,
                ..
            } => Some(ShowaDay),
            _ => None,
        }
    }

    pub fn sports_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 2020,
                month: 7,
                day: 24,
                ..
            } => Some(SportsDay),
            Self {
                year: 2021,
                month: 7,
                day: 23,
                ..
            } => Some(SportsDay),
            Self {
                year: 2020..,
                month: 10,
                ..
            } if happy_monday_second(&self) => Some(SportsDay),
            _ => None,
        }
    }

    pub fn substitute_holiday(&self) -> Option<Holiday> {
        let enforced = NaiveDate::from_ymd_opt(1973, 4, 30).unwrap();
        let date = NaiveDate::from_ymd_opt(self.year, self.month, self.day).unwrap();

        if date < enforced {
            None
        } else {
            match self {
                Self { year: y, .. } if *y <= 1973 => None,
                _ => NaiveDate::from_ymd_opt(self.year, self.month, self.day)
                    .and_then(|c| c.pred_opt())
                    .map(|c| Self::from(&c))
                    .and_then(|k| match k.holiday_without_substitute() {
                        None => None,
                        Some(_) if k.weekday == 6 => Some(SubstituteDay),
                        Some(_) => k.substitute_holiday(),
                    }),
            }
        }
    }

    pub fn vernal_equinox_day(&self) -> Option<Holiday> {
        match self {
            Self {
                year: 1949..,
                month: 3,
                ..
            } if vernal_equinox_day(&self) => Some(VernalEquinoxDay),
            _ => None,
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }
}

impl From<&NaiveDate> for Koyomi {
    fn from(item: &NaiveDate) -> Self {
        Self {
            year: item.year(),
            month: item.month(),
            day: item.day(),
            weekday: item.weekday().num_days_from_monday(),
        }
    }
}

fn autumnal_equinox_day(koyomi: &Koyomi) -> bool {
    koyomi.day == equinox_day(23.2488, koyomi.year)
}

fn vernal_equinox_day(koyomi: &Koyomi) -> bool {
    koyomi.day == equinox_day(20.8431, koyomi.year)
}

fn equinox_day(equinox: f64, year: i32) -> u32 {
    let x = (year - 1980) as f64;
    let y = ((0.242194_f64 * x) + equinox).floor() as i32;
    let z = (x / 4.0_f64).floor() as i32;

    (y - z).abs() as u32
}

fn happy_monday_second(koyomi: &Koyomi) -> bool {
    match (koyomi.weekday, koyomi.day) {
        (0, 8..=14) => true,
        _ => false,
    }
}

fn happy_monday_third(koyomi: &Koyomi) -> bool {
    match (koyomi.weekday, koyomi.day) {
        (0, 15..=21) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests_new_years_day {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後1月1日は元日である() {
        let date = NaiveDate::from_ymd_opt(1949, 1, 1).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(NewYearsDay), koyomi.new_years_day());
    }

    #[rstest]
    fn 祝日法の施行以前は1月1日であっても元日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 1, 1).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(NewYearsDay), koyomi.new_years_day());
    }
}

#[cfg(test)]
mod tests_coming_of_age_day {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後1月15日は成人の日である() {
        let date = NaiveDate::from_ymd_opt(1949, 1, 15).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(ComingOfAgeDay), koyomi.coming_of_age_day());
    }

    #[rstest]
    fn 祝日法の施行以前は1月15日であっても成人の日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 1, 15).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(ComingOfAgeDay), koyomi.coming_of_age_day());
    }

    #[rstest]
    fn 祝日法の改正後は1月の第2月曜が成人の日である() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 10).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(ComingOfAgeDay), koyomi.coming_of_age_day());
    }

    #[rstest]
    fn 祝日法の改正後は1月15日であっても第2月曜でなければ成人の日ではない() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 15).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(ComingOfAgeDay), koyomi.coming_of_age_day());
    }
}

#[cfg(test)]
mod tests_national_foundation_day {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の改正後は2月15日が建国記念の日である() {
        let date = NaiveDate::from_ymd_opt(1967, 2, 11).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(NationalFoundationDay),
            koyomi.national_foundation_day()
        );
    }

    #[rstest]
    fn 祝日法の改正以前は2月15日であっても建国記念の日ではない() {
        let date = NaiveDate::from_ymd_opt(1966, 2, 15).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(
            Some(NationalFoundationDay),
            koyomi.national_foundation_day()
        );
    }
}

#[cfg(test)]
mod tests_emperors_birthday {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1949)]
    #[case(1988)]
    fn 祝日法の施行後で昭和天皇在位時は4月29日が天皇誕生日になる(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 4, 29).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(EmperorsBirthday), koyomi.emperors_birthday());
    }

    #[rstest]
    #[case(1989)]
    #[case(2018)]
    fn 祝日法の施行後で平成天皇在位時は12月23日が天皇誕生日になる(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 12, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(EmperorsBirthday), koyomi.emperors_birthday());
    }

    #[rstest]
    #[case(2020)]
    fn 祝日法の施行後で令和天皇在位時は2月23日が天皇誕生日になる(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 2, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(EmperorsBirthday), koyomi.emperors_birthday());
    }

    #[rstest]
    fn 祝日法の施行前は昭和天皇在位時の4月29日でも天皇誕生日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 4, 29).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(EmperorsBirthday), koyomi.emperors_birthday());
    }

    #[rstest]
    #[case(2, 23)]
    #[case(12, 23)]
    fn 天皇退位の2019年は天皇誕生日が存在しない(#[case] m: u32, #[case] d: u32) {
        let date = NaiveDate::from_ymd_opt(2019, m, d).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(EmperorsBirthday), koyomi.emperors_birthday());
    }
}

#[cfg(test)]
mod vernal_equinox_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の春分日は春分の日である() {
        let date = NaiveDate::from_ymd_opt(1949, 3, 21).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(VernalEquinoxDay), koyomi.vernal_equinox_day());
    }

    #[rstest]
    fn 祝日法施行以前は春分日であっても春分の日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 3, 21).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(VernalEquinoxDay), koyomi.vernal_equinox_day());
    }
}

#[cfg(test)]
mod green_days_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1989)]
    #[case(2006)]
    fn 昭和天皇崩御から昭和の日制定まで4月29日はみどりの日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 4, 29).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(GreenDay), koyomi.green_day());
    }

    #[rstest]
    fn 昭和の日制定後は5月4日がみどりの日である() {
        let date = NaiveDate::from_ymd_opt(2007, 5, 4).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(GreenDay), koyomi.green_day());
    }

    #[rstest]
    fn 昭和天皇在位時の4月29日はみどりの日ではない() {
        let date = NaiveDate::from_ymd_opt(1988, 4, 29).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(GreenDay), koyomi.green_day());
    }
}

#[cfg(test)]
mod showa_days_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 昭和の日が制定された2007年以降は4月29日が昭和の日である() {
        let date = NaiveDate::from_ymd_opt(2007, 4, 29).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(ShowaDay), koyomi.showa_day());
    }

    #[rstest]
    fn 昭和の日制定以前の4月29日は昭和の日ではない() {
        let date = NaiveDate::from_ymd_opt(2006, 4, 29).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(ShowaDay), koyomi.showa_day());
    }
}

#[cfg(test)]
mod constitution_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後5月3日は憲法記念日である() {
        let date = NaiveDate::from_ymd_opt(1948, 5, 3).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(ConstitutionDay), koyomi.constitution_day());
    }

    #[rstest]
    fn 祝日法施行以前の5月3日は憲法記念日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 5, 3).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(ConstitutionDay), koyomi.constitution_day());
    }
}

#[cfg(test)]
mod childrens_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法の施行後5月5日はこどもの日である() {
        let date = NaiveDate::from_ymd_opt(1948, 5, 5).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(ChildrensDay), koyomi.childrens_day());
    }

    #[rstest]
    fn 祝日法施行以前の5月5日はこどもの日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 5, 5).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(ChildrensDay), koyomi.childrens_day());
    }
}

#[cfg(test)]
mod marin_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1996)]
    #[case(2002)]
    fn 制定年の1996年からハッピーマンデー導入の2002年まで7月20日は海の日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 7, 20).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MarineDay), koyomi.marine_day());
    }

    #[rstest]
    fn 制定以前の7月20日は海の日ではない() {
        let date = NaiveDate::from_ymd_opt(1995, 7, 20).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(MarineDay), koyomi.marine_day());
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
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MarineDay), koyomi.marine_day());
    }

    #[rstest]
    fn 東京五輪の特措法に基づき2020年は7月23日が海の日である() {
        let date = NaiveDate::from_ymd_opt(2020, 7, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MarineDay), koyomi.marine_day());
    }

    #[rstest]
    fn 東京オリンピック開催年の2021年は開会式前日の7月22日が海の日である() {
        let date = NaiveDate::from_ymd_opt(2021, 7, 22).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MarineDay), koyomi.marine_day());
    }

    #[rstest]
    fn 東京オリンピック以降は7月の第3月曜が海の日である() {
        let date = NaiveDate::from_ymd_opt(2022, 7, 18).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MarineDay), koyomi.marine_day());
    }
}

#[cfg(test)]
mod mountain_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(2016)]
    #[case(2019)]
    fn 制定年の2016年から2019年まで8月11日は山の日である(#[case] y: i32) {
        let date = NaiveDate::from_ymd_opt(y, 8, 11).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MountainDay), koyomi.mountain_day());
    }

    #[rstest]
    fn 制定以前の8月11日は山の日ではない() {
        let date = NaiveDate::from_ymd_opt(2015, 8, 11).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(MountainDay), koyomi.mountain_day());
    }

    #[rstest]
    fn 東京五輪の特措法に基づき2020年は8月10日が山の日である() {
        let date = NaiveDate::from_ymd_opt(2020, 8, 10).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MountainDay), koyomi.mountain_day());
    }

    #[rstest]
    fn 東京オリンピック開催年の2021年は閉会式翌日の8月8日が山の日である() {
        let date = NaiveDate::from_ymd_opt(2021, 8, 8).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MountainDay), koyomi.mountain_day());
    }

    #[rstest]
    fn 東京オリンピック以降は8月11日が山の日である() {
        let date = NaiveDate::from_ymd_opt(2022, 8, 11).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(MountainDay), koyomi.mountain_day());
    }
}

#[cfg(test)]
mod respect_for_the_age_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1966)]
    #[case(2002)]
    fn 制定年の1966年からハッピーマンデー導入の2002年まで9月15日は敬老の日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 9, 15).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(RespectForTheAgeDay), koyomi.respect_for_the_age_day());
    }

    #[rstest]
    fn 制定以前の9月15日は敬老の日ではない() {
        let date = NaiveDate::from_ymd_opt(1965, 9, 15).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(RespectForTheAgeDay), koyomi.respect_for_the_age_day());
    }

    #[rstest]
    fn ハッピーマンデー導入後は9月の第3月曜が敬老の日である() {
        let date = NaiveDate::from_ymd_opt(2022, 9, 19).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(RespectForTheAgeDay), koyomi.respect_for_the_age_day());
    }
}

#[cfg(test)]
mod autumnal_equinox_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の秋分日は秋分の日である() {
        let date = NaiveDate::from_ymd_opt(1949, 9, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(AutumnalEquinoxDay), koyomi.autumnal_equinox_day());
    }

    #[rstest]
    fn 祝日法施行以前は秋分日であっても秋分の日ではない() {
        let date = NaiveDate::from_ymd_opt(1948, 9, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(AutumnalEquinoxDay), koyomi.autumnal_equinox_day());
    }
}

#[cfg(test)]
mod physical_education_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1966)]
    #[case(1999)]
    fn 制定年の1966年からハッピーマンデー導入の1999年まで10月10日は体育の日である(
        #[case] y: i32,
    ) {
        let date = NaiveDate::from_ymd_opt(y, 10, 10).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(PhysicalEducationDay), koyomi.physical_education_day());
    }

    #[rstest]
    fn 制定以前の10月10日は体育の日ではない() {
        let date = NaiveDate::from_ymd_opt(1965, 10, 10).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(PhysicalEducationDay), koyomi.physical_education_day());
    }

    #[rstest]
    fn ハッピーマンデー導入後は10月の第2月曜が体育の日である() {
        let date = NaiveDate::from_ymd_opt(2000, 10, 9).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(PhysicalEducationDay), koyomi.physical_education_day());
    }
}

#[cfg(test)]
mod sports_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 東京五輪の特措法に基づき2020年は7月24日がスポーツの日である() {
        let date = NaiveDate::from_ymd_opt(2020, 7, 24).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(SportsDay), koyomi.sports_day());
    }

    #[rstest]
    fn 東京オリンピック開催年の2021年は7月23日がスポーツの日である() {
        let date = NaiveDate::from_ymd_opt(2021, 7, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(SportsDay), koyomi.sports_day());
    }

    #[rstest]
    fn 東京オリンピック以降は10月の第2月曜がスポーツの日である() {
        let date = NaiveDate::from_ymd_opt(2022, 10, 10).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(SportsDay), koyomi.sports_day());
    }

    #[rstest]
    fn 制定以前の10月第2月曜はスポーツの日ではない() {
        let date = NaiveDate::from_ymd_opt(2019, 10, 14).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(SportsDay), koyomi.sports_day());
    }
}

#[cfg(test)]
mod culture_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の11月3日は文化の日である() {
        let date = NaiveDate::from_ymd_opt(1948, 11, 3).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(CultureDay), koyomi.culture_day());
    }

    #[rstest]
    fn 祝日法の施行以前は11月3日であっても文化の日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 11, 3).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(CultureDay), koyomi.culture_day());
    }
}

#[cfg(test)]
mod labor_thanksgiving_day_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 祝日法施行後の11月23日は勤労感謝の日である() {
        let date = NaiveDate::from_ymd_opt(1948, 11, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(LaborThanksgivingDay), koyomi.labor_thanksgiving_day());
    }

    #[rstest]
    fn 祝日法の施行以前は11月23日であっても勤労感謝の日ではない() {
        let date = NaiveDate::from_ymd_opt(1947, 11, 23).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(LaborThanksgivingDay), koyomi.labor_thanksgiving_day());
    }
}

#[cfg(test)]
mod imperial_ceremony_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 明仁親王の結婚の儀が行われた1959年4月10日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1959, 4, 10).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(ImperialCeremony("明仁親王の結婚の儀".to_string())),
            koyomi.imperial_ceremony()
        );
    }

    #[rstest]
    fn 昭和天皇大喪の礼が行われた1989年2月24日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1989, 2, 24).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(ImperialCeremony("昭和天皇大喪の礼".to_string())),
            koyomi.imperial_ceremony()
        );
    }

    #[rstest]
    fn 平成天皇の即位礼正殿の儀が行われた1990年11月12日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1990, 11, 12).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(ImperialCeremony("即位礼正殿の儀".to_string())),
            koyomi.imperial_ceremony()
        );
    }

    #[rstest]
    fn 徳仁親王の結婚の儀が行われた1993年6月9日は祝日である() {
        let date = NaiveDate::from_ymd_opt(1993, 6, 9).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(ImperialCeremony("徳仁親王の結婚の儀".to_string())),
            koyomi.imperial_ceremony()
        );
    }

    #[rstest]
    fn 令和天皇が即位した2019年5月1日は祝日である() {
        let date = NaiveDate::from_ymd_opt(2019, 5, 1).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(ImperialCeremony("令和天皇即位".to_string())),
            koyomi.imperial_ceremony()
        );
    }

    #[rstest]
    fn 令和天皇の即位礼正殿の儀が行われた2019年10月22日は祝日である() {
        let date = NaiveDate::from_ymd_opt(2019, 10, 22).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(
            Some(ImperialCeremony("即位礼正殿の儀".to_string())),
            koyomi.imperial_ceremony()
        );
    }
}

#[cfg(test)]
mod substitute_holiday_tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    fn 日曜が祝日の場合は次の月曜が振替休日である() {
        let date = NaiveDate::from_ymd_opt(2021, 8, 9).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(SubstituteDay), koyomi.substitute_holiday());
    }

    #[rstest]
    fn 日曜の次に祝日が連続する場合は祝日の次の平日が振替休日である() {
        let date = NaiveDate::from_ymd_opt(2020, 5, 6).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_eq!(Some(SubstituteDay), koyomi.substitute_holiday());
    }

    #[rstest]
    fn 祝日法の改正前は日曜が祝日であっても次の月曜は振替休日ではない() {
        let date = NaiveDate::from_ymd_opt(1973, 2, 12).unwrap();
        let koyomi = Koyomi::from(&date);
        assert_ne!(Some(SubstituteDay), koyomi.substitute_holiday());
    }
}
