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

## Using Bash and /sys

P8.13 -> /sys/class/pwm/pwmchip6/pwm1

How is this determined?

```
# Enable pwm driver
echo am33xx_pwm > /sys/devices/platform/bone_capemgr/slots
echo cape-universal > /sys/devices/platform/bone_capemgr/slots

# Export chip
echo 1 > /sys/class/pwm/pwmchip6/export

# Setup PWM
echo 20000 > /sys/class/pwm/pwmchip6/pwm1/period
echo 10000 > /sys/class/pwm/pwmchip6/pwm1/duty_cycle
echo 1 > /sys/class/pwm/pwmchip6/pwm1/enable
```

Resources:
- https://groups.google.com/forum/#!category-topic/beagleboard/gpio/1mkf_s_g0vI

-https://github.com/beagleboard/bb.org-overlays/blob/master/examples/BB-BONE-BACONE/example.sh
- https://groups.google.com/forum/#!topic/beagleboard/dkS51WbicTo
- https://briancode.wordpress.com/2015/01/06/working-with-pwm-on-a-beaglebone-black/
- http://stackoverflow.com/questions/29369616/beaglebone-black-pwm-using-c
