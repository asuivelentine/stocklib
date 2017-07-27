macro_rules! build_list{
    {
        $typename: ident,
        $modname: ident,
        $listname: expr,
        $name: expr,
        $uri: expr
    } => {
        mod $modname {
            use stock::Stock;
            use std::convert::Into;

            use std::fmt::{ Formatter, Debug, Error };

            use reqwest;
            use select::document::Document;
            use select::predicate::{Class, Attr};
            use regex::{RegexBuilder, Regex};

            pub struct $typename { 
                indizes: Vec<Stock>,
                value: f32,
            }

            impl Debug for $typename{
                fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
                    println!("{}: {}", $listname, self.value);
                    for s in &self.indizes{
                        println!("{:?}", s);
                    }
                    Ok(())
                }
            }

            impl Into<f32> for $typename {
                fn into(self) -> f32 {
                    self.value
                }
            }

            impl $typename {

                pub fn new() -> $typename {
                    let html = $typename::download_source();
                    if html.is_err() {
                        return $typename{
                            indizes: Vec::new(),
                            value: -1.234
                        }
                    }

                    let source = html.unwrap();
                    $typename {
                        indizes: $typename::scrape_indizes(&source),
                        value: $typename::scrape_value(&source)
                    }
                }

                fn download_source() -> Result<Document, ()> {
                    reqwest::get($uri)
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

                    let val: String = src.find(Attr("data-item", $name))
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
            }
        }
    }
}

pub mod dax;
pub mod mdax;
pub mod techdax;
pub mod stoxx50e;
