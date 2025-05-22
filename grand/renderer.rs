// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use skia_safe::{Surface, Canvas, Color};
use winit::window::Window;

pub struct SkiaRenderer {
    surface: Surface,
    width: u32,
    height: u32,
}

impl SkiaRenderer {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        // Create a new Skia surface.
        let surface = Surface::new_raster_n32_premul((width as i32, height as i32))
            .expect("Failed to create Skia surface");

        Self { surface, width, height }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == self.width && height == self.height {
            return;
        }
        self.surface = Surface::new_raster_n32_premul((width as i32, height as i32))
            .expect("Failed to resize Skia surface");
        self.width = width;
        self.height = height;
    }

    pub fn draw(&mut self, widget: &mut dyn crate::widget::Widget) {
        let canvas: &mut Canvas = self.surface.canvas();
        canvas.clear(Color::WHITE);
        widget.draw(canvas);
        self.surface.flush_and_submit();
    }
}
