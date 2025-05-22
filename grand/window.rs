// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    window::{Window as WinitWindow, WindowBuilder},
    event_loop::EventLoopWindowTarget,
};

use crate::widget::Widget;

use glutin::{
    ContextBuilder, PossiblyCurrent, window::WindowBuilder as GlutinWindowBuilder, ContextWrapper,
};
use skia_safe::{
    gpu::{BackendRenderTarget, DirectContext, SurfaceOrigin},
    ColorType, Surface, Color, Paint, PaintStyle, Rect, Font, Typeface,
};
use winit::event_loop::EventLoop;

pub struct GpuSkiaRenderer {
    context: ContextWrapper<PossiblyCurrent, winit::window::Window>,
    gr_context: DirectContext,
    surface: Surface,
    width: u32,
    height: u32,
}

impl GpuSkiaRenderer {
    pub fn new(winit_window: &winit::window::Window) -> Self {
        let size = winit_window.inner_size();
        let event_loop = winit_window
            .event_loop()
            .expect("Failed to get event loop from window");

        // Since we need the EventLoop to build Glutin context, 
        // it's better to pass the EventLoop separately to Window::new (see below).
        // For now, this function assumes the caller provides it.

        panic!("Use Window::new_with_event_loop instead to create GpuSkiaRenderer"); // If this ever goes off, it's fucked.
    }

    pub fn new_with_event_loop<T>(
        event_loop: &EventLoopWindowTarget<T>,
        title: &str,
        width: u32,
        height: u32,
    ) -> (Self, winit::window::Window) {
        // Build Glutin context with window
        let wb = GlutinWindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height));

        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, event_loop)
            .expect("Failed to build windowed context")
            .make_current()
            .expect("Failed to make context current");

        let winit_window = windowed_context.window().clone();

        let gl = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        let gr_context = DirectContext::new_gl(Some(&gl), None)
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
            &gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Failed to create Skia Surface");

        (
            Self {
                context: windowed_context,
                gr_context,
                surface,
                width,
                height,
            },
            winit_window,
        )
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

    pub fn draw(&mut self, root_widget: &mut dyn Widget) {
        let canvas = self.surface.canvas();

        // Clear background white
        canvas.clear(Color::WHITE);

        // Draw the widget tree on the canvas
        root_widget.render(canvas);

        // Flush drawing commands and swap buffers
        self.surface.flush_and_submit();
        self.context.swap_buffers().unwrap();
    }
}

pub struct Window {
    winit_window: WinitWindow,
    renderer: GpuSkiaRenderer,
    root_widget: Box<dyn Widget>,
    size: PhysicalSize<u32>,
}

impl Window {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        title: &str,
        width: u32,
        height: u32,
        root_widget: Box<dyn Widget>,
    ) -> Self {
        let (renderer, winit_window) = GpuSkiaRenderer::new_with_event_loop(event_loop, title, width, height);
        let size = winit_window.inner_size();

        Self {
            winit_window,
            renderer,
            root_widget,
            size,
        }
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        use winit::event::WindowEvent;

        match event {
            WindowEvent::Resized(new_size) => {
                self.size = *new_size;
                self.renderer.resize(new_size.width, new_size.height);
                self.winit_window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.size = **new_inner_size;
                self.renderer.resize(new_inner_size.width, new_inner_size.height);
                self.winit_window.request_redraw();
            }
            _ => {
                // Forward other events to the widget tree
                self.root_widget.handle_event(event);
            }
        }
    }

    pub fn render(&mut self) {
        self.renderer.draw(self.root_widget.as_mut());
    }

    pub fn request_redraw(&self) {
        self.winit_window.request_redraw();
    }
}
