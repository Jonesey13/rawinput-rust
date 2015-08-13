extern crate rawlib;
extern crate time;

use rawlib::*;

fn main(){
    let devices = produce_raw_device_list();
    print_raw_device_list(devices.clone());
    let mut window = match setup_message_loop(){
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
        read_input_buffer(&devices);
        current_time = time::precise_time_s() - start_time;
    }
}
