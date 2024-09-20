# Rust Synthesizer
A simple, cross-platform synthesizer written in Rust for optimal performance.

Still in development.

## Features
* Sine, triangle, square, and saw oscillators.
* Polyphony.
* ADSR envelope.
* Real-time MIDI input.

## Building
**Prerequisites:** Cargo

For more information on how to install Cargo, see [this](https://doc.rust-lang.org/cargo/getting-started/installation.html) page in *The Cargo Book*.

After installation, simply build using Cargo.
```bash
$ cargo build --release
```
Then run the `rust-synthesizer` executable located in `target/release`.

Alternatively, you can run the synthesizer directly by using `cargo run`.
```bash
$ cargo run --release
```
