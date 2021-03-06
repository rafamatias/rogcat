// Copyright © 2016 Felix Obenhuber
// This program is free software. It comes without any warranty, to the extent
// permitted by applicable law. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License, Version 2, as
// published by Sam Hocevar. See the COPYING file for more details.

use csv::WriterBuilder;
use failure::Error;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use time::{strftime, strptime, Tm};

type StdResult<T, E> = ::std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum Format {
    Csv,
    Html,
    Human,
    Json,
    Raw,
}

impl FromStr for Format {
    type Err = &'static str;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s {
            "csv" => Ok(Format::Csv),
            "html" => Ok(Format::Html),
            "human" => Ok(Format::Human),
            "json" => Ok(Format::Json),
            "raw" => Ok(Format::Raw),
            _ => Err("Format parsing error"),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Format::Csv => "csv",
                Format::Html => "html",
                Format::Human => "human",
                Format::Json => "json",
                Format::Raw => "raw",
            }
        )
    }
}

const LEVEL_VALUES: &[&str] = &[
    "trace", "debug", "info", "warn", "error", "fatal", "assert", "T", "D", "I", "W", "E", "F",
    "A",
];

#[derive(Clone, Debug, Deserialize, PartialOrd, PartialEq, Serialize)]
pub enum Level {
    None,
    Trace,
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Assert,
}

impl Display for Level {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Level::None => "-",
                Level::Trace => "T",
                Level::Verbose => "V",
                Level::Debug => "D",
                Level::Info => "I",
                Level::Warn => "W",
                Level::Error => "E",
                Level::Fatal => "F",
                Level::Assert => "A",
            }
        )
    }
}

impl Default for Level {
    fn default() -> Level {
        Level::None
    }
}

impl<'a> From<&'a str> for Level {
    fn from(s: &str) -> Self {
        match s {
            "T" | "trace" => Level::Trace,
            "V" | "verbose" => Level::Verbose,
            "D" | "debug" => Level::Debug,
            "I" | "info" => Level::Info,
            "W" | "warn" => Level::Warn,
            "E" | "error" => Level::Error,
            "F" | "fatal" => Level::Fatal,
            "A" | "assert" => Level::Assert,
            _ => Level::None,
        }
    }
}

impl Level {
    pub fn values() -> &'static [&'static str] {
        LEVEL_VALUES
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Timestamp {
    pub tm: Tm,
}

impl Deref for Timestamp {
    type Target = Tm;

    fn deref(&self) -> &Tm {
        &self.tm
    }
}

impl Timestamp {
    pub fn new(t: Tm) -> Timestamp {
        Timestamp { tm: t }
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        strftime("%m-%d %H:%M:%S.%f", &self.tm)
            .map_err(|e| ::serde::ser::Error::custom(e.to_string()))?
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeVisitor;

        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = Timestamp;
            fn visit_str<E>(self, str_data: &str) -> StdResult<Timestamp, E>
            where
                E: ::serde::de::Error,
            {
                strptime(str_data, "%m-%d %H:%M:%S.%f")
                    .map(Timestamp::new)
                    .map_err(|_| {
                        ::serde::de::Error::invalid_value(
                            ::serde::de::Unexpected::Str(str_data),
                            &self,
                        )
                    })
            }

            fn expecting(&self, formatter: &mut Formatter) -> ::std::fmt::Result {
                formatter.write_str("string %m-%d %H:%M:%S.%f")
            }
        }

        deserializer.deserialize_str(TimeVisitor)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Record {
    pub timestamp: Option<Timestamp>,
    pub message: String,
    pub level: Level,
    pub tag: String,
    pub process: String,
    pub thread: String,
    pub raw: String,
}

impl Record {
    pub fn format(&self, format: &Format) -> Result<String, Error> {
        match *format {
            Format::Csv => {
                let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
                wtr.serialize(self)?;
                wtr.flush()?;
                Ok(String::from_utf8(wtr.into_inner().unwrap())?
                    .trim_right_matches('\n')
                    .to_owned())
            }
            Format::Html => unimplemented!(),
            Format::Human => unimplemented!(),
            Format::Json => ::serde_json::to_string(self)
                .map_err(|e| format_err!("Json serialization error: {}", e)),
            Format::Raw => Ok(self.raw.clone()),
        }
    }
}
