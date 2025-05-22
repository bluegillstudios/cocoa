// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use crate::window::Window;
use crate::widget::Widget; 
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct App {
    event_loop: Option<EventLoop<()>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            event_loop: Some(EventLoop::new()),
        }
    }

    pub fn run(mut self, mut root_widget: Box<dyn Widget>) {
        if let Some(event_loop) = self.event_loop.take() {
            let winit_window = WindowBuilder::new()
                .with_title("Cocoa GUI")
                .build(&event_loop)
                .unwrap();

            let mut window = Window::new(winit_window, root_widget);

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } if window_id == window.id() => {
                        *control_flow = ControlFlow::Exit;
                    }

                    Event::WindowEvent { event, window_id } if window_id == window.id() => {
                        window.handle_event(&event);
                    }

                    Event::RedrawRequested(window_id) if window_id == window.id() => {
                        window.render();
                    }

                    Event::MainEventsCleared => {
                        // window.request_redraw();
                    }

                    _ => {}
                }
            });
        }
    }
}
