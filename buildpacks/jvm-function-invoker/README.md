# Heroku Cloud Native JVM Function Invoker Buildpack

## Requirements
* [Rust](https://www.rust-lang.org/tools/install) via `rustup`.
* `x86_64-unknown-linux-musl` Rust target: `rustup target add x86_64-unknown-linux-musl`
* [cargo-make](https://crates.io/crates/cargo-make): `cargo install cargo-make`
* [musl libc](https://www.musl-libc.org/)
  * Linux (Debian): `sudo apt install musl-tools`
  * MacOS via [homebrew-musl-cross](https://github.com/FiloSottile/homebrew-musl-cross): `brew install FiloSottile/musl-cross/musl-cross`
* libssl-dev
  * Linux (Debian): `sudo apt install libssl-dev`
  * MacOS: `brew install openssl`
* [pack](https://buildpacks.io/docs/tools/pack/) (for local development)

## Usage
This buildpack targets `x86_64-unknown-linux-musl` as the platform for the buildpack. It uses [`libcnb.rs`](https://github.com/Malax/libcnb.rs) as the language binding for buildpacks which cames with tooling for cross-compilation and packagaing.

### Development
Use [`libcnb-cargo`](https://github.com/Malax/libcnb.rs/tree/main/libcnb-cargo) to cross-compile and build the buildpack for local development and testing:

```shell
$ cargo libcnb package
```

### Production
A production release can also be build with `libcnb-cargo` by passing in the `--release` flag:

```shell
$ cargo libcnb package --release
```

### Testing
To run the unit + doc tests:
```shell
$ cargo test
```

## License
See [LICENSE](../../LICENSE) file.
