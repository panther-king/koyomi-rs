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
fn 西暦2027年はひのとひつじである() {
    let date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
    assert_eq!("丁未", SexagenaryCycle::from_datelike(&date).name());
}

#[test]
fn 西暦2027年は未年である() {
    let date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
    assert_eq!("未", JapaneseZodiac::from_datelike(&date).name());
}

#[test]
fn 西暦2027年は令和9年である() {
    let chrono_date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
    let japanese_date = JapaneseDate::from_datelike(&chrono_date);

    assert_eq!(JapaneseEra::Reiwa(9), japanese_date.era().unwrap());
}

#[rustfmt::skip]
#[test]
fn 西暦2027年1月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 1).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 1, 1, "金", Some("元日"));
    assert_calendar(k.next().unwrap(), 2027, 1, 2, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 3, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 4, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 5, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 6, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 7, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 8, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 9, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 10, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 11, "月", Some("成人の日"));
    assert_calendar(k.next().unwrap(), 2027, 1, 12, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 13, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 14, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 15, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 16, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 17, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 18, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 19, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 20, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 21, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 22, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 23, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 24, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 25, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 26, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 27, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 28, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 29, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 30, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 1, 31, "日", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2027年2月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 2).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 2, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 11, "木", Some("建国記念の日"));
    assert_calendar(k.next().unwrap(), 2027, 2, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 23, "火", Some("天皇誕生日"));
    assert_calendar(k.next().unwrap(), 2027, 2, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 2, 28, "日", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2027年3月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 3).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 3, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 3, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 21, "日", Some("春分の日"));
    assert_calendar(k.next().unwrap(), 2027, 3, 22, "月", Some("振替休日"));
    assert_calendar(k.next().unwrap(), 2027, 3, 23, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 30, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 3, 31, "水", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2027年4月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 4).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 4, 1, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 3, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 4, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 5, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 6, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 11, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 12, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 19, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 23, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 4, 29, "木", Some("昭和の日"));
    assert_calendar(k.next().unwrap(), 2027, 4, 30, "金", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2027年5月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 5).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 5, 1, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 2, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 3, "月", Some("憲法記念日"));
    assert_calendar(k.next().unwrap(), 2027, 5, 4, "火", Some("みどりの日"));
    assert_calendar(k.next().unwrap(), 2027, 5, 5, "水", Some("こどもの日"));
    assert_calendar(k.next().unwrap(), 2027, 5, 6, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 7, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 8, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 9, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 10, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 11, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 12, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 13, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 14, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 15, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 16, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 17, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 18, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 19, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 20, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 21, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 22, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 23, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 24, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 25, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 26, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 27, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 28, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 29, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 30, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 5, 31, "月", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2027年6月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 6).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 6, 1, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 2, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 3, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 4, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 5, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 6, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 7, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 8, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 9, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 10, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 11, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 12, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 13, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 14, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 15, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 16, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 17, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 18, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 19, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 20, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 21, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 22, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 23, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 24, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 25, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 26, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 27, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 28, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 29, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 6, 30, "水", None);
}

#[rustfmt::skip]
#[test]
fn 西暦2027年7月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 7).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 7, 1, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 2, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 3, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 4, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 5, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 6, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 7, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 8, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 9, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 10, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 11, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 12, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 13, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 14, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 15, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 16, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 17, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 18, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 19, "月", Some("海の日"));
    assert_calendar(k.next().unwrap(), 2027, 7, 20, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 21, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 22, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 23, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 24, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 25, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 26, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 27, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 28, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 29, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 30, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 7, 31, "土", None);
}

#[test]
fn 西暦2027年8月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 8).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 8, 1, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 2, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 3, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 4, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 5, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 6, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 7, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 8, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 9, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 10, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 11, "水", Some("山の日"));
    assert_calendar(k.next().unwrap(), 2027, 8, 12, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 13, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 14, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 15, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 16, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 17, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 18, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 19, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 20, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 21, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 22, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 23, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 24, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 25, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 26, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 27, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 28, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 29, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 30, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 8, 31, "火", None);
}

#[test]
fn 西暦2027年9月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 9).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 9, 1, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 3, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 4, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 5, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 6, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 13, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 20, "月", Some("敬老の日"));
    assert_calendar(k.next().unwrap(), 2027, 9, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 23, "木", Some("秋分の日"));
    assert_calendar(k.next().unwrap(), 2027, 9, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 29, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 9, 30, "木", None);
}

#[test]
fn 西暦2027年10月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 10).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 10, 1, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 2, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 3, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 4, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 5, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 6, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 7, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 8, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 9, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 10, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 11, "月", Some("スポーツの日"));
    assert_calendar(k.next().unwrap(), 2027, 10, 12, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 13, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 14, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 15, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 16, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 17, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 18, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 19, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 20, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 21, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 22, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 23, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 24, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 25, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 26, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 27, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 28, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 29, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 30, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 10, 31, "日", None);
}

#[test]
fn 西暦2027年11月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 11).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 11, 1, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 2, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 3, "水", Some("文化の日"));
    assert_calendar(k.next().unwrap(), 2027, 11, 4, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 5, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 6, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 7, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 8, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 9, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 10, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 11, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 12, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 13, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 14, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 15, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 16, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 17, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 18, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 19, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 20, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 21, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 22, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 23, "火", Some("勤労感謝の日"));
    assert_calendar(k.next().unwrap(), 2027, 11, 24, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 25, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 26, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 27, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 28, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 29, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 11, 30, "火", None);
}

#[test]
fn 西暦2027年12月のカレンダーを生成できる() {
    let mut k = Koyomi::month_of(2027, 12).unwrap();

    assert_calendar(k.next().unwrap(), 2027, 12, 1, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 2, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 3, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 4, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 5, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 6, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 7, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 8, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 9, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 10, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 11, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 12, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 13, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 14, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 15, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 16, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 17, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 18, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 19, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 20, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 21, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 22, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 23, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 24, "金", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 25, "土", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 26, "日", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 27, "月", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 28, "火", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 29, "水", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 30, "木", None);
    assert_calendar(k.next().unwrap(), 2027, 12, 31, "金", None);
}
