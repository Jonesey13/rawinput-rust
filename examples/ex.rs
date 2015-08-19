extern crate rawinput;
use rawinput::*;
use rawinput::RawEvent::*;
fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);
    'outer: loop{
        if let Some(event) = manager.get_event(){
            match event{
                KeyboardEvent(id,  KeyId::Return, State::Pressed) => println!("Keyboard {:?} Return Pressed", id),
                KeyboardEvent(id,  KeyId::Escape, State::Pressed) => break 'outer,
                _ => (),
            }
        }
    }
}
