#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub enum Event {
    MouseDown { x: i32, y: i32, button: MouseButton },
    MouseUp { x: i32, y: i32, button: MouseButton },
    MouseMove { x: i32, y: i32 },

    KeyDown { keycode: u32 },
    KeyUp { keycode: u32 },
}
