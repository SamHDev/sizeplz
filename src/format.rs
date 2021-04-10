use crate::calc::Record;
use crate::sort::SortKey;

#[derive(Clone, PartialEq)]
pub enum FormatUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
    Petabytes,
    Auto
}

impl FormatUnit {
    pub fn parse(x: &str) -> Option<FormatUnit> {
        Some(match x.to_ascii_lowercase().trim() {
            "b" => FormatUnit::Bytes,
            "bytes" => FormatUnit::Bytes,

            "kb" => FormatUnit::Kilobytes,
            "k" => FormatUnit::Kilobytes,
            "kilo" => FormatUnit::Kilobytes,
            "kilobytes" => FormatUnit::Kilobytes,

            "mb" => FormatUnit::Megabytes,
            "m" => FormatUnit::Megabytes,
            "mega" => FormatUnit::Megabytes,
            "megabytes" => FormatUnit::Megabytes,

            "gb" => FormatUnit::Gigabytes,
            "g" => FormatUnit::Gigabytes,
            "giga" => FormatUnit::Gigabytes,
            "gigabytes" => FormatUnit::Gigabytes,

            "tb" => FormatUnit::Terabytes,
            "t" => FormatUnit::Terabytes,
            "tera" => FormatUnit::Terabytes,
            "terabytes" => FormatUnit::Terabytes,

            "pb" => FormatUnit::Petabytes,
            "p" => FormatUnit::Petabytes,
            "peta" => FormatUnit::Petabytes,
            "petabytes" => FormatUnit::Petabytes,

            "auto" => FormatUnit::Auto,
            "human" => FormatUnit::Auto,
            "a" => FormatUnit::Auto,
            "h" => FormatUnit::Auto,

            _ => return None
        })
    }

    pub fn suffix(&self) -> &'static str {
        match &self {
            FormatUnit::Bytes => "bytes",
            FormatUnit::Kilobytes => "kb",
            FormatUnit::Megabytes => "mb",
            FormatUnit::Gigabytes => "gb",
            FormatUnit::Terabytes => "tb",
            FormatUnit::Petabytes => "pb",
            FormatUnit::Auto => ""
        }
    }

    pub fn factor(&self) -> f64 {
        match &self {
            FormatUnit::Bytes => 0x1i64 as f64,
            FormatUnit::Kilobytes => 0x400i64 as f64,
            FormatUnit::Megabytes => 0x100000i64 as f64,
            FormatUnit::Gigabytes => 0x40000000i64 as f64,
            FormatUnit::Terabytes => 0x10000000000i64 as f64,
            FormatUnit::Petabytes => 0x4000000000000i64 as f64,
            FormatUnit::Auto => 0x1i64 as f64
        }
    }

    pub fn autosize<'x>(&'x self, value: &u64) -> &'x Self {
        if self == &Self::Auto {
            if value < &0x400u64 {
                &Self::Bytes
            } else if value < &0x100000u64 {
                &Self::Kilobytes
            } else if value < &0x40000000u64 {
                &Self::Megabytes
            } else if value < &0x10000000000u64 {
                &Self::Gigabytes
            } else if value < &0x4000000000000u64 {
                &Self::Terabytes
            } else {
                &Self::Petabytes
            }
        } else {
            self
        }
    }

    pub fn calc(&self, value: u64, round: &RoundFactor) -> f64 {
        round.calc((value as f64) / self.factor())
    }

    pub fn string(&self, value: u64, round: &RoundFactor) -> String {
        round.string((value as f64) / self.factor())
    }

}

#[derive(Clone)]
pub struct RoundFactor(f64, usize);

impl RoundFactor {
    pub fn parse(p: i8) -> Self {
        Self(10f64.powi(p as i32), if p < 0 { 0 } else { p } as usize)
    }

    pub fn calc(&self, value: f64) -> f64 {
        (value * self.0).round() / self.0
    }

    pub fn string(&self, value: f64) -> String {
        format!("{:.1$}", self.calc(value), self.1)
    }
}

pub fn format_record(mut record: Record, indent: usize, last: bool, format: &FormatUnit, round: &RoundFactor, files: &bool, empty: &bool, sort: &SortKey, invert: &bool) {
    let mut ind = String::new();
    if indent != 0 {
        ind.push_str(&"│  ".repeat(indent - 1));
        if last {
            ind.push_str("└  ");
        } else {
            ind.push_str("├  ");
        }
    }

    let form = format.autosize(&record.size);
    println!("{}'{}'  {} {}", ind, record.name, form.string(record.size, round), form.suffix());

    sort.sort(&mut record.children);
    if *invert { record.children.reverse(); }

    let length = record.children.len();
    for (i, child) in record.children.into_iter().enumerate() {
        if child.file && !*files { continue; }
        if child.size == 0 && *empty { continue; }
        format_record(child, indent + 1, i + 1 == length, format, round, files, empty, sort, invert);
    }
}