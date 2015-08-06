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
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::mem::size_of;


unsafe fn garbage_vec(size: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(size);
    v.set_len(size);
    v
}

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
        println!("{:?} {:?}", mem::size_of::<HWND>(), mem::size_of::<INT>());
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

        //ShowWindow(hwnd, SW_SHOWDEFAULT);
        //UpdateWindow(hwnd);



        let mut exit: BOOL = 1;
        let mut array: [u8;1024] = mem::uninitialized();


        while exit == 1{
	    let mut bs: u32 = 0;
            let mut numberofelements: i32 = GetRawInputBuffer(ptr::null_mut(),
                                                              &mut bs,
                                                              mem::size_of::<RAWINPUTHEADER>() as UINT) as INT;
            if numberofelements as INT == -1{
                panic!("{:?}", numberofelements);
            }


            let mut buffer_size: u32 = mem::size_of::<[u8;1024]>() as u32;


            numberofelements = GetRawInputBuffer(array.as_mut_ptr() as PRAWINPUT,
                                                 &mut buffer_size,
                                                 mem::size_of::<RAWINPUTHEADER>() as UINT) as INT;
            if numberofelements != 0{
                println!("/n {:?}", buffer_size);
                println!("{:?}", numberofelements);
                println!("{:?}", mem::size_of::<RAWINPUT>());
                //println!("{:?} {:?}", array[0].header.dwType,array[0].header.dwSize);
            }
            //UpdateWindow(hwnd);

            //std::thread::sleep_ms(2000);


            // let mut msg: MSG = mem::uninitialized();
            // PeekMessageW(&mut msg, ptr::null_mut(), 0, 0,0);
            // if msg.message != WM_INPUT{
            //     exit = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
            //     TranslateMessage(&mut msg);
	    //     DispatchMessageW(&mut msg);
            // }
        }


    }
}
