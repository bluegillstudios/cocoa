// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License, v2.0. 

use winit::event::{WindowEvent, KeyboardInput, ElementState, MouseButton, ModifiersState, VirtualKeyCode};
use crate::event::types::Event;

pub fn translate_event(
    event: &WindowEvent<'_>,
    modifiers: ModifiersState,
) -> Option<Event> {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            Some(Event::MouseMove {
                x: position.x as f32,
                y: position.y as f32,
                modifiers,
            })
        }
        WindowEvent::MouseInput { state, button, .. } => match state {
            ElementState::Pressed => Some(Event::MouseDown {
                x: 0.0,
                y: 0.0,
                button: *button,
                modifiers,
            }),
            ElementState::Released => Some(Event::MouseUp {
                x: 0.0,
                y: 0.0,
                button: *button,
                modifiers,
            }),
        },
        WindowEvent::KeyboardInput { input, .. } => {
            match input.state {
                ElementState::Pressed => Some(Event::KeyDown {
                    key: input.virtual_keycode,
                    modifiers,
                }),
                ElementState::Released => Some(Event::KeyUp {
                    key: input.virtual_keycode,
                    modifiers,
                }),
            }
        }
        WindowEvent::ReceivedCharacter(c) => Some(Event::CharInput(*c)),
        WindowEvent::Focused(true) => Some(Event::FocusGained),
        WindowEvent::Focused(false) => Some(Event::FocusLost),
        WindowEvent::Resized(size) => Some(Event::Resized {
            width: size.width,
            height: size.height,
        }),
        WindowEvent::RedrawRequested => Some(Event::RedrawRequested),
        _ => None,
    }
}
