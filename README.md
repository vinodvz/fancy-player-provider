# Channelprovider (in Rust)

### Build Setup for Vizio TV

1. Install [Rust](https://www.rust-lang.org/learn/get-started). --OR-- Open in [GH Codepsaces](https://codespaces.new/BuddyTV/fancy-player-provider?quickstart=1).
1. run `rustup target add armv7-unknown-linux-gnueabihf`
1. unzip and place the toolchain in a directory
    eg: `$HOME/vizio-toolchain`

### Build command

Run the following from this directory:

- `cargo build --target armv7-unknown-linux-gnueabihf --config target.armv7-unknown-linux-gnueabihf.linker=\"$HOME/vizio-toolchain/bin/armv7a-cros-linux-gnueabihf-gcc\"`

### To run

1. Copy file to the fancyplayer directory in the TV

- `scp target/armv7-unknown-linux-gnueabihf/debug/Channelprovider root@192.168.0.17:/vendor/data/app/fancyplayer/`

2. Restart fancyplayer in the TV

- `systemctl restart fancyplayer`

### Build and run test app (in Mac or Ubuntu)

```
    cargo build

    mkdir build
    cd build
    cmake ../testapp
    make
    ./testapp
```
