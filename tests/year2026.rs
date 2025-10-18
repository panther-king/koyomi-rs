extern crate chrono;
extern crate koyomi_rs;
extern crate rstest;

use chrono::NaiveDate;
use koyomi_rs::prelude::*;

fn assert_calendar(jd: JapaneseDate, y: i32, m: u32, d: u32, w: &str, h: Option<&str>) {
    assert_eq!(y, jd.western_year());
    assert_eq!(m, jd.month_number());
    assert_eq!(d, jd.day());
    assert_eq!(w, jd.weekday_name());
    assert_eq!(h, jd.holiday_name());
}

#[test]
fn 西暦2026年はひのえうまである() {
    let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    assert_eq!("丙午", SexagenaryCycle::from_datelike(&date).name());
}

#[test]
fn 西暦2026年は午年である() {
    let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    assert_eq!("午", JapaneseZodiac::from_datelike(&date).name());
}

#[test]
fn 西暦2026年は令和8年である() {
    let chrono_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let japanese_date = JapaneseDate::from_datelike(&chrono_date);

    assert_eq!(JapaneseEra::Reiwa(8), japanese_date.era().unwrap());
}

#[rustfmt::skip]
#[test]
fn 西暦2026年1月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 1).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 1, 1, "木", Some("元日"));
    assert_calendar(k.next().unwrap(), 2026, 1, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 3, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 4, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 5, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 6, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 11, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 12, "月", Some("成人の日"));
    assert_calendar(k.next().unwrap(), 2026, 1, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 19, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 23, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 29, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 30, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 1, 31, "土", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2026年2月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 2).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 2, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 3, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 11, "水", Some("建国記念の日"));
    assert_calendar(k.next().unwrap(), 2026, 2, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 16, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 20, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 22, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 23, "月", Some("天皇誕生日"));
    assert_calendar(k.next().unwrap(), 2026, 2, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 2, 28, "土", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2026年3月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 3).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 3, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 3, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 11, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 16, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 20, "金", Some("春分の日"));
    assert_calendar(k.next().unwrap(), 2026, 3, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 22, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 23, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 28, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 29, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 30, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 3, 31, "火", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2026年4月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 4).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 4, 1, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 3, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 4, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 5, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 6, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 13, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 20, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 23, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 4, 29, "水", Some("昭和の日"));
    assert_calendar(k.next().unwrap(), 2026, 4, 30, "木", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2026年5月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 5).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 5, 1, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 2, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 3, "日", Some("憲法記念日"));
    assert_calendar(k.next().unwrap(), 2026, 5, 4, "月", Some("みどりの日"));
    assert_calendar(k.next().unwrap(), 2026, 5, 5, "火", Some("こどもの日"));
    assert_calendar(k.next().unwrap(), 2026, 5, 6, "水", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2026, 5, 7, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 8, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 9, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 10, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 11, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 12, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 13, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 14, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 15, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 16, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 17, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 18, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 19, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 20, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 21, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 22, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 23, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 24, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 25, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 26, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 27, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 28, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 29, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 30, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 5, 31, "日", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2026年6月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 6).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 6, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 23, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 6, 30, "火", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2026年7月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 7).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 7, 1, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 3, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 4, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 5, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 6, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 13, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 20, "月", Some("海の日"));
    assert_calendar(k.next().unwrap(), 2026, 7, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 23, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 29, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 30, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 7, 31, "金", None);
}

#[test]
fn 西暦2026年8月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 8).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 8, 1, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 2, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 3, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 4, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 5, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 6, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 7, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 8, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 9, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 10, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 11, "火", Some("山の日"));
    assert_calendar(k.next().unwrap(), 2026, 8, 12, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 13, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 14, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 15, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 16, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 17, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 18, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 19, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 20, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 21, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 22, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 23, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 24, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 25, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 26, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 27, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 28, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 29, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 30, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 8, 31, "月", None);
}

#[test]
fn 西暦2026年9月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 9).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 9, 1, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 2, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 3, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 4, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 5, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 6, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 7, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 8, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 9, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 10, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 11, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 12, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 13, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 14, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 15, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 16, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 17, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 18, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 19, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 20, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 21, "月", Some("敬老の日"));
    assert_calendar(k.next().unwrap(), 2026, 9, 22, "火", Some("国民の休日"));
    assert_calendar(k.next().unwrap(), 2026, 9, 23, "水", Some("秋分の日"));
    assert_calendar(k.next().unwrap(), 2026, 9, 24, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 25, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 26, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 27, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 28, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 29, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 9, 30, "水", None);
}

#[test]
fn 西暦2026年10月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 10).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 10, 1, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 3, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 4, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 5, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 6, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 11, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 12, "月", Some("スポーツの日"));
    assert_calendar(k.next().unwrap(), 2026, 10, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 19, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 23, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 29, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 30, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 10, 31, "土", None);
}

#[test]
fn 西暦2026年11月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 11).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 11, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 3, "火", Some("文化の日"));
    assert_calendar(k.next().unwrap(), 2026, 11, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 11, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 16, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 20, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 22, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 23, "月", Some("勤労感謝の日"));
    assert_calendar(k.next().unwrap(), 2026, 11, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 28, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 29, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 11, 30, "月", None);
}

#[test]
fn 西暦2026年12月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2026, 12).unwrap();

    assert_calendar(k.next().unwrap(), 2026, 12, 1, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 2, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 3, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 4, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 5, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 6, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 7, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 8, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 9, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 10, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 11, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 12, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 13, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 14, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 15, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 16, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 17, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 18, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 19, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 20, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 21, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 22, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 23, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 24, "木", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 25, "金", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 26, "土", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 27, "日", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 28, "月", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 29, "火", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 30, "水", None);
    assert_calendar(k.next().unwrap(), 2026, 12, 31, "木", None);
}
