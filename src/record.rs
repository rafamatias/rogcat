// Copyright © 2016 Felix Obenhuber
// This program is free software. It comes without any warranty, to the extent
// permitted by applicable law. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License, Version 2, as
// published by Sam Hocevar. See the COPYING file for more details.

#[derive (Clone, PartialOrd, PartialEq)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Assert,
}

impl ::std::fmt::Display for Level {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Level::Trace => "T",
                   Level::Debug => "D",
                   Level::Info => "I",
                   Level::Warn => "W",
                   Level::Error => "E",
                   Level::Fatal => "F",
                   Level::Assert => "A",
               })
    }
}

impl Default for Level {
    fn default() -> Level {
        Level::Debug
    }
}

impl<'a> From<&'a str> for Level {
    fn from(s: &str) -> Self {
        match s {
            "T" => Level::Trace,
            "I" => Level::Info,
            "W" => Level::Warn,
            "E" => Level::Error,
            "F" => Level::Fatal,
            "A" => Level::Assert,
            "D" | _ => Level::Debug,
        }
    }
}

impl ::std::str::FromStr for Level {
    type Err = bool;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

#[derive(Clone)]
pub struct Record {
    pub timestamp: ::time::Tm,
    pub message: String,
    pub level: Level,
    pub tag: String,
    pub process: String,
    pub thread: String,
    pub raw: String,
}

impl Record {
    pub fn new(message: String) -> Record {
        Record {
            timestamp: ::time::now(),
            level: Level::Debug,
            tag: String::default(),
            process: String::default(),
            thread: String::default(),
            message: message,
            raw: String::default(),
        }
    }
}

impl Default for Record {
    fn default() -> Record {
        Record {
            timestamp: ::time::now(),
            level: Level::Debug,
            tag: String::default(),
            process: String::default(),
            thread: String::default(),
            message: String::default(),
            raw: String::default(),
        }
    }
}