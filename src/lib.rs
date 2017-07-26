#[deny(non_camel_case_types,
       non_snake_case,
       unused_import_braces,
       trivial_numeric_casts,
       unstable_features,
       unused_allocation,
       unused_imports,
       unused_must_use,
       unused_mut,
       unused_qualifications,
       while_true,
       unsafe_code)]

extern crate select;
extern crate reqwest;
extern crate regex;

pub mod stock;
pub mod dax;
pub mod mdax;
pub mod stoxx50e;
