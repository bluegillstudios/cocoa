// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use crate::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct App {
    event_loop: Option<EventLoop<()>>,
}

impl App {
    // Create a new instance. 
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        App {
            event_loop: Some(event_loop),
        }
    }

    pub fn run(mut self, mut window: Window) {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } if window_id == window.id() => {
                        *control_flow = ControlFlow::Exit;
                    }

                    Event::WindowEvent { event, .. } => {
                        window.handle_event(&event);
                    }

                    Event::RedrawRequested(window_id) if window_id == window.id() => {
                        window.render();
                    }

                    _ => (),
                }
            });
        }
    }
}
Explanation