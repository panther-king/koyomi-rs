#[rustfmt::skip]
pub(crate) const HEAVENLY_STEMS: [&str; 10] = [
    "甲",
    "乙",
    "丙",
    "丁",
    "戊",
    "己",
    "庚",
    "辛",
    "壬",
    "癸"
];

#[rustfmt::skip]
pub(crate) const JAPANESE_MONTHS: [&str; 12] = [
    "睦月",
    "如月",
    "弥生",
    "卯月",
    "皐月",
    "水無月",
    "文月",
    "葉月",
    "長月",
    "神無月",
    "霜月",
    "師走",
];

#[rustfmt::skip]
pub(crate) const JAPANESE_WEEKDAY: [&str; 7] = [
    "月",
    "火",
    "水",
    "木",
    "金",
    "土",
    "日"
];

#[rustfmt::skip]
pub(crate) const JAPANESE_ZODIAC: [&str; 12] = [
    "子",
    "丑",
    "寅",
    "卯",
    "辰",
    "巳",
    "午",
    "未",
    "申",
    "酉",
    "戌",
    "亥",
];

#[rustfmt::skip]
pub(crate) const SEXAGENARY_CYCLE: [&str; 60] = [
    "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌", "乙亥",
    "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉", "丙戌", "丁亥",
    "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥",
    "庚子", "辛丑", "壬寅", "癸卯", "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥",
    "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥",
];

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct InternalDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}
