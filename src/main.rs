#![crate_name = "rusty_nail"]
#![warn(missing_docs)]
pub mod pumps;
pub use pumps::Pump;
extern crate sysfs_pwm;

fn main() {
    // let mut b = sysfs_pwm::Pwm::new(6,1).unwrap();
    // let a = pumps::PeristalticPump{pwm: b};
    let a = pumps::TestPump{max_flow_rate: 1.2431};

    println!("{:?}", a.max_flow_rate());
    // println!("{:?}", a);
}
