use stock::Stock;

use std::fmt::{ Formatter, Debug, Error };

use reqwest;
use select::document::Document;
use select::predicate::Class;
use regex::{RegexBuilder};

pub struct Dax { dax: Vec<Stock> }

impl Debug for Dax {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        for s in &self.dax {
            println!("{:?}", s);
        }
        Ok(())
    }
}

impl Dax {
    pub fn new() -> Dax {
        Dax {
            dax: Dax::scape()
        }
    }

    fn scape() -> Vec<Stock> {
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
}
