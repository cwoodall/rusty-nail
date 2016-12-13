**Table of Contents**

## Basic beaglebone information

```
$ uname -a
Linux beaglebone 4.4.9-ti-r25 #1 SMP Thu May 5 23:08:13 UTC 2016 armv7l GNU/Linux
```

Using debian Jessie

## Setting up wifi
First step was to setup wifi on the Beaglebone Green. First I looked in
`/etc/network/interfaces` which told me to use `connmanctl`. The [arch wiki][https://wiki.archlinux.org/index.php/Connman] had some good instructions which I used:

```
root@beaglebone:~# connmanctl
Error getting VPN connections: The name net.connman.vpn was not provided by any .service filesconnmanctl>connmanctl> scan wifi
Scan completed for wifi
connmanctl> services
*AO tesla                wifi_2cf7f106045e_7465736c61_managed_psk
                         wifi_2cf7f106045e_hidden_managed_psk
    xfinitywifi          wifi_2cf7f106045e_7866696e69747977696669_managed_none
    Amanda and Jon       wifi_2cf7f106045e_416d616e646120616e64204a6f6e_managed_psk
    ARRIS-B762           wifi_2cf7f106045e_41525249532d42373632_managed_psk
    ARRIS-B48A           wifi_2cf7f106045e_41525249532d42343841_managed_psk
    ARRIS-2C02           wifi_2cf7f106045e_41525249532d32433032_managed_psk
    XXFSETUP-A984        wifi_2cf7f106045e_58584653455455502d41393834_managed_psk
    Pignet               wifi_2cf7f106045e_5069676e6574_managed_psk
    franstein            wifi_2cf7f106045e_6672616e737465696e_managed_psk
    Persephone           wifi_2cf7f106045e_506572736570686f6e65_managed_psk
    WENDY                wifi_2cf7f106045e_57454e4459_managed_psk
connmanctl> agent on
Agent registered
connmanctl> connect wifi_2cf7f106045e_7465736c61_managed_psk
Passphrase?
```

## Getting Rust Running on BeagleBone Black

### Attempt number 1: rustup

We will use [rustup][rustup] which is a rust toolchain installer similar
to rvm or virtualenv. On one hand I could do development locally and
then deploy, I would rather have a local toolchain on the beaglebone
as well for tinkering. In the long run I might prefer to push
cross compiled binaries.

First lets upgrade:

```
$ sudo apt update
$ sudo apt upgrade
```

```
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

I get an error when building package... Seems to be a gcc-4 vs gcc-6 error... Updated all things  to stretch... May be a terrible idea.
Partially following: https://strongrandom.com/spi-on-a-beaglebone-black-for-led-domination.html

### Attempt number 2: Cross compiling

Install the arm-unknown-linux-gnueabihf target. On Fedora 25 we can find
this in copr:

```
sudo dnf copr enable lantw44/arm-linux-gnueabihf-toolchain
sudo dnf install arm-linux-gnueabihf-gcc
```

Using [rustup][rustup] we can download and install the arm-unknown-linux-gnueabihf target

```
$rustup target add arm-unknown-linux-gnueabihf
```

Next we need to add the followign lines to our `~/.cargo/config` file:

```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```
Now finally you can:

```
$ cargo new --bin test-on-arm
$ cd test-on-arm
$ cargo build --target arm-unknown-linux-gnueabihf
$ scp target/arm-unknown-linux-gnueabihf/debug/test-on-arm root@beaglebone.local:~
Debian GNU/Linux 8

BeagleBoard.org Debian Image 2016-05-27

Support/FAQ: http://elinux.org/Beagleboard:BeagleBoneBlack_Debian

default username:password is [debian:temppwd]

test-on-arm                                                            100% 1578KB   3.4MB/s   00:00
$ ssh root@beaglebone.local ./test-on-arm
Debian GNU/Linux 8

BeagleBoard.org Debian Image 2016-05-27

Support/FAQ: http://elinux.org/Beagleboard:BeagleBoneBlack_Debian

default username:password is [debian:temppwd]

Hello, world!
```

Thanks to:
https://github.com/andygrove/rust-bbb-experiments
https://fedorahosted.org/copr/wiki/HowToEnableRepo

[rustup]: https://github.com/rust-lang-nursery/rustup.rs
