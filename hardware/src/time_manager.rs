#![allow(unused)]

use std::string;

use time::format_description::well_known::Iso8601;
use time::{Date, PrimitiveDateTime, Time};

use time::macros::date;
use time::macros::time;
use time::{OffsetDateTime, UtcOffset};

pub struct TimeManager {
    current_time: PrimitiveDateTime,
    sunrise_time: PrimitiveDateTime,
    sunset_time: PrimitiveDateTime,
}

pub enum SetTime {
    CurrentTime,
    SunriseTime,
    SunsetTime,
}

impl SetTime {
    pub fn get_type(value: &str) -> SetTime {
        if value.to_uppercase().contains("SUNRISE") {
            return SetTime::SunriseTime;
        }
        if value.to_uppercase().contains("SUNSET") {
            return SetTime::SunsetTime;
        }
        return SetTime::CurrentTime;
    }
}

impl TimeManager {
    pub fn new() -> Self {
        let mut current_time = PrimitiveDateTime::new(date!(2023 - 01 - 01), time!(0:00));
        let mut sunrise_time = PrimitiveDateTime::new(date!(2023 - 01 - 01), time!(0:00));
        let mut sunset_time = PrimitiveDateTime::new(date!(2023 - 01 - 01), time!(0:00));

        TimeManager {
            current_time: current_time,
            sunrise_time: sunrise_time,
            sunset_time: sunset_time,
        }
    }

    pub fn set_time(&mut self, value: &str, time_target: SetTime) {
        match time_target {
            SetTime::CurrentTime => {
                let time = PrimitiveDateTime::parse(value, &Iso8601::DEFAULT).unwrap();
                self.current_time = time;
                println!("The current time is {}", self.current_time);
            }
            SetTime::SunriseTime => {
                let time = PrimitiveDateTime::parse(value, &Iso8601::DEFAULT).unwrap();
                self.sunrise_time = time;
                println!("The current time is {}", self.sunrise_time);
            }
            SetTime::SunsetTime => {
                let time = PrimitiveDateTime::parse(value, &Iso8601::DEFAULT).unwrap();
                self.sunset_time = time;
                println!("The current time is {}", self.sunset_time);
            }
        }
    }
}
