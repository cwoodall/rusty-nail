use sysfs_pwm;
use diesel;

// Define the errors for this crate.
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        PwmError(sysfs_pwm::Error) #[doc = "PwmError"];
        DatabaseError(diesel::result::Error);
    }

    errors {
        /// Target flow rate was out of range
        TargetFlowRateOutOfRange {
            description("The target flow rate is out of range")
            display("The target flow rate is out of range.")

        }

        /// Datbase issue
        CouldNotAddIngredient(iname: String, rname: String) {
            description("The ingredient could not be added to recipe")
            display("The ingredient {} could not be added to recipe {}.", iname, rname)
        }

        /// Not enough liquid
        NotEnoughLiquid(level: f64) {
            description("There is not enough liquid in the current dispense")
            display("There is currently {} mL left in the dispenser, this is not enough.", level)
        }
    }
}
