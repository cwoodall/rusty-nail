extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

// PIN: EHRPWM0A (P8_13)
const BB_PWM_CHIP: u32 = 6;
const BB_PWM_NUMBER: u32 = 1;

/// Make an LED "breathe" by increasing and
/// decreasing the brightness
fn main() {
    let pwm = Pwm::new(BB_PWM_CHIP, BB_PWM_NUMBER).unwrap(); // number depends on chip, etc.
    pwm.with_exported(|| {
        pwm.enable(true).unwrap();
        pwm.set_period_ns(20_000).unwrap();
        pwm.set_duty_cycle_ns(1000)
    }).unwrap();
}
