extern crate libc;
extern crate winapi;
extern crate rawlib;
extern crate kernel32;
extern crate user32;
extern crate time;



#[macro_reexport]
use winapi::*;

use kernel32::*;
use user32::*;
use rawlib::*;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::mem;
use std::mem::size_of;


fn main(){
    unsafe{
        let hInstance = GetModuleHandleW(ptr::null());
        if hInstance == ptr::null_mut(){
            println!("Instance Generation Failed");
        }
        let classname =  OsStr::new("Experimental NightMare").encode_wide().chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let wcex = WNDCLASSEXW{
            cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor:  ptr::null_mut(),
            hIcon:  ptr::null_mut(),
            hIconSm:  ptr::null_mut(),
            hInstance: hInstance,
            lpfnWndProc: Some(DefWindowProcW),
            lpszClassName: classname.as_ptr(),
            lpszMenuName: ptr::null_mut(),
            style: 0,
        };
        let a = RegisterClassExW(&wcex);
        if a == 0{
	    panic!("Registering WindowClass Failed!");
        }

        let null: *const c_void  = ptr::null();
        let hwnd: HWND;
        let a: isize = -3;
        let mut HWND_MESSAGE: HWND = std::mem::transmute(a);
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
                               hInstance,
                               ptr::null_mut());
        if hwnd.is_null(){
            panic!("Window Creation Failed!");
        }

        let rid = RAWINPUTDEVICE{
	    usUsagePage: 1,
	    usUsage: 2,	// Joystick
	    dwFlags: RIDEV_EXINPUTSINK,
	    hwndTarget:  hwnd,
        };

	if RegisterRawInputDevices(&rid, 1, mem::size_of::<RAWINPUTDEVICE>() as UINT) ==0 {
	    panic!("Registration of Controller Failed");
        }

        let device_list = produce_raw_device_list();
        print_raw_device_list(device_list);

        let mut exit: BOOL = 1;
        let mut raw_device_size = mem::size_of::<RAWINPUTDEVICELIST>() as UINT;
        let mut num_devices: UINT = 0;
        let mut array: [u8;1024] = mem::uninitialized();
        let mut array2: [u8;1024] = mem::uninitialized();


        let start_time = time::precise_time_s();
        let mut current_time = time::precise_time_s() - start_time;
        while current_time < 10f64{
	    let mut bs: u32 = 0;
            let mut numberofelements: i32 = GetRawInputBuffer(ptr::null_mut(),
                                                              &mut bs,
                                                              mem::size_of::<RAWINPUTHEADER>() as UINT) as INT;
            if numberofelements as INT == -1{
                panic!("{:?}", numberofelements);
            }


            let mut buffer_size: UINT = 1024;


            numberofelements = GetRawInputBuffer(array.as_mut_ptr() as PRAWINPUT,
                                                 &mut buffer_size,
                                                 mem::size_of::<RAWINPUTHEADER>() as UINT) as INT;

            for _ in 0..numberofelements as u32{
                let raw_input_ptr  = derive_rawinput_type(array.as_mut_ptr() as *mut RAWINPUT);
                match raw_input_ptr{
                    RAWINPUTTYPE::MOUSE(pointer) => { let value = *pointer;
                                                      let mut size: UINT  = 0;
                                                      let mut name_buffer: [u16; 1024] = mem::uninitialized();
                                                      let mut name_buffer_size: UINT = 1024;
                                                      let result_2 = GetRawInputDeviceInfoW(value.header.hDevice,
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
                                                      println!("{}", full_name);
                                                      println!("{:?}" ,value.header.hDevice);
                                                      }
                    _ => (),
                }
            }
            current_time = time::precise_time_s() - start_time;
        }


    }
}
