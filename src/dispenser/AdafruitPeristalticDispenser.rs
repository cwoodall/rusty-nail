//! Testing module level documentation
#[warn(missing_docs)]

extern crate sysfs_pwm;

use ::errors::*;
use dispenser::Dispenser;
use sysfs_pwm::Pwm;
use std::{thread, time};

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
#[derive(Debug)]
pub struct AdafruitPeristalticDispenser {
    /// The pwm channel to use.
    pwm: Pwm,
    /// Period of the PWM Channel
    period_ns: u32,
    /// Motor voltage (used in calculations)
    motor_voltage: f64,
    /// Maximum flow rate allowed (used in calculations)
    max_flow_rate_mL_per_s: f64,
    /// Remaining liquid in mL
    remaining_mL: f64,
    // Target flow rate to dispense at in mL/s
    target_flow_rate_mL_per_s: f64,
}

impl AdafruitPeristalticDispenser {
    /// Create a new AdafruitPeristalticDispenser which will have pwm control over the pwmchip at
    /// number. This function does not export he PwmChip
    ///
    /// Arguments:
    ///
    /// * chip: u32 - The `pwmchip` number to provide to AdafruitPeristalticDispenser
    /// * number: u32 - The `pwmchip` number to provide to AdafruitPeristalticDispenser
    ///
    /// Returns:
    ///
    /// * An error if setting the PWM chip is not found or AdafruitPeristalticDispenser.
    fn new(chip: u32, number: u32) -> Result<AdafruitPeristalticDispenser> {
        let pwm: Pwm = try!(Pwm::new(chip, number));
        Ok(AdafruitPeristalticDispenser {
            pwm: pwm,
            motor_voltage: 12.0, // Volts
            max_flow_rate_mL_per_s: 1.666666, // mL/s
            target_flow_rate_mL_per_s: 1.66666, // mL/s
            remaining_mL: 0.0, // mL
            period_ns: 20_000, // Period to set the pwm channel to.
        })
    }

    /// Calculate the duty_cycle and timeout for the given dispensing quantity, based on the
    /// Dispensers current state.
    ///
    /// Arguments:
    ///
    /// * Borrows a reference to an AdafruitPeristalticDispenser instance
    /// * Quantity in mL of liquid to dispense
    ///
    /// Returns
    ///
    /// * The duty_cycle in nanoseconds.
    /// * The wait time in milliseconds.
    fn calculate_dispense_parameters(&self, quantity_mL: f64) -> Result<(u32, u64)> {
        if (self.target_flow_rate_mL_per_s > self.max_flow_rate_mL_per_s) {
            Err(ErrorKind::TargetFlowRateOutOfRange.into())
        } else {
            let duty_cycle: u32 =
                ((self.target_flow_rate_mL_per_s / self.max_flow_rate_mL_per_s) *
                 (self.period_ns as f64)) as u32;
            let duration_ms: u64 = (1e3 * (quantity_mL / self.target_flow_rate_mL_per_s)) as u64;
            Ok((duty_cycle, duration_ms))
        }
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
            let (duty_cycle, timeout_ms) = try!(self.calculate_dispense_parameters(quantity_mL));

            try!(self.pwm.with_exported(|| {
                try!(self.pwm.enable(true));
                try!(self.pwm.set_period_ns(self.period_ns));
                try!(self.pwm.set_duty_cycle_ns(duty_cycle));
                thread::sleep(time::Duration::from_millis(timeout_ms));
                try!(self.pwm.set_duty_cycle_ns(0));
                self.pwm.enable(false)
                // self.pwm.set_duty_cycle_ns(*self.period_ns)
            }));

            self.remaining_mL = self.remaining_mL - quantity_mL;

            Ok(self.remaining_mL)
        }
    }

    fn set_level(&mut self, quantity_mL: f64) -> Result<()> {
        self.remaining_mL = quantity_mL;
        Ok(())
    }

    fn max_flow_rate(&self) -> f64 {
        self.max_flow_rate_mL_per_s
    }

    fn set_flow_rate(&mut self, rate: f64) -> Result<()> {
        self.target_flow_rate_mL_per_s = rate;
        Ok(())
    }
}
