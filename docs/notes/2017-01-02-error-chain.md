## Moving Errors To Error Chain

After doing some research on different ways of handling errors, and the
chaining of errors from different modules, I found the
[error-chain][error-chain] library. This allows for the use of a
standard way of creating the `errors.rs` file for your module.
The [rust-lang book entry on error handling](https://doc.rust-lang.org/book/error-handling.html) has a
good entry on error handling and why each step is important. I reccomend
going through this tutorial.

However, `error-chain` more or less takes the canonical methodology,
which has a lot of boiler plate, this allows you to create `Error`,
`Result`, and `ErrorKind` types which can be properly `Box`ed and
wrapped using the `From::from` trait so that all `Error` types appear
to be the same trait object. The most compelling feature of
`error-chain` for me is that it can also convert Error types or enums
from other libraries into part of your 'error chain'.

For example:

```rust
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
```

## Simple Dispenser Implementation for Adafruit Peristaltic Pump

Currently I am making an assumption that the peristaltic pump will respond
linearly to the duty cycle with 100% duty cycle corresponding to the max
flow rate and 50% corresponding to 50% of the max flow rate. This is an  over
simplification, but will provide sufficient results for the time being.s

## Starting to Design the `Mixer` Struct

The `Mixers` are essentially a `HashMap` of `String`'s mapping to Dispensers.
Because the Dispensers are a trait object they must be `Box<Dispenser>`s. The
plan is to implement the following:

- `clear`
- `add`
- `remove`
- `get`
- `iter`
- `make_recipe`
- `status`

[error-chain]: https://docs.rs/error-chain/0.7.2/error_chain/
