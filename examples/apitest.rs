extern crate rawlib;
extern crate time;

use rawlib::*;
use rawlib::RawEvent::*;

fn main(){
    //print_raw_device_list(devices.clone());
    let mut window = match setup_message_window(){
        Ok(thing) => thing,
        Err(message) => panic!(message),
    };
    match window.using_mice().register_devices(){
        Ok(thing) => thing,
        Err(message) => panic!(message),
    };


    let start_time = time::precise_time_s();
    let mut current_time = time::precise_time_s() - start_time;
    while current_time < 10f64{
        while let Some(event) = window.get_event(){
            match event{
                MouseButtonEvent(id,MouseButton::Left,ButtonState::Pressed) => println!("Mouse {:?} Left Button Down", id),
                MouseButtonEvent(id,MouseButton::Left,ButtonState::Released) => println!("Mouse {:?} Left Button Up", id),
                MouseButtonEvent(id,MouseButton::Right,ButtonState::Pressed) => println!("Mouse {:?} Right Button Down", id),
                MouseButtonEvent(id,MouseButton::Right,ButtonState::Released) => println!("Mouse {:?} Right Button Up", id),
                _  => (),


            }
        }
        current_time = time::precise_time_s() - start_time;
    }
}
