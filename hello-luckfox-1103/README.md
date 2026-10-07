# Hello, Luckfox 1103!

Dependency-free Rust CLI, cross-compiled and tested on the Luckfox Pico RV1103.
See [board setup and deployment instructions](../docs/bringup.md).

From this directory:

```sh
rustup target add armv7-unknown-linux-musleabihf
cargo build --release --offline
scp target/armv7-unknown-linux-musleabihf/release/hello-luckfox-1103 root@172.32.0.93:/root/hello-luckfox-1103
ssh root@172.32.0.93 '/root/hello-luckfox-1103'
```

Factory password: `luckfox`. `.cargo/config.toml` selects ARMv7 Linux with static
musl linking through `rust-lld`. No vendor cross-compiler is needed for this
pure Rust example. Native C/C++ dependencies may need additional tooling.

```text
Hello, Luckfox 1103!
Rust CLI running on arm Linux
```

Options: `--help`, `--version`. Invalid arguments exit with status 2.
