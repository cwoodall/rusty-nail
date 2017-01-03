use sysfs_pwm;

// Define the errors for this crate.
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        PwmError(sysfs_pwm::Error);
    }

    errors {
        NotEnoughLiquid(level: f64) {
            description("There is not enough liquid in the current dispense")
            display("There is currently {} mL left in the dispenser, this is not enough.", level)
        }
    }
}
