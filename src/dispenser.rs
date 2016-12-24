use errors::{RustyNailResult, RustyNailError};

/// Defines trait for interfacing to a dispense
pub trait Dispenser {
    /// Returns the maximum flow rate of the pump in mL/s
    fn max_flow_rate(&self) -> f64;

    /// Sets the current flow rate of the pump in mL/s
    fn set_flow_rate(&mut self, rate: f64) -> RustyNailResult<()>;

    /// Get the name of beverage being pumped.
    fn get_name(&self) -> RustyNailResult<&String>;

    /// Get the name of beverage being pumped.
    fn set_name(&mut self, str: String);

    /// Set the current liquid level in mL.
    fn set_level(&mut self, level: f64) -> RustyNailResult<()>;

    /// Get the current liquid level in mL.
    fn get_level(&self) -> RustyNailResult<f64>;

    /// Dispense some quantity of liquid in mL.
    fn dispense(&mut self, quantity: f64) -> RustyNailResult<f64>;
}

// Tests

#[derive(Debug)]
struct TestDispenser {
    name: String,
    pub max_flow_rate: f64,
    pub fluid_level: f64,

}

impl TestDispenser {
    pub fn new(max_flow: f64, fluid_level: f64) -> TestDispenser {
        TestDispenser{ name: "wow".to_string(),
        max_flow_rate: max_flow,
        fluid_level: fluid_level}
    }
}

impl Dispenser for TestDispenser {
    fn max_flow_rate(&self) -> f64 {
        return self.max_flow_rate
    }

    fn set_flow_rate(&mut self, rate: f64) -> RustyNailResult<()> {
        println!("{:?}", rate);

        return Ok(());
    }

    /// Get the name of beverage being pumped.
    fn get_name(&self) -> RustyNailResult<&String> { Ok(&self.name) }

    fn set_name(&mut self, new_name: String) { self.name = new_name }

    /// Set the current liquid level in mL.
    fn set_level(&mut self, level: f64) -> RustyNailResult<()> {
        self.fluid_level = level;
        Ok(())
    }

    /// Get the current liquid level in mL.
    fn get_level(&self) -> RustyNailResult<f64> {
        Ok(self.fluid_level)
    }

    /// Dispense some quantity of liquid in mL.
    fn dispense(&mut self, quantity: f64) -> RustyNailResult<f64> {
        Ok(1.0)
    }
}

#[test]
fn makeTestDispenser() {
    let a = TestDispenser::new(1.1, 2.2);


    println!("{:?}", a.max_flow_rate);
}
