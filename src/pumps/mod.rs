/// Defines traits for using and interfacing with pumps.
pub trait Pump {
    /// Returns the maximum flow rate of the pump in mL/s
    fn max_flow_rate(&self) -> f64;

    /// Sets the current flow rate of the pump in mL/s
    fn set_flow_rate(&self, rate: f64) -> Result<(), i32>;
}


#[derive(Debug)]
pub struct TestPump {
    pub max_flow_rate: f64,
}

impl Pump for TestPump {
    fn max_flow_rate(&self) -> f64 {
        return self.max_flow_rate
    }

    fn set_flow_rate(&self, rate: f64) -> Result<(), i32> {
        println!("{:?}", rate);

        return Ok(());
    }
}
