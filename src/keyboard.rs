use winapi::*;
use user32::*;
use kernel32::*;
use event::*;

#[derive(Clone)]
enum KeyPos {
    Left,
    Right,
}


pub fn process_keyboard_data(raw_data: &RAWKEYBOARD, id: usize) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    let flags = raw_data.Flags as u32;
    let key = raw_data.VKey as i32;
    let mut key_opt: Option<KeyId> = None;
    let key_state: State;
    let key_pos: KeyPos;
    if flags & RI_KEY_BREAK != 0 {
        key_state = State::Released;
    }
    else {
        key_state = State::Pressed;
    }
    if flags & RI_KEY_E0 != 0 {
        key_pos = KeyPos::Left;
    }
    else {
        key_pos = KeyPos::Right;
    }
    if key == VK_ESCAPE {
        key_opt = Some(KeyId::Escape);
    }
    if key == VK_RETURN {
        key_opt = Some(KeyId::Return);
    }
    if key == VK_BACK {
        key_opt = Some(KeyId::Backspace);
    }
    if key == VK_LEFT {
        key_opt = Some(KeyId::Left);
    }
    if key == VK_RIGHT {
        key_opt = Some(KeyId::Right);
    }
    if key == VK_UP {
        key_opt = Some(KeyId::Up);
    }
    if key == VK_DOWN {
        key_opt = Some(KeyId::Down);
    }
    if key == VK_SPACE {
        key_opt = Some(KeyId::Space);
    }
    if key == VK_LSHIFT {
        key_opt = Some(KeyId::LeftShift);
    }
    if key == VK_RSHIFT {
        key_opt = Some(KeyId::RightShift);
    }
    if key == VK_LCONTROL {
        key_opt = Some(KeyId::LeftCtrl);
    }
    if key == VK_RCONTROL {
        key_opt = Some(KeyId::RightCtrl);
    }
    if key == VK_LMENU {
        key_opt = Some(KeyId::LeftAlt);
    }
    if key == VK_RMENU {
        key_opt = Some(KeyId::RightAlt);
    }
    if key == 0x30 {
        key_opt = Some(KeyId::Zero);
    }
    if key == 0x31 {
        key_opt = Some(KeyId::One);
    }
    if key == 0x32 {
        key_opt = Some(KeyId::Two);
    }
    if key == 0x33 {
        key_opt = Some(KeyId::Three);
    }
    if key == 0x34 {
        key_opt = Some(KeyId::Four);
    }
    if key == 0x35 {
        key_opt = Some(KeyId::Five);
    }
    if key == 0x36 {
        key_opt = Some(KeyId::Six);
    }
    if key == 0x37 {
        key_opt = Some(KeyId::Seven);
    }
    if key == 0x38 {
        key_opt = Some(KeyId::Eight);
    }
    if key == 0x39 {
        key_opt = Some(KeyId::Nine);
    }
    if key == 0x41 {
        key_opt = Some(KeyId::A);
    }
    if key == 0x42 {
        key_opt = Some(KeyId::B);
    }
    if key == 0x43 {
        key_opt = Some(KeyId::C);
    }
    if key == 0x44 {
        key_opt = Some(KeyId::D);
    }
    if key == 0x45 {
        key_opt = Some(KeyId::E);
    }
    if key == 0x46 {
        key_opt = Some(KeyId::F);
    }
    if key == 0x47 {
        key_opt = Some(KeyId::G);
    }
    if key == 0x48 {
        key_opt = Some(KeyId::H);
    }
    if key == 0x49 {
        key_opt = Some(KeyId::I);
    }
    if key == 0x4A {
        key_opt = Some(KeyId::J);
    }
    if key == 0x4B {
        key_opt = Some(KeyId::K);
    }
    if key == 0x4C {
        key_opt = Some(KeyId::L);
    }
    if key == 0x4D {
        key_opt = Some(KeyId::M);
    }
    if key == 0x4E {
        key_opt = Some(KeyId::N);
    }
    if key == 0x4F {
        key_opt = Some(KeyId::O);
    }
    if key == 0x50 {
        key_opt = Some(KeyId::P);
    }
    if key == 0x51 {
        key_opt = Some(KeyId::Q);
    }
    if key == 0x52 {
        key_opt = Some(KeyId::R);
    }
    if key == 0x53 {
        key_opt = Some(KeyId::S);
    }
    if key == 0x54 {
        key_opt = Some(KeyId::T);
    }
    if key == 0x55 {
        key_opt = Some(KeyId::U);
    }
    if key == 0x56 {
        key_opt = Some(KeyId::V);
    }
    if key == 0x57 {
        key_opt = Some(KeyId::W);
    }
    if key == 0x58 {
        key_opt = Some(KeyId::X);
    }
    if key == 0x59 {
        key_opt = Some(KeyId::Y);
    }
    if key == 0x5A {
        key_opt = Some(KeyId::Z);
    }

    if let Some(key_id) = key_opt {
        output.push(RawEvent::KeyboardEvent(id, key_id, key_state));
        }
    output
}
