#[derive(Clone)]
pub enum State {
    Pressed,
    Released,
}

#[derive(Clone)]
pub enum KeyPos {
    Left,
    Right,
}

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
    Shift,
    Ctrl,
    Alt,
}

#[derive(Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone)]
pub enum RawEvent {
    MouseButtonEvent(usize,MouseButton,State),
    MouseMoveEvent(usize,i32,i32),
    MouseWheelEvent(usize,u16),
    KeyboardEvent(usize,KeyId,State,KeyPos),
}
