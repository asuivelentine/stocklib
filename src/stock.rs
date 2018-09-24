use std::convert::Into;
use std::cmp::{ PartialOrd, Ordering };
use std::ops::Drop;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;

use nix::fcntl::{ flock, FlockArg };

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Stock {
    name: String,
    value: f32,
}

impl Into<String> for Stock{
    fn into(self) -> String {
        self.name.clone()
    }
}

impl Into<f32> for Stock {
    fn into(self) -> f32 {
        self.value
    }
}

impl Drop for Stock {
    fn drop(&mut self) {
        let store = "/home/asui/.config/stockdata/stocks/";
        let path: String = format!("{}{}", store, self.name);
        let datetime: DateTime<Local> = Local::now();

        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path);

        if let Ok(mut f) = f {
            let val: String = format!("{}\n", self.value);
            flock(f.as_raw_fd(), FlockArg::LockExclusive)
                .map(|_| f.write(val.as_bytes()))
                .map(|_| f.write(datetime.to_rfc2822().as_bytes()))
                .map(|_| f.write("\n".as_bytes()))
                .map(|_| drop(f) );
        }
    }
}

impl PartialOrd for Stock {
    fn partial_cmp(&self, other: &Stock) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }

    fn lt(&self, other: &Stock) -> bool {
        self.value < other.value
    }
    fn le(&self, other: &Stock) -> bool {
        self.value <= other.value
    }
    fn gt(&self, other: &Stock) -> bool {
        self.value > other.value
    }
    fn ge(&self, other: &Stock) -> bool {
        self.value >= other.value
    }
}

impl Stock {
    pub fn new<S: Into<String>>(name: S, value: f32) -> Stock {
        Stock {
            name: name.into(),
            value: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stock;

    #[test]
    fn partial() {
        let stockA = Stock::new("ads", 43.43);
        let stockB = Stock::new(String::from("ads"), 43.43);

        assert_eq!(stockA, stockB);
    }

    #[test]
    fn clone() {
        let stockA = Stock::new("ads", 45.43);
        let stockB = stockA.clone();

        assert_eq!(stockA, stockB);
    }

    #[test]
    fn intostr() {
        let stockA = Stock::new("ads", 45.43);
        let name: String = stockA.into();

        assert_eq!(name, "ads".to_string());
    }

    #[test]
    fn intof32() {
        let stockA = Stock::new("ads", 45.43);
        let value: f32 = stockA.into();

        assert_eq!(value, 45.43);
    }

    #[test]
    fn lt() {
        let stockA = Stock::new("a", 123.0);
        let stockB = Stock::new("b", 124.0);

        assert!(stockA < stockB);
        assert!(!(stockB < stockA));
    }

    #[test]
    fn le_ge() {
        let stockA = Stock::new("a", 123.0);
        let stockB = Stock::new("b", 123.0);

        assert!(stockA <= stockB);
        assert!(stockA >= stockB);
    }
}
