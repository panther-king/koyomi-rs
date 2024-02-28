extern crate chrono;
extern crate koyomi;
extern crate rstest;

use chrono::NaiveDate;
use koyomi::prelude::*;

fn assert_calendar(jd: JapaneseDate, y: i32, m: u32, d: u32, w: &str, h: Option<&str>) {
    assert_eq!(y, jd.western_year());
    assert_eq!(m, jd.month_number());
    assert_eq!(d, jd.day());
    assert_eq!(w, jd.weekday_name());
    assert_eq!(h, jd.holiday_name());
}

#[test]
fn 西暦2024年はきのえたつである() {
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    assert_eq!("甲辰", SexagenaryCycle::from_datelike(&date).name());
}

#[test]
fn 西暦2024年は辰年である() {
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    assert_eq!("辰", JapaneseZodiac::from_datelike(&date).name());
}

#[test]
fn 西暦2024年は令和6年である() {
    let chrono_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let japanese_date = JapaneseDate::from_datelike(&chrono_date);

    assert_eq!(JapaneseEra::Reiwa(6), japanese_date.era().unwrap());
}

#[rustfmt::skip]
#[test]
fn 西暦2024年1月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 1).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 1, 1, "月", Some("元日"));
    assert_calendar(k.next().unwrap(), 2024, 1, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 8, "月", Some("成人の日"));
    assert_calendar(k.next().unwrap(), 2024, 1, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 23, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 30, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 1, 31, "水", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2024年2月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 2).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 2, 1, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 3, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 4, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 5, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 6, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 11, "日", Some("建国記念の日"));
    assert_calendar(k.next().unwrap(), 2024, 2, 12, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2024, 2, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 19, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 23, "金", Some("天皇誕生日"));
    assert_calendar(k.next().unwrap(), 2024, 2, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 2, 29, "木", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2024年3月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 3).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 3, 1, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 2, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 3, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 4, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 5, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 6, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 7, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 8, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 9, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 10, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 11, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 12, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 13, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 14, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 15, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 16, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 17, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 18, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 19, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 20, "水", Some("春分の日"));
    assert_calendar(k.next().unwrap(), 2024, 3, 21, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 22, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 23, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 24, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 25, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 26, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 27, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 28, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 29, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 30, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 3, 31, "日", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2024年4月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 4).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 4, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 23, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 4, 29, "月", Some("昭和の日"));
    assert_calendar(k.next().unwrap(), 2024, 4, 30, "火", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2024年5月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 5).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 5, 1, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 3, "金", Some("憲法記念日"));
    assert_calendar(k.next().unwrap(), 2024, 5, 4, "土", Some("みどりの日"));
    assert_calendar(k.next().unwrap(), 2024, 5, 5, "日", Some("こどもの日"));
    assert_calendar(k.next().unwrap(), 2024, 5, 6, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2024, 5, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 13, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 20, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 23, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 29, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 30, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 5, 31, "金", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2024年6月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 6).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 6, 1, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 2, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 3, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 4, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 5, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 6, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 7, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 8, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 9, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 10, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 11, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 12, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 13, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 14, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 15, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 16, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 17, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 18, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 19, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 20, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 21, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 22, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 23, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 24, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 25, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 26, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 27, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 28, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 29, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 6, 30, "日", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2024年7月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 7).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 7, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 15, "月", Some("海の日"));
    assert_calendar(k.next().unwrap(), 2024, 7, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 23, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 30, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 7, 31, "水", None);
}

#[test]
fn 西暦2024年8月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 8).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 8, 1, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 3, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 4, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 5, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 6, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 11, "日", Some("山の日"));
    assert_calendar(k.next().unwrap(), 2024, 8, 12, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2024, 8, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 19, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 23, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 29, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 30, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 8, 31, "土", None);
}

#[test]
fn 西暦2024年9月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 9).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 9, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 3, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 11, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 16, "月", Some("敬老の日"));
    assert_calendar(k.next().unwrap(), 2024, 9, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 20, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 22, "日", Some("秋分の日"));
    assert_calendar(k.next().unwrap(), 2024, 9, 23, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2024, 9, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 28, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 29, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 9, 30, "月", None);
}

#[test]
fn 西暦2024年10月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 10).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 10, 1, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 2, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 3, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 4, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 5, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 6, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 7, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 8, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 9, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 10, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 11, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 12, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 13, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 14, "月", Some("スポーツの日"));
    assert_calendar(k.next().unwrap(), 2024, 10, 15, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 16, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 17, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 18, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 19, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 20, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 21, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 22, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 23, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 24, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 25, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 26, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 27, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 28, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 29, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 30, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 10, 31, "木", None);
}

#[test]
fn 西暦2024年11月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 11).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 11, 1, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 2, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 3, "日", Some("文化の日"));
    assert_calendar(k.next().unwrap(), 2024, 11, 4, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2024, 11, 5, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 6, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 7, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 8, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 9, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 10, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 11, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 12, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 13, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 14, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 15, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 16, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 17, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 18, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 19, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 20, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 21, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 22, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 23, "土", Some("勤労感謝の日"));
    assert_calendar(k.next().unwrap(), 2024, 11, 24, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 25, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 26, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 27, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 28, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 29, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 11, 30, "土", None);
}

#[test]
fn 西暦2024年12月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2024, 12).unwrap();

    assert_calendar(k.next().unwrap(), 2024, 12, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 3, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 11, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 16, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 20, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 22, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 23, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 28, "土", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 29, "日", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 30, "月", None);
    assert_calendar(k.next().unwrap(), 2024, 12, 31, "火", None);
}
