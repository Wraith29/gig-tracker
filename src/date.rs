use std::fmt::Display;

use crate::error::Error;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Month {
    Unknown,
    Jan = 1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

pub const MONTHS: [Month; 12] = [
    Month::Jan,
    Month::Feb,
    Month::Mar,
    Month::Apr,
    Month::May,
    Month::Jun,
    Month::Jul,
    Month::Aug,
    Month::Sep,
    Month::Oct,
    Month::Nov,
    Month::Dec,
];

impl From<u32> for Month {
    fn from(value: u32) -> Self {
        match value {
            1 => Month::Jan,
            2 => Month::Feb,
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            _ => Month::Unknown,
        }
    }
}

impl From<Month> for u32 {
    fn from(value: Month) -> Self {
        match value {
            Month::Unknown => 0,
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dec => 12,
        }
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Month::Unknown => "Unknown",
                Month::Jan => "Jan",
                Month::Feb => "Feb",
                Month::Mar => "Mar",
                Month::Apr => "Apr",
                Month::May => "May",
                Month::Jun => "Jun",
                Month::Jul => "Jul",
                Month::Aug => "Aug",
                Month::Sep => "Sep",
                Month::Oct => "Oct",
                Month::Nov => "Nov",
                Month::Dec => "Dec",
            }
        )
    }
}

#[derive(Clone)]
pub struct Date {
    pub date: u32,
    pub month: Month,
    pub year: u32,
}

impl Date {
    pub fn is_valid(raw_date: &str) -> bool {
        let components: Vec<Result<u32, Error>> = raw_date
            .split("/")
            .map(|s| match s.parse::<u32>() {
                Ok(val) => Ok(val),
                Err(err) => Err(Error::Str(err.to_string())),
            })
            .collect();

        if components.len() != 3 || components.iter().any(|c| c.is_err()) {
            return false;
        }

        true
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}/{:02}/{:02}",
            self.year,
            u32::from(self.month.clone()),
            self.date
        )
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        let components: Vec<u32> = value.split("/").map(|s| s.parse().unwrap()).collect();

        assert!(components.len() == 3);

        Self {
            year: components[0],
            month: Month::from(components[1]),
            date: components[2],
        }
    }
}
