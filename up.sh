cargo build --target arm-unknown-linux-gnueabihf --release

scp user.db root@192.168.0.110:/home/root/bin
scp cpu.sh root@192.168.0.110:/home/root/bin
scp target/arm-unknown-linux-gnueabihf/release/actix_login root@192.168.0.110:/home/root/bin