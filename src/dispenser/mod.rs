//! Testing module level documentation

pub mod AdafruitPeristalticDispenser;

use ::errors::*;

/// Defines trait for interfacing to a dispense
pub trait Dispenser {
    /// Returns the maximum flow rate of the pump in mL/s
    fn max_flow_rate(&self) -> f64;

    /// Sets the current flow rate of the pump in mL/s
    fn set_flow_rate(&mut self, rate: f64) -> Result<()>;

    /// Set the current liquid level in mL.
    fn set_level(&mut self, level: f64) -> Result<()>;

    /// Get the current liquid level in mL.
    fn remaining(&self) -> Result<f64>;

    /// Dispense some quantity of liquid in mL.
    fn dispense(&mut self, quantity: f64) -> Result<f64>;
}

// Tests
#[allow(dead_code)]
#[derive(Debug)]
pub struct TestDispenser {
    pub max_flow_rate: f64,
    pub fluid_level: f64,
}

#[allow(dead_code)]
impl TestDispenser {
    pub fn new(max_flow: f64, fluid_level: f64) -> TestDispenser {
        TestDispenser {
            max_flow_rate: max_flow,
            fluid_level: fluid_level,
        }
    }
}

impl Dispenser for TestDispenser {
    fn max_flow_rate(&self) -> f64 {
        self.max_flow_rate
    }

    fn set_flow_rate(&mut self, rate: f64) -> Result<()> {
        println!("{:?}", rate);

        Ok(())
    }

    /// Set the current liquid level in mL.
    fn set_level(&mut self, level: f64) -> Result<()> {
        self.fluid_level = level;
        Ok(())
    }

    /// Get the current liquid level in mL.
    fn remaining(&self) -> Result<f64> {
        Ok(self.fluid_level)
    }

    /// Dispense some quantity of liquid in mL.
    fn dispense(&mut self, quantity: f64) -> Result<f64> {
        if quantity > self.fluid_level {
            Err(ErrorKind::NotEnoughLiquid(self.fluid_level).into())
        } else {
            self.fluid_level = self.fluid_level - quantity;
            Ok(self.fluid_level)
        }
    }
}

#[test]
fn test_make_dispenser() {
    let a = TestDispenser::new(1.1, 2.2);

    assert!(a.max_flow_rate == 1.1);
}

#[test]
fn test_dispense_decreases() {
    let mut a = TestDispenser::new(1.1, 2.2);

    a.dispense(0.1).unwrap();

    println!("{:?}", a);
    assert!(a.fluid_level == 2.1);

    let b = a.dispense(3.0);

    assert!(b.is_err())
}
