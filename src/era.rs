use self::Epoc::*;
use self::Range::*;
use chrono::{Datelike, NaiveDate};

#[derive(Clone, Copy, PartialEq)]
pub enum Epoc {
    Reiwa,
    Heisei,
    Showa,
}

pub struct Era {
    epoc: Epoc,
    year: i32,
}

impl Era {
    pub fn from(date: &NaiveDate) -> Result<Self, String> {
        if within(Range::reiwa(), &date) {
            Ok(Era {
                epoc: Reiwa,
                year: date.year(),
            })
        } else if within(Range::heisei(), &date) {
            Ok(Era {
                epoc: Heisei,
                year: date.year(),
            })
        } else if within(Range::showa(), &date) {
            Ok(Era {
                epoc: Showa,
                year: date.year(),
            })
        } else {
            Err("昭和より前の元号は利用できません".to_string())
        }
    }

    pub fn epoc(&self) -> Epoc {
        self.epoc
    }

    pub fn name(&self) -> String {
        match self.epoc {
            Reiwa => "令和".to_string(),
            Heisei => "平成".to_string(),
            Showa => "昭和".to_string(),
        }
    }
}

enum Range {
    Current(NaiveDate),
    Recent(NaiveDate, NaiveDate),
}

impl Range {
    fn heisei() -> Self {
        Recent(
            NaiveDate::from_ymd_opt(1989, 1, 8).unwrap(),
            NaiveDate::from_ymd_opt(2019, 4, 30).unwrap(),
        )
    }

    fn reiwa() -> Self {
        Current(NaiveDate::from_ymd_opt(2019, 5, 1).unwrap())
    }

    fn showa() -> Self {
        Recent(
            NaiveDate::from_ymd_opt(1926, 12, 25).unwrap(),
            NaiveDate::from_ymd_opt(1989, 1, 7).unwrap(),
        )
    }
}

fn within(range: Range, date: &NaiveDate) -> bool {
    match range {
        Recent(from, until) => &from <= date && date <= &until,
        Current(from) => &from <= date,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::*;

    #[rstest]
    #[case(1926, 12, 24, false)]
    #[case(1926, 12, 25, true)]
    #[case(1989, 1, 7, true)]
    #[case(1989, 1, 8, false)]
    fn 昭和の範囲を判定できる(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: bool,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(expect, within(Range::showa(), &date));
    }

    #[rstest]
    #[case(1989, 1, 7, false)]
    #[case(1989, 1, 8, true)]
    #[case(2019, 4, 30, true)]
    #[case(2019, 5, 1, false)]
    fn 平成の範囲を判定できる(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: bool,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(expect, within(Range::heisei(), &date));
    }

    #[rstest]
    #[case(2019, 4, 30, false)]
    #[case(2019, 5, 1, true)]
    fn 令和の範囲を判定できる(
        #[case] year: i32,
        #[case] month: u32,
        #[case] day: u32,
        #[case] expect: bool,
    ) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(expect, within(Range::reiwa(), &date));
    }
}
