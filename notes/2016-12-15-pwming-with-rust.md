**Table of Contents:**
<!-- TOC -->
<!-- /TOC -->

## PWM With SysFS Library

Found a pwm library from [posborne](https://github.com/posborne) which works
with sysfs: https://github.com/posborne/rust-sysfs-pwm

Add the following to Cargo.toml
```
[dependencies]
# or latest version
sysfs-pwm = "^0.1.0"
```

Package does not seem to be found... Might need to build it up on my own...
