# rawinput-rust
Rawinput API for Rust on Windows. Allows for the detection of individual mice and keyboards. Runs in a separate thread to avoid conflict with other input systems.

[Documentation](http://jonesey13.github.io/rawinput-rust/rawinput/index.html)

## Examples:
### Individual Mouse Detection:
cargo run --example apitest
### Glium Integration Example:
cargo run --example separatemouse

## Current Status:
EARLY Proof of Concept Stage. Basic mouse support only.

### Short Term Goals: 
* Full Mouse and Keyboard Support

### Medium Term Goals:
* Add HID Support

### (Very) Long Term Goals:
* Cross Platform Support

## Thanks

Dylan Ede for kicking off this idea and tons of help besides

The winapi-rs contributors for making ffi in rust more bearable
