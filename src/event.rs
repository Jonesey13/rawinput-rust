/// State of a Key or Button
#[derive(Clone)]
pub enum State {
    Pressed,
    Released,
}

/// Key Identifier
#[derive(Clone)]
pub enum KeyId {
    Escape,
    Return,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
}

/// Mouse Buttons
#[derive(Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

/// Event types
///
/// The usize entry acts as a device ID unique to each DeviceType (Mouse, Keyboard, Hid)
#[derive(Clone)]
pub enum RawEvent {
    MouseButtonEvent(usize,MouseButton,State),
    MouseMoveEvent(usize,i32,i32),
    MouseWheelEvent(usize,f32),
    KeyboardEvent(usize,KeyId,State),
}
