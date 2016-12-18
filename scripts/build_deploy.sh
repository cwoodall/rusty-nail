TARGET=arm-unknown-linux-gnueabihf

cargo build --target=$TARGET
rsync target/$TARGET/debug/rusty-nail root@beaglebone.local:~/rusty-nail
ssh root@beaglebone.local ./rusty-nail
ssh root@beaglebone.local pkill -9 rusty-nail
