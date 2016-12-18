extern crate sysfs_gpio;
extern crate sysfs_pwm;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_pin = Pin::new(68);
    println!("{:?}", my_pin);

    my_pin.with_exported(|| {
        my_pin.set_direction(Direction::Out).unwrap();
        loop {
            my_pin.set_value(0).unwrap();
            sleep(Duration::from_millis(1000));
            my_pin.set_value(1).unwrap();
            sleep(Duration::from_millis(1000));
        }
    }).unwrap();
}
