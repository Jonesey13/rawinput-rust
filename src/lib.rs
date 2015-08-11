extern crate libc;
extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate regex;

use winapi::*;
use kernel32::*;
use user32::*;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::collections::HashMap;

#[test]
fn it_works() {
}

#[repr(C)] #[derive(Debug)]
pub struct RAWINPUTHID {
    pub header: RAWINPUTHEADER,
    pub data: RAWHID,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWINPUTMOUSE {
    pub header: RAWINPUTHEADER,
    pub data: RAWMOUSE,
}


#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWINPUTKEYBOARD {
    pub header: RAWINPUTHEADER,
    pub data: RAWKEYBOARD,
}


pub enum RAWINPUTTYPE{
    MOUSE(*mut RAWINPUTMOUSE),
    KEYBOARD(*mut RAWINPUTKEYBOARD),
    HID(*mut RAWINPUTHID),
}

pub unsafe fn derive_rawinput_type(input: *mut RAWINPUT) -> RAWINPUTTYPE{
    use RAWINPUTTYPE::*;
    let input_type = (*input).header.dwType;
    match input_type{
        RIM_TYPEMOUSE => MOUSE(input as *mut RAWINPUTMOUSE),
        RIM_TYPEKEYBOARD => KEYBOARD(input as *mut RAWINPUTKEYBOARD),
        RIM_TYPEHID => HID(input as *mut RAWINPUTHID),
        _ => panic!("Should be Unreachable!"),
    }
}

pub enum ButtonState{
    Pressed,
    Released,
}

pub enum MouseEvent{
    XPosChange(u64),
    YPosChange(u64),
    LeftButton(ButtonState),
    RightButton(ButtonState),
    MiddleButton(ButtonState),
}

#[derive(Clone)]
pub struct Mouse{
    name: String,
}

#[derive(Clone)]
pub struct Keyboard{
    name: String,
}

#[derive(Clone)]
pub struct Hid{
    name: String,
}

pub struct Devices{
    mice: Vec<Mouse>,
    keyboards: Vec<Keyboard>,
    hids: Vec<Hid>,
    device_map: HashMap<HANDLE,(DeviceType, usize)>,
}

pub enum DeviceType{
    Mouse,
    Keyboard,
    Hid,
}

impl Devices{
    pub fn new() -> Devices{
        Devices{ mice: Vec::new(),
                 keyboards: Vec::new(),
                 hids: Vec::new(),
                 device_map: HashMap::new(),
        }
    }
}


pub fn produce_raw_device_list() -> Devices{
    let mut device_list = Devices::new();
    unsafe{
        let mut buffer: [u8; 10000] = mem::uninitialized();
        let mut num_devices: UINT = 0;
        let buffer_size = 1024 as UINT;
        let device_list_size =  mem::size_of::<RAWINPUTDEVICELIST>();
        let mut result = GetRawInputDeviceList(ptr::null_mut(),
                                               &mut num_devices,
                                               device_list_size as UINT);
        if result == -1i32 as UINT{
            panic!("Failed to Get Raw Device List!");
        }
        result = GetRawInputDeviceList(buffer.as_mut_ptr() as *mut RAWINPUTDEVICELIST,
                                       &mut num_devices,
                                       mem::size_of::<RAWINPUTDEVICELIST>() as UINT);
        if result == -1i32 as UINT{
            panic!("Failed to Get Raw Device List Again!");
        }

        for pos in 0..result as usize{
            let device_ptr = (&mut buffer[pos * device_list_size..
                                          (pos+1) * device_list_size]).as_mut_ptr() as *const RAWINPUTDEVICELIST;
            let device = *device_ptr;
            let device_handle = device.hDevice;
            let device_type = device.dwType;
            let mut name_buffer: [u16; 1024] = mem::uninitialized();
            let mut name_buffer_size: UINT = 1024;
            let result_2 = GetRawInputDeviceInfoW(device_handle,
                                                  RIDI_DEVICENAME,
                                                  name_buffer.as_mut_ptr() as LPVOID,
                                                  &mut name_buffer_size);
            if result_2 == -1i32 as UINT{
                panic!("GetRawInputDeviceInfo Failed: Required Size: {:?}", name_buffer_size);
            }
            let name_slice = &name_buffer[0..result_2 as usize];
            let full_name = match OsString::from_wide(name_slice).into_string(){
                Ok(something) => something,
                Err(_) => panic!("String Conversion Failed"),
            };

            let name = String::from(full_name);

            match device_type{
                RIM_TYPEMOUSE => {
                    if let Some(pos) = device_list.mice.iter().cloned().enumerate().find(|m| m.1.name == name) {
                        device_list.device_map.insert(device_handle, (DeviceType::Mouse, pos.0));
                    }
                    else{
                        device_list.mice.push(Mouse{name: name});
                        device_list.device_map.insert(device_handle, (DeviceType::Mouse, device_list.mice.len()));
                    }
                },
                RIM_TYPEKEYBOARD => {
                    if let Some(pos) = device_list.keyboards.iter().cloned().enumerate().find(|m| m.1.name == name) {
                        device_list.device_map.insert(device_handle, (DeviceType::Keyboard, pos.0));
                    }
                    else{
                        device_list.keyboards.push(Keyboard{name: name});
                        device_list.device_map.insert(device_handle, (DeviceType::Keyboard, device_list.keyboards.len()));
                    }
                },
                RIM_TYPEHID => {
                    if let Some(pos) = device_list.hids.iter().cloned().enumerate().find(|m| m.1.name == name) {
                        device_list.device_map.insert(device_handle, (DeviceType::Hid, pos.0));
                    }
                    else{
                        device_list.hids.push(Hid{name: name});
                        device_list.device_map.insert(device_handle, (DeviceType::Hid, device_list.hids.len()));
                    }
                },
                _ => (),
            }
        }
    }
    device_list
}

pub fn print_raw_device_list(device_list: Devices){
    println!("Mice:");
    for mouse in device_list.mice{
        println!("{}", mouse.name);
    }
    println!("Keyboards:");
    for keyboard in device_list.keyboards{
        println!("{}", keyboard.name);
    }
    println!("Hids:");
    for hid in device_list.hids{
        println!("{}", hid.name);
    }
}
