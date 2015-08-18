use winapi::*;
use user32::*;
use kernel32::*;
use event::*;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RAWMOUSEMOD {
    pub usFlags: USHORT,
    pub memory_padding: USHORT, // 16bit Padding for 32bit align in following union
    pub usButtonFlags: USHORT,
    pub usButtonData: USHORT,
    pub ulRawButtons: ULONG,
    pub lLastX: LONG,
    pub lLastY: LONG,
    pub ulExtraInformation: ULONG,
}


pub fn process_mouse_data(raw_data: &RAWMOUSEMOD, id: usize) -> Vec<RawEvent> {
    let cursor = (raw_data.lLastX, raw_data.lLastY);
    let buttons = raw_data.usButtonFlags;
    let mut output: Vec<RawEvent> = Vec::new();
    if buttons & RI_MOUSE_LEFT_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Left, State::Pressed ));
    }
    if buttons & RI_MOUSE_LEFT_BUTTON_UP != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Left, State::Released ));
    }
    if buttons & RI_MOUSE_RIGHT_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Right, State::Pressed ));
    }
    if buttons & RI_MOUSE_RIGHT_BUTTON_UP != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Right, State::Released ));
    }
    if buttons & RI_MOUSE_MIDDLE_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Middle, State::Pressed ));
    }
    if buttons & RI_MOUSE_MIDDLE_BUTTON_UP != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Middle, State::Released ));
    }
    if buttons & RI_MOUSE_WHEEL != 0{
        let wheel_data = raw_data.usButtonData;
        output.push(RawEvent::MouseWheelEvent(id, wheel_data));
    }
    if (cursor.0 != 0) || (cursor.1 != 0) {
        output.push(RawEvent::MouseMoveEvent(id, cursor.0, cursor.1));
    }
    output
}
