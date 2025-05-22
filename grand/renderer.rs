// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use skia_safe::{Surface, Canvas, Color};
use winit::window::Window;
use skia_safe::{Canvas, Paint, PaintStyle, Color, Rect, Font, Typeface};

impl Renderer {
    pub fn draw_button(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        is_pressed: bool,
    ) {
        if let Some(canvas) = self.canvas.as_mut() {
            let mut bg_paint = Paint::default();
            bg_paint.set_anti_alias(true);

            // Trying to look like Mac's, right?
            let (r, g, b) = if is_pressed {
                (180, 180, 180)
            } else {
                (230, 230, 230)
            };
            bg_paint.set_color(Color::from_rgb(r, g, b));
            bg_paint.set_style(PaintStyle::Fill);

            let button_rect = Rect::from_xywh(x, y, width, height);
            canvas.draw_rounded_rect(button_rect, 6.0, 6.0, &bg_paint);

            // Draw border
            let mut border_paint = Paint::default();
            border_paint.set_anti_alias(true);
            border_paint.set_color(Color::from_rgb(160, 160, 160));
            border_paint.set_style(PaintStyle::Stroke);
            border_paint.set_stroke_width(1.0);
            canvas.draw_rounded_rect(button_rect, 6.0, 6.0, &border_paint);

            // Draw label
            let typeface = Typeface::default();
            let font = Font::new(typeface, 16.0);
            let mut text_paint = Paint::default();
            text_paint.set_anti_alias(true);
            text_paint.set_color(Color::BLACK);

            let text_x = x + (width - font.measure_str(label, Some(&text_paint)).1.width) / 2.0;
            let text_y = y + height / 2.0 + 5.0;
            canvas.draw_str(label, (text_x, text_y), &font, &text_paint);
        }
    }
}

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
