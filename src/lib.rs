#![crate_name = "rusty_nail"]
#![warn(missing_docs)]
#![feature(custom_derive, plugin)]
#![plugin(diesel_codegen)]
//!

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate sysfs_pwm;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod dispenser;
pub mod errors;
pub mod mixer;
pub mod recipe;

use dispenser::AdafruitPeristalticDispenser;
