use stock::Stock;
use std::convert::Into;

use std::fmt::{ Formatter, Debug, Error };

use reqwest;
use select::document::Document;
use select::predicate::Class;
use regex::{RegexBuilder};

pub struct Dax { 
    indizes: Vec<Stock>,
    value: f32,
}

impl Debug for Dax {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        println!("Dax: {}", self.value);
        for s in &self.indizes{
            println!("{:?}", s);
        }
        Ok(())
    }
}

impl Into<f32> for Dax {
    fn into(self) -> f32 {
        self.value
    }
}

impl Dax {
    pub fn new() -> Dax {
        Dax {
            indizes: Dax::scape_indizes(),
            value: 0.0
        }
    }

    fn scape_indizes() -> Vec<Stock> {
        let res = reqwest::get("http://www.boerse-online.de/index/liste/DAX")
            .map_err(|_| ())
            .and_then(|r| Document::from_read(r)
                .map_err(|_| ()));

        if res.is_err() {
            return Vec::new();
        }
        let d = res.unwrap();

        let pat = RegexBuilder::new(r"/aktie/(.*)-Aktie.*\n(.*\n)*?(\d+,\d+)")
            .multi_line(true)
            .build()
            .unwrap();

        let x: String = d.find(Class("table-hover"))
            .next()
            .map(|x| x.html())
            .unwrap_or(String::new());

        pat.captures_iter(&x)
            .map(|v| {
                let name = String::from(&v[1]);
                let value = String::from(&v[3])
                    .replace(",", ".")
                    .parse()
                    .unwrap_or(-1.234);
                Stock::new(name, value)
            })
            .collect()
    }

    pub fn find<S: Into<String>>(&self, name: S) -> Option<&Stock> {
        let n = name.into();

        self.indizes
            .iter()
            .find(|&s| {
                let xn: String = s.clone().into();
                xn.to_uppercase() == n.to_uppercase()
            })
    }
}
