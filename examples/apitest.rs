extern crate rawlib;
extern crate time;

use rawlib::*;
use rawlib::RawEvent::*;

fn main(){
    //print_raw_device_list(devices.clone());
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);

    let start_time = time::precise_time_s();
    let mut current_time = time::precise_time_s() - start_time;
    while current_time < 10f64{
        while let Some(event) = manager.get_event(){
            match event{
                MouseButtonEvent(id, MouseButton::Left,ButtonState::Pressed) => println!("Mouse {:?} Left Button Down", id),
                MouseButtonEvent(id, MouseButton::Left,ButtonState::Released) => println!("Mouse {:?} Left Button Up", id),
                MouseButtonEvent(id, MouseButton::Right,ButtonState::Pressed) => println!("Mouse {:?} Right Button Down", id),
                MouseButtonEvent(id, MouseButton::Right,ButtonState::Released) => println!("Mouse {:?} Right Button Up", id),
                MouseMoveEvent(id, move_x, move_y) => println!("Mouse {:?}  Moved {:?} {:?}", id, move_x, move_y),
                MouseWheelEvent(id, data) => println!("Mouse {:?} Wheel Data {:?}", id, data),
                _  => (),
            }
        }
        current_time = time::precise_time_s() - start_time;
    }
}
