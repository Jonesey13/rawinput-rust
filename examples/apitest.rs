extern crate rawinput;
extern crate time;

use rawinput::*;
use rawinput::RawEvent::*;

fn main(){
    //print_raw_device_list(devices.clone());
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);

    let start_time = time::precise_time_s();
    let mut current_time = time::precise_time_s() - start_time;
    while current_time < 10f64{
        while let Some(event) = manager.get_event(){
            match event{
                MouseButtonEvent(id, MouseButton::Left,State::Pressed) => println!("Mouse {:?} Left Button Down", id),
                MouseButtonEvent(id, MouseButton::Left,State::Released) => println!("Mouse {:?} Left Button Up", id),
                MouseButtonEvent(id, MouseButton::Right,State::Pressed) => println!("Mouse {:?} Right Button Down", id),
                MouseButtonEvent(id, MouseButton::Right,State::Released) => println!("Mouse {:?} Right Button Up", id),
                MouseMoveEvent(id, move_x, move_y) => println!("Mouse {:?}  Moved {:?} {:?}", id, move_x, move_y),
                MouseWheelEvent(id, data) => println!("Mouse {:?} Wheel Data {:?}", id, data),
                KeyboardEvent(id,  KeyId::Escape, State::Pressed, _) => println!("Keyboard {:?} Escape Pressed", id),
                KeyboardEvent(id,  KeyId::Escape, State::Released, _) => println!("Keyboard {:?} Escape Released", id),
                KeyboardEvent(id,  KeyId::Return, State::Pressed, _) => println!("Keyboard {:?} Return Pressed", id),
                KeyboardEvent(id,  KeyId::Return, State::Released, _) => println!("Keyboard {:?} Return Released", id),
                KeyboardEvent(id,  KeyId::Left, State::Pressed, _) => println!("Keyboard {:?} Left Pressed", id),
                KeyboardEvent(id,  KeyId::Left, State::Released, _) => println!("Keyboard {:?} Left Released", id),
                KeyboardEvent(id,  KeyId::Right, State::Pressed, _) => println!("Keyboard {:?} Right Pressed", id),
                KeyboardEvent(id,  KeyId::Right, State::Released, _) => println!("Keyboard {:?} Right Released", id),
                KeyboardEvent(id,  KeyId::Up, State::Pressed, _) => println!("Keyboard {:?} Up Pressed", id),
                KeyboardEvent(id,  KeyId::Up, State::Released, _) => println!("Keyboard {:?} Up Released", id),
                KeyboardEvent(id,  KeyId::Down, State::Pressed, _) => println!("Keyboard {:?} Down Pressed", id),
                KeyboardEvent(id,  KeyId::Down, State::Released, _) => println!("Keyboard {:?} Down Released", id),
                KeyboardEvent(id,  KeyId::Zero, State::Pressed, _) => println!("Keyboard {:?} Zero Pressed", id),
                KeyboardEvent(id,  KeyId::Zero, State::Released, _) => println!("Keyboard {:?} Zero Released", id),
                _  => (),
            }
        }
        current_time = time::precise_time_s() - start_time;
    }
}
