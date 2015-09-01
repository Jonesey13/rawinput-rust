NOTICE - This library is now depreciated and has been superceded by the multiinput library [Here](https://github.com/Jonesey13/multiinput-rust).

# rawinput-rust
Rawinput API for Rust on Windows. Allows for the detection of individual mice and keyboards. Runs in a separate thread to avoid conflict with other input systems. 

[Documentation](http://jonesey13.github.io/rawinput-rust/rawinput/index.html)

## Examples:
### Individual Mouse Detection:
cargo run --example apitest
### Glium Integration Example:
cargo run --example separatemouse

## Current Status:
Early development stage. Basic mouse and keyboard support.

### Short Term Goals: 
* Full Mouse and Keyboard Support - MOSTLY MET (see multiinput)

### Medium Term Goals:
* Add HID Support - MET (See the multiinput library)

### Long Term Goals:
* Cross Platform Support

## Thanks

Dylan Ede for kicking off this idea and tons of help besides

The winapi-rs contributors for making ffi in rust much easier
