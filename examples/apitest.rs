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
                MouseButtonEvent(id, MouseButton::Button4,State::Pressed) => println!("Mouse {:?} Button 4 Down", id),
                MouseButtonEvent(id, MouseButton::Button4,State::Released) => println!("Mouse {:?} Button 4 Up", id),
                MouseButtonEvent(id, MouseButton::Button5,State::Pressed) => println!("Mouse {:?} Button 5 Down", id),
                MouseButtonEvent(id, MouseButton::Button5,State::Released) => println!("Mouse {:?} Button 5 Up", id),
                MouseMoveEvent(id, move_x, move_y) => println!("Mouse {:?}  Moved {:?} {:?}", id, move_x, move_y),
                MouseWheelEvent(id, data) => println!("Mouse {:?} Wheel Data {:?}", id, data),
                KeyboardEvent(id,  KeyId::Escape, State::Pressed) => println!("Keyboard {:?} Escape Pressed", id),
                KeyboardEvent(id,  KeyId::Escape, State::Released) => println!("Keyboard {:?} Escape Released", id),
                KeyboardEvent(id,  KeyId::Return, State::Pressed) => println!("Keyboard {:?} Return Pressed", id),
                KeyboardEvent(id,  KeyId::Return, State::Released) => println!("Keyboard {:?} Return Released", id),
                _  => (),
            }
        }
        current_time = time::precise_time_s() - start_time;
    }
}
