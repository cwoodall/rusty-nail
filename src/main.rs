#![crate_name = "rusty_nail"]
#![warn(missing_docs)]
//!

#[macro_use]
extern crate error_chain;
extern crate sysfs_pwm;
mod dispenser;
mod errors;
mod mixer;

use dispenser::AdafruitPeristalticDispenser;

fn main() {}
