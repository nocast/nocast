
# NoCast
> Where light does not reach, darkness reigns.

## What is this?
NoCast is an alternative to the *AWESOME* [Raycast](https://www.raycast.com/) available only on MacOS.
It's written in Rust, with a plugin system supporting precompiled rust plugins for the power of the language.

## Installation
Currently, the only way to install this is building it from source. To do it you need [cargo](https://doc.rust-lang.org/cargo/),
which you can install with [the normal Rust installation](https://rustup.rs/).

With rust installed, run the following commands:
- `git clone https://github.com/roger-padrell/nocast`
- `cd nocast`
- `cargo install --path .`
- `nocast setup`

## Creating plugins
See [the dev guide](DEV.md)
