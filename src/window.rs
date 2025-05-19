// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use winit::{
    dpi::LogicalSize,
    event::WindowEvent,
    window::{Window as WinitWindow, WindowBuilder},
    event_loop::EventLoopWindowTarget,
};

use crate::widget::Widget;
use crate::renderer::Renderer;

use glutin::{
    ContextBuilder, PossiblyCurrent, window::WindowBuilder as GlutinWindowBuilder, ContextWrapper, dpi::PhysicalSize,
};
use skia_safe::{
    gpu::{BackendRenderTarget, DirectContext, SurfaceOrigin},
    ColorType, Surface, Color, Paint, PaintStyle, Rect, Font, Typeface,
};
use winit::event_loop::EventLoop;

pub struct GlSkiaContext {
    context: ContextWrapper<PossiblyCurrent, winit::window::Window>,
    gr_context: DirectContext,
    surface: Surface,
    width: u32,
    height: u32,
}

impl GlSkiaContext {
    pub fn new(event_loop: &EventLoop<()>, width: u32, height: u32, title: &str) -> Self {
        let wb = GlutinWindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height));

        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, event_loop)
            .expect("Failed to build windowed context")
            .make_current()
            .expect("Failed to make context current");

        let gl = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        let mut gr_context = DirectContext::new_gl(Some(&gl), None)
            .expect("Failed to create GrDirectContext");

        let fb_info = {
            use glutin::platform::ContextTraitExt;
            let fboid = unsafe { gl.GetInteger(glow::FRAMEBUFFER_BINDING) };
            skia_safe::gpu::gl::FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let backend_render_target = BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            0,
            8,
            fb_info,
        );

        let surface = Surface::from_backend_render_target(
            &mut gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Failed to create Skia Surface");

        Self {
            context: windowed_context,
            gr_context,
            surface,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return; // Ignore zero-sized windows
        }

        self.width = width;
        self.height = height;

        let fb_info = {
            use glutin::platform::ContextTraitExt;
            let gl = glow::Context::from_loader_function(|s| {
                self.context.get_proc_address(s) as *const _
            });
            let fboid = unsafe { gl.GetInteger(glow::FRAMEBUFFER_BINDING) };
            skia_safe::gpu::gl::FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let backend_render_target = BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            0,
            8,
            fb_info,
        );

        self.surface = Surface::from_backend_render_target(
            &mut self.gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Failed to recreate Skia Surface");
    }

    pub fn draw(&mut self) {
        let canvas = self.surface.canvas();

        // Clear background white
        canvas.clear(Color::WHITE);

        // Draw a filled blue rectangle
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Color::from_argb(255, 100, 149, 237)); // cornflower blue
        paint.set_style(PaintStyle::Fill);
        let rect = Rect::from_xywh(50.0, 50.0, 200.0, 100.0);
        canvas.draw_rect(rect, &paint);

        // Draw black text on top
        paint.set_color(Color::BLACK);
        let typeface = Typeface::default();
        let font = Font::new(typeface, 24.0);
        canvas.draw_str("cocoa", (60, 120), &font, &paint);

        // Flush drawing commands and swap buffers
        self.surface.flush_and_submit();
        self.context.swap_buffers().unwrap();
    }
}

pub struct Window {
    winit_window: WinitWindow,
    widgets: Vec<Box<dyn Widget>>,
    renderer: Renderer,
}

impl Window {
    pub fn new<T>(event_loop: &EventLoopWindowTarget<T>, title: &str, width: u32, height: u32) -> Self {
        let winit_window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .build(event_loop)
            .expect("Failed to create window");

        Window {
            winit_window,
            widgets: Vec::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        for widget in self.widgets.iter_mut() {
            widget.handle_event(event);
        }
    }
    pub fn render(&mut self) {
        for widget in self.widgets.iter_mut() {
            widget.render(&mut self.renderer);
        }

        // Present the rendered content to the window (TODO: backend-specific)
    }
}