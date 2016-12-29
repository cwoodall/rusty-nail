#[warn(missing_docs)]

extern crate sysfs_pwm;

use ::errors::RustyNailError;
use ::dispenser::Dispenser;
use sysfs_pwm::Pwm;

/// An implementation of a Dispenser which works with the
/// [Adafruit 12V DC Peristaltic Pumps](https://www.adafruit.com/product/1150). These pumps have
/// the following parameters:
///
/// - Working Temperature: 0℃ - 40 ℃
/// - Motor voltage: 12VDC
/// - Motor current: 200-300mA
/// - Flow rate: up to 100 mL/min (1.6666... mL/second)
///
/// This will allow us to create a controller for the dispenser which should output ~100mL/min
/// pretty reliably and scale that flow rate.
struct AdafruitPeristalticDispenser {
    /// The pwm channel to use.
    pwm: Pwm,
    /// Period of the PWM Channel
    period_ns: u32,
    /// Motor voltage (used in calculations)
    motor_voltage: f64,
    /// Maximum flow rate allowed (used in calculations)
    max_flow_rate_mL_s: f64,
    /// Remaining liquid in mL
    remaining_mL: f64,
}

impl AdafruitPeristalticDispenser {
    /// Create a new AdafruitPeristalticDispenser which will have pwm control over the pwmchip at
    /// number. This function does not export he PwmChip
    ///
    /// Arguments:
    /// * chip: u32 - The `pwmchip` number to provide to AdafruitPeristalticDispenser
    /// * number: u32 - The `pwmchip` number to provide to AdafruitPeristalticDispenser
    fn new(chip: u32, number: u32) -> RustyNailResult<AdafruitPeristalticDispenser> {
        let pwm: Pwm = try!(Pwm(chip, number));
        Ok(AdafruitPeristalticDispenser {
            pwm: pwm,
            motor_voltage: 12.0, // Volts
            max_flow_rate_mL_s: 1.666666, // mL/s
            remainin_mL: 0, // mL
            period_ns: 20_000, // Period to set the pwm channel to.
        })
    }
}

impl Dispenser for AdafruitPeristalticDispenser {
    fn remaining() -> RustyNailResult<f64> {
        Ok(remaining_mL)
    }

    fn dispense(quantity_mL: f64) -> RustyNailResult<f64> {
        if (quantity_mL > remaining_mL) {
            Err(RustyNailError::NotEnoughLiquid)
        } else if (remaining_mL <= 0) {
            Err(RustyNailError::DispenserEmpty)
        } else {

        }
    }
}
