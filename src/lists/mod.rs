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
            use std::ops::Drop;
            use std::fs::OpenOptions;
            use std::io::Write;

            use std::fmt::{ Formatter, Debug, Error };

            use ::error::StocklibError;

            use reqwest;
            use select::document::Document;
            use select::predicate::{Class, Attr};
            use regex::{RegexBuilder, Regex};
            use chrono::prelude::*;

			pub type Result<T> = ::std::result::Result<T, StocklibError>;
			pub type FmtResult = ::std::result::Result<(), Error>;

            pub struct $typename { 
                indizes: Vec<Stock>,
                value: f32,
            }

            impl Drop for $typename {
                fn drop(&mut self) {
                    let store = "/home/asui/.config/stockdata/indizes/";
                    let path: String = format!("{}{}", store, $listname);
                    let datetime: DateTime<Local> = Local::now();

                    let f = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(path);

                    if let Ok(mut f) = f {
                        let val: String = format!("{}\n", self.value);
                        let _ = f.write(datetime.to_rfc2822().as_bytes());
                        let _ = f.write("\n".as_bytes());
                        let _ = f.write(val.as_bytes());
                    }
                }
            }

            impl Debug for $typename {
                fn fmt(&self, _: &mut Formatter) -> FmtResult {
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

                pub fn new() -> Result<$typename> {
                    let html = $typename::download_source()?;
					
					let indize = $typename::scrape_indizes(&html)?;
					let list_value = $typename::scrape_value(&html)?;

                    Ok($typename {
                        indizes: indize,
                        value: list_value
                    })
                }

				fn download_source() -> Result<Document> {
					let downloaded_page = reqwest::get($uri)
						.map_err(|_| StocklibError::ParseError)?;
					Document::from_read(downloaded_page)
						.map_err(|_| StocklibError::ParseError)
                }

                fn scrape_indizes(src: &Document) -> Result<Vec<Stock>> {
                    let pat = RegexBuilder::new(r"/aktie/(.*)-Aktie.*\n(.*\n)*?(\d+,\d+)")
                        .multi_line(true)
                        .build()
                        .unwrap();

                    let x: String = src.find(Class("table-hover"))
                        .next()
                        .map(|x| x.html())
						.ok_or_else(|| StocklibError::ParseError)?;

					let mut indizes = Vec::new();
					for v in pat.captures_iter(&x) {
						let name = String::from(&v[1]);
						let value = String::from(&v[3])
							.replace(",", ".")
							.parse()
							.map_err(|_| StocklibError::ParseError)?;

						indizes.push(Stock::new(name, value));
					}
					Ok(indizes)

                }

                fn scrape_value(src: &Document) -> Result<f32> {
                    let pat = Regex::new(r"\d+.\d+,\d").unwrap();

                    let val: String = src.find(Attr("data-item", $name))
                        .skip(1)
                        .next()
                        .map(|x| x.html())
						.ok_or_else(|| StocklibError::ParseError)?;

                    pat.captures(&val)
                        .and_then(|v| {
                            String::from(&v[0])
                                .replace(".", "")
                                .replace(",", ".")
                                .parse()
								.ok()
                        })
						.ok_or_else(|| StocklibError::ParseError)
                }
            }
        }
    }
}

pub mod dax;
pub mod mdax;
pub mod techdax;
pub mod stoxx50e;
