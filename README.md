# koyomi-rs

[![CI](https://github.com/panther-king/koyomi-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/panther-king/koyomi-rs/actions/workflows/ci.yml/badge.svg) [![version](https://img.shields.io/crates/v/koyomi-rs.svg)](https://crates.io/crates/koyomi-rs) [![downloads](https://img.shields.io/crates/d/koyomi-rs.svg)](https://crates.io/crates/koyomi-rs) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/panther-king/koyomi-rs/blob/master/LICENSE)

`koyomi-rs` is a calendar utility for Japan that depends [chrono](https://crates.io/crates/chrono) crate.

# About

This can determine Japanese holidays and no configuration file or database maintenance is required.

In addition, you can derive and determine various Japanese-specific definitions related to dates (year, month, and day).

# Installation

Add `koyomi-rs` as a dependency in your `Cargo.toml`.

``` shell
cargo add koyomi-rs
```

``` toml
[dependencies]
koyomi-rs = "0.1"
```

# Usage

## Japanese holidays

Using a struct that implements `Datelike` from chrono, you can determine whether a specific date corresponds to a Japanese holiday.

To use it, do the following.

``` rust
use chrono::NaiveDate;
use koyomi_rs::JapaneseHoliday;

let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let holiday = JapaneseHoliday::holiday(&date); // Some(JapaneseHoliday::NewYearsDay);

// You can also determine whether it's a specific holiday.
let holiday = JapaneseHoliday::marine_day(&date); // None
```

## Japanese era

Using a struct that implements `Datelike` from chrono, you can determine Japanese era.
Please note that only the eras up to the `明治` are supported for past era names.

To use it, do the following.

``` rust
use chrono::NaiveDate;
use koyomi_rs::JapaneseEra;

let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let era = JapaneseEra::from_datelike(&date); // Some(JapaneseEra::Reiwa(6))
```

## Japanese month

Using a struct that implements `Datelike` from chrono, you can determine Japanese era.

To use it, do the following.

``` rust
use chrono::NaiveDate;
use koyomi_rs::JapaneseMonth;

let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let month = JapaneseMonth::from_datelike(&date); // JapaneseMonth::Mutsuki;
```

## Japanese weekday

Using a struct that implements `Datelike` from chrono, you can determine Japanese weekday.

To use it, do the following.

``` rust
use chrono::NaiveDate;
use koyomi_rs::JapaneseWeekday;

let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let zodiac = JapaneseWeekday::from_datelike(&date); // JapaneseWeekday::Getsu;
```

## Japanese zodiac, etc.

Using a struct that implements `Datelike` from chrono, you can determine Japanese zodiac.

To use it, do the following.

``` rust
use chrono::NaiveDate;
use koyomi_rs::JapaneseZodiac;

let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let zodiac = JapaneseZodiac::from_datelike(&date); // JapaneseZodiac::Tatsu;
```

Please refer to the documentation for Japanese-specific definitions related to years other than the zodiac signs.

## Japanese calendar

You can also generate a calendar that includes all of the above.

To use it as a Japanese calendar, do the following.

``` rust
use koyomi_rs::Koyomi;

// Koyomi implements Iterator trait.
let mut calendar = Koyomi::year_of(2024);
let date = calendar.next().unwrap();

date.sexagenary_cycle_name(); // 甲辰
date.heavenly_stem_name();    // 甲
date.zodiac_name();           // 辰
date.western_year();          // 2024
date.era_name();              // Some("令和")

date.month_number();          // 1
date.month_name();            // 睦月

date.day();                   // 1
date.weekday_name();          // 月
date.holiday_name();          // Some("元日")
```

# Note

`koyomi-rs` only handles dates (year, month, and day) and cannot handle hours, minutes, or seconds.

# License

[MIT](LICENSE)
