#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![cfg_attr(feature="nightly", deny(warnings))]

#[macro_use]
extern crate log;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

pub mod parsers;
pub mod utils;
pub mod matcher;
pub mod grammar;

pub use matcher::Matcher;
