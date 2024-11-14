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
fn 西暦2025年はきのとみである() {
    let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    assert_eq!("乙巳", SexagenaryCycle::from_datelike(&date).name());
}

#[test]
fn 西暦2025年は巳年である() {
    let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    assert_eq!("巳", JapaneseZodiac::from_datelike(&date).name());
}

#[test]
fn 西暦2025年は令和7年である() {
    let chrono_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let japanese_date = JapaneseDate::from_datelike(&chrono_date);

    assert_eq!(JapaneseEra::Reiwa(7), japanese_date.era().unwrap());
}

#[rustfmt::skip]
#[test]
fn 西暦2025年1月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 1).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 1, 1, "水", Some("元日"));
    assert_calendar(k.next().unwrap(), 2025, 1, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 3, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 4, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 5, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 6, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 13, "月", Some("成人の日"));
    assert_calendar(k.next().unwrap(), 2025, 1, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 20, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 23, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 29, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 30, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 1, 31, "金", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2025年2月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 2).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 2, 1, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 2, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 3, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 4, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 5, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 6, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 7, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 8, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 9, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 10, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 11, "火", Some("建国記念の日"));
    assert_calendar(k.next().unwrap(), 2025, 2, 12, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 13, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 14, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 15, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 16, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 17, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 18, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 19, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 20, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 21, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 22, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 23, "日", Some("天皇誕生日"));
    assert_calendar(k.next().unwrap(), 2025, 2, 24, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2025, 2, 25, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 26, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 27, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 2, 28, "金", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2025年3月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 3).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 3, 1, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 2, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 3, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 4, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 5, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 6, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 7, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 8, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 9, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 10, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 11, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 12, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 13, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 14, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 15, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 16, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 17, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 18, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 19, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 20, "木", Some("春分の日"));
    assert_calendar(k.next().unwrap(), 2025, 3, 21, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 22, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 23, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 24, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 25, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 26, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 27, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 28, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 29, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 30, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 3, 31, "月", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2025年4月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 4).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 4, 1, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 2, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 3, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 4, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 5, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 6, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 7, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 8, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 9, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 10, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 11, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 12, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 13, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 14, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 15, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 16, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 17, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 18, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 19, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 20, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 21, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 22, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 23, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 24, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 25, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 26, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 27, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 28, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 4, 29, "火", Some("昭和の日"));
    assert_calendar(k.next().unwrap(), 2025, 4, 30, "水", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2025年5月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 5).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 5, 1, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 3, "土", Some("憲法記念日"));
    assert_calendar(k.next().unwrap(), 2025, 5, 4, "日", Some("みどりの日"));
    assert_calendar(k.next().unwrap(), 2025, 5, 5, "月", Some("こどもの日"));
    assert_calendar(k.next().unwrap(), 2025, 5, 6, "火", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2025, 5, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 11, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 12, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 19, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 23, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 29, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 30, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 5, 31, "土", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2025年6月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 6).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 6, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 3, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 11, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 16, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 20, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 22, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 23, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 28, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 29, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 6, 30, "月", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2025年7月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 7).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 7, 1, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 2, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 3, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 4, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 5, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 6, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 7, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 8, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 9, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 10, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 11, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 12, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 13, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 14, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 15, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 16, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 17, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 18, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 19, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 20, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 21, "月", Some("海の日"));
    assert_calendar(k.next().unwrap(), 2025, 7, 22, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 23, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 24, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 25, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 26, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 27, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 28, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 29, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 30, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 7, 31, "木", None);
}

#[test]
fn 西暦2025年8月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 8).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 8, 1, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 2, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 3, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 4, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 5, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 6, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 7, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 8, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 9, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 10, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 11, "月", Some("山の日"));
    assert_calendar(k.next().unwrap(), 2025, 8, 12, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 13, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 14, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 15, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 16, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 17, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 18, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 19, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 20, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 21, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 22, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 23, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 24, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 25, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 26, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 27, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 28, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 29, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 30, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 8, 31, "日", None);
}

#[test]
fn 西暦2025年9月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 9).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 9, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 15, "月", Some("敬老の日"));
    assert_calendar(k.next().unwrap(), 2025, 9, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 23, "火", Some("秋分の日"));
    assert_calendar(k.next().unwrap(), 2025, 9, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 9, 30, "火", None);
}

#[test]
fn 西暦2025年10月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 10).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 10, 1, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 3, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 4, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 5, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 6, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 13, "月", Some("スポーツの日"));
    assert_calendar(k.next().unwrap(), 2025, 10, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 20, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 23, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 29, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 30, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 10, 31, "金", None);
}

#[test]
fn 西暦2025年11月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 11).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 11, 1, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 2, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 3, "月", Some("文化の日"));
    assert_calendar(k.next().unwrap(), 2025, 11, 4, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 5, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 6, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 7, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 8, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 9, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 10, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 11, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 12, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 13, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 14, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 15, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 16, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 17, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 18, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 19, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 20, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 21, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 22, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 23, "日", Some("勤労感謝の日"));
    assert_calendar(k.next().unwrap(), 2025, 11, 24, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2025, 11, 25, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 26, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 27, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 28, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 29, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 11, 30, "日", None);
}

#[test]
fn 西暦2025年12月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2025, 12).unwrap();

    assert_calendar(k.next().unwrap(), 2025, 12, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 23, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 30, "火", None);
    assert_calendar(k.next().unwrap(), 2025, 12, 31, "水", None);
}
