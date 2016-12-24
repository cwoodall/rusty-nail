#![crate_name = "rusty_nail"]
#![warn(missing_docs)]

mod errors;
pub mod dispenser;
use dispenser::Dispenser;
extern crate sysfs_pwm;

fn main() {
    // let mut b = sysfs_pwm::Pwm::new(6,1).unwrap();
    // let a = pumps::PeristalticPump{pwm: b};
    // println!("{:?}", a);
}
