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
This buildpack targets `x86_64-unknown-linux-musl` as the platform for the buildpack and comes with tooling to support cross-compilation on macOS. It uses [`libcnb.rs`](https://github.com/Malax/libcnb.rs) as the language binding for buildpacks.

### Development
To use this buildpack for local development that can be used by `pack`, a buildpack dir needs to be made:

```
$ cargo make pack
```

This will create a `target/` directory that can be passed to `pack`. If a default builder hasn't been set, the heroku one can be set:

```
$ pack config default-builder heroku/buildpacks:20
```

With the heroku builder image set and from the buildpack directory:

```
$ pack build <IMAGE NAME> -b heroku/jvm -b heroku/maven -b `target` -p <APP SOURCE DIR>
```

### Production
`cargo-make` has the concept of [profiles](https://sagiegurari.github.io/cargo-make/#usage-workspace-profiles) which is how it can choose where to do a "release" build with optimizations for runtime vs. optimizing for build speed.

When packaging up the buildpack the `build.sh` script uses this `cargo-make` command:

```
$ cargo make pack --profile production
```

### Testing
To run the unit + doc tests:
```
$ cargo test
```

## License
See [LICENSE](../../LICENSE) file.
