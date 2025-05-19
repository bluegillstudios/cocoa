// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License, v2.0.

use winit::event::{MouseButton, VirtualKeyCode, ModifiersState};

#[derive(Debug, Clone)]
pub enum Event {
    MouseDown { x: f32, y: f32, button: MouseButton, modifiers: ModifiersState },
    MouseUp { x: f32, y: f32, button: MouseButton, modifiers: ModifiersState },
    MouseMove { x: f32, y: f32, modifiers: ModifiersState },
    KeyDown { key: Option<VirtualKeyCode>, modifiers: ModifiersState },
    KeyUp { key: Option<VirtualKeyCode>, modifiers: ModifiersState },
    CharInput(char),
    FocusGained,
    FocusLost,
    Resized { width: u32, height: u32 },
    RedrawRequested,
}
