# rawinput-rust
Rawinput API for Rust on Windows. Allows for the detection of individual mice and keyboards.

## Examples:
### Individual Mouse Click Detection (Requires at least two mice!):
cargo run --example apitest

## Current Status:
VERY EARLY Proof of Concept Stage. Mouse click support only.

### Short Term Goals: 
* Full Mouse and Keyboard Support
* Run in a separate thread to prevent interference with other libraries' message loops.

### Medium Term Goals:
* Add HID Support

### (Very) Long Term Goals:
* Cross Platform Support

## Thanks

Dylan Ede for kicking off this idea and tons of help besides

The winapi-rs contributors for making ffi in rust more bearable
