use stock::Stock;
use std::convert::Into;

use std::fmt::{ Formatter, Debug, Error };

use reqwest;
use select::document::Document;
use select::predicate::{Class, Attr};
use regex::{RegexBuilder, Regex};

pub struct Techdax { 
    indizes: Vec<Stock>,
    value: f32,
}

impl Debug for Techdax {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        println!("Techdax: {}", self.value);
        for s in &self.indizes{
            println!("{:?}", s);
        }
        Ok(())
    }
}

impl Into<f32> for Techdax {
    fn into(self) -> f32 {
        self.value
    }
}

impl Techdax {
    pub fn new() -> Techdax {
        let html = Techdax::download_source();
        if html.is_err() {
            return Techdax {
                indizes: Vec::new(),
                value: -1.234
            }
        }

        let source = html.unwrap();
        Techdax {
            indizes: Techdax::scrape_indizes(&source),
            value: Techdax::scrape_value(&source)
        }
    }

    fn download_source() -> Result<Document, ()> {
        reqwest::get("http://www.boerse-online.de/index/liste/TECDAX")
            .map_err(|_| ())
            .and_then(|r| Document::from_read(r)
                .map_err(|_| ()))
    }

    fn scrape_indizes(src: &Document) -> Vec<Stock> {
        let pat = RegexBuilder::new(r"/aktie/(.*)-Aktie.*\n(.*\n)*?(\d+,\d+)")
            .multi_line(true)
            .build()
            .unwrap();

        let x: String = src.find(Class("table-hover"))
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

    fn scrape_value(src: &Document) -> f32 {
        let pat = Regex::new(r"\d+.\d+,\d").unwrap();

        let val: String = src.find(Attr("data-item", "Y0306000000TECDAX"))
            .skip(1)
            .next()
            .map(|x| x.html())
            .unwrap_or(String::new());

        pat.captures(&val)
            .and_then(|v| {
                String::from(&v[0])
                    .replace(".", "")
                    .replace(",", ".")
                    .parse()
                    .ok()
            })
            .unwrap_or(-1.234)
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
