#[warn(missing_docs)]

extern crate sysfs_pwm;

use ::errors::*;
use dispenser::Dispenser;
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
pub struct AdafruitPeristalticDispenser {
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
    // Target flow rate to dispense at in mL/s
    target_flow_rate_mL_s: f64,
}

impl AdafruitPeristalticDispenser {
    /// Create a new AdafruitPeristalticDispenser which will have pwm control over the pwmchip at
    /// number. This function does not export he PwmChip
    ///
    /// Arguments:
    /// * chip: u32 - The `pwmchip` number to provide to AdafruitPeristalticDispenser
    /// * number: u32 - The `pwmchip` number to provide to AdafruitPeristalticDispenser
    fn new(chip: u32, number: u32) -> Result<AdafruitPeristalticDispenser> {
        let pwm: Pwm = try!(Pwm::new(chip, number));
        Ok(AdafruitPeristalticDispenser {
            pwm: pwm,
            motor_voltage: 12.0, // Volts
            max_flow_rate_mL_s: 1.666666, // mL/s
            target_flow_rate_mL_s: 1.66666, // mL/s
            remaining_mL: 0.0, // mL
            period_ns: 20_000, // Period to set the pwm channel to.
        })
    }
}

impl Dispenser for AdafruitPeristalticDispenser {
    fn remaining(&self) -> Result<f64> {
        Ok(self.remaining_mL)
    }

    fn dispense(&mut self, quantity_mL: f64) -> Result<f64> {
        if quantity_mL > self.remaining_mL {
            Err(ErrorKind::NotEnoughLiquid(self.remaining_mL).into())
        } else {
            Ok(quantity_mL)
        }
    }

    fn set_level(&mut self, quantity_mL: f64) -> Result<()> {
        self.remaining_mL = quantity_mL;
        Ok(())
    }

    fn max_flow_rate(&self) -> f64 {
        self.max_flow_rate_mL_s
    }

    fn set_flow_rate(&mut self, rate: f64) -> Result<()> {
        self.target_flow_rate_mL_s = rate;
        Ok(())
    }
}
