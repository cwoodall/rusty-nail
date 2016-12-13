cargo build --target arm-unknown-linux-gnueabihf
rsync target/arm-unknown-linux-gnueabihf/debug/rusty-nail root@beaglebone.local:~/rusty-nail
ssh root@beaglebone.local ./rusty-nail
ssh root@beaglebone.local pkill -9 rusty-nail
