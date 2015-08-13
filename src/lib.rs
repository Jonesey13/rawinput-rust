extern crate libc;
extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate regex;


use winapi::*;
use user32::*;
use kernel32::*;
use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
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

#[derive(Clone)]
pub enum ButtonState{
    Pressed,
    Released,
}

#[derive(Clone)]
pub enum MouseButton{
    Left,
    Right,
    Middle,
}

#[derive(Clone)]
pub enum MouseDirection{
    X,
    Y,
}

#[derive(Clone)]
pub enum RawEvent{
    MouseButtonEvent(usize,MouseButton,ButtonState),
    MouseMoveEvent(usize,MouseDirection,isize),
    MouseWheelEvent(usize,isize),
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

#[derive(Clone)]
pub struct Devices{
    mice: Vec<Mouse>,
    keyboards: Vec<Keyboard>,
    hids: Vec<Hid>,
    device_map: HashMap<HANDLE, usize>,
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
                        device_list.device_map.insert(device_handle, pos.0);
                    }
                    else{
                        device_list.mice.push(Mouse{name: name});
                        device_list.device_map.insert(device_handle, device_list.mice.len());
                    }
                },
                RIM_TYPEKEYBOARD => {
                    if let Some(pos) = device_list.keyboards.iter().cloned().enumerate().find(|m| m.1.name == name) {
                        device_list.device_map.insert(device_handle, pos.0);
                    }
                    else{
                        device_list.keyboards.push(Keyboard{name: name});
                        device_list.device_map.insert(device_handle, device_list.keyboards.len());
                    }
                },
                RIM_TYPEHID => {
                    if let Some(pos) = device_list.hids.iter().cloned().enumerate().find(|m| m.1.name == name) {
                        device_list.device_map.insert(device_handle, pos.0);
                    }
                    else{
                        device_list.hids.push(Hid{name: name});
                        device_list.device_map.insert(device_handle, device_list.hids.len());
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

pub fn process_mouse_data(raw_data: &RAWMOUSE, id: usize) -> Vec<RawEvent>{
    let buttons = &raw_data.usButtonFlags;
    let mut output: Vec<RawEvent> = Vec::new();
    if *buttons & RI_MOUSE_LEFT_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Left, ButtonState::Pressed ));
        println!("Clicked");
    }
    output
}


pub fn read_input_buffer(devices: &Devices) -> Vec<RawEvent>{
    let mut output: Vec<RawEvent> = Vec::new();
    unsafe{
        let mut array_alloc: [u8;1024] = mem::uninitialized();
        let mut buffer_size: UINT = 0;

        let mut numberofelements: i32 = GetRawInputBuffer(ptr::null_mut(),
                                                          &mut buffer_size,
                                                          mem::size_of::<RAWINPUTHEADER>() as UINT) as INT;
        if numberofelements as INT == -1{
            panic!("GetRawInputBuffer Gave Error on First Call!");
        }
        buffer_size = 1024;
        numberofelements = GetRawInputBuffer(array_alloc.as_mut_ptr() as PRAWINPUT,
                                             &mut buffer_size,
                                             mem::size_of::<RAWINPUTHEADER>() as UINT) as INT;

        if numberofelements as INT == -1{
            panic!("GetRawInputBuffer Gave Error on Second Call!");
        }

        let mut array_ptr = array_alloc.as_mut_ptr();
        for _ in 0..numberofelements as u32{
            let raw_input_ptr  = derive_rawinput_type(array_ptr as *mut RAWINPUT);
            match raw_input_ptr{
                RAWINPUTTYPE::MOUSE(pointer) =>
                { let value = *pointer;
                  let pos = match devices.device_map.get(&value.header.hDevice){
                      Some(item) => *item,
                      None => continue,
                  };
                  output.extend(process_mouse_data(&value.data, pos));
                  array_ptr = array_ptr.offset(1);
                }
                _ => (),
            }
        }
    }
    output
}

#[derive(Clone)]
pub struct RawInputWindow{
    hwnd: HWND,
    keyboard_input: bool,
    mouse_input: bool,
    gamepad_input: bool,
}

pub fn setup_message_loop() -> Result<RawInputWindow, &'static str>{
    let hwnd: HWND;
    unsafe{
        let hinstance = GetModuleHandleW(ptr::null());
        if hinstance == ptr::null_mut(){
            return Err("Instance Generation Failed");
        }
        let classname =  OsStr::new("RawInput Hidden Window").encode_wide().chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let wcex = WNDCLASSEXW{
            cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor:  ptr::null_mut(),
            hIcon:  ptr::null_mut(),
            hIconSm:  ptr::null_mut(),
            hInstance: hinstance,
            lpfnWndProc: Some(DefWindowProcW),
            lpszClassName: classname.as_ptr(),
            lpszMenuName: ptr::null_mut(),
            style: 0,
        };
        let a = RegisterClassExW(&wcex);
        if a == 0{
	    return Err("Registering WindowClass Failed!");
        }

        //let HWND_MESSAGE: HWND = -3isize as HWND;
        hwnd = CreateWindowExW(0,
                               classname.as_ptr(),
                               classname.as_ptr(),
                               0,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               HWND_MESSAGE,
                               ptr::null_mut(),
                               hinstance,
                               ptr::null_mut());
        if hwnd.is_null(){
            return Err("Window Creation Failed!");
        }
    }
    Ok(RawInputWindow{
        hwnd: hwnd,
        mouse_input: false,
        keyboard_input: false,
        gamepad_input: false,
    })
}

impl RawInputWindow{
    pub fn using_mice(&mut self) -> &mut Self{
        self.mouse_input = true;
        self
    }

    pub fn register_devices(&mut self) -> Result<&mut Self,&'static str>{
        let mut rid_vec: Vec<RAWINPUTDEVICE> = Vec::new();

        if self.mouse_input{
            let rid = RAWINPUTDEVICE{
	        usUsagePage: 1,
	        usUsage: 2,	// Mice
	        dwFlags: RIDEV_INPUTSINK,
	        hwndTarget:  self.hwnd,
            };
            rid_vec.push(rid);
        }


        if self.gamepad_input{
            let rid = RAWINPUTDEVICE{
	        usUsagePage: 1,
	        usUsage: 5,	// Gamepads
	        dwFlags: RIDEV_INPUTSINK,
	        hwndTarget:  self.hwnd,
            };
            rid_vec.push(rid);
        }


        if self.keyboard_input{
            let rid = RAWINPUTDEVICE{
	        usUsagePage: 1,
	        usUsage: 6,	// Keyboards
	        dwFlags: RIDEV_INPUTSINK,
	        hwndTarget:  self.hwnd,
            };
            rid_vec.push(rid);
        }

        unsafe{
            if RegisterRawInputDevices(rid_vec.as_mut_ptr(), rid_vec.len() as UINT, mem::size_of::<RAWINPUTDEVICE>() as UINT) ==0 {
	        return Err("Registration of Controller Failed");
            }
        }
        Ok(self)
    }
}
