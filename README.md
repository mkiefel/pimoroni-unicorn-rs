# Pimoroni Unicorn

Rust implementation of the pimoroni unicorn devices.

Current support is focused on the cosmic unicorn with embassy.

## Current Features

### Galactic Unicorn

- [x] Display
- [x] Buttons
- [ ] Speaker
- [ ] Extensions

## Unicorn Graphics

Holds a buffer of the led matrix 2d array used by the display. Benefits for using this library include:

- Hold multiple buffers of what can possibly be on the display
- Easily update the actual display buffer without loops
- Run comparisons against what is in the buffer, such as if it is colored or the same color as something else at a given pixel
- Support for the embedded graphics crate

## Examples

Examples show how to make use of the library and the unicorn graphics library.

## Running examples

Install required dependencies.

```sh
rustup target add thumbv6m-none-eabi
cargo install probe-rs --features cli
```

Run the example

For example:

```bash
cargo run --target thumbv6m-none-eabi --example scrolling_text
```
