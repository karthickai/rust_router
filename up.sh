cargo build --target arm-unknown-linux-gnueabihf --release

scp user.db root@192.168.1.39:/home/root/bin
scp cpu.sh root@192.168.1.39:/home/root/bin
scp .env root@192.168.1.39:/home/root/bin
scp target/arm-unknown-linux-gnueabihf/release/router_web root@192.168.1.39:/home/root/bin