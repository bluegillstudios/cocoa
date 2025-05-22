// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

use glutin::{
    config::{ConfigTemplateBuilder, Config},
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{Display, DisplayApiPreference},
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use skia_safe::{
    gpu, Color, Surface as SkSurface, Canvas,
};
use winit::{
    dpi::PhysicalSize,
    window::Window,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct GpuSkiaRenderer {
    gl_context: PossiblyCurrentContext,
    surface: Surface,
    gr_context: gpu::DirectContext,
    skia_surface: SkSurface,
    width: i32,
    height: i32,
}

impl GpuSkiaRenderer {
    pub fn new(window: &Window) -> Self {
        // Create display (platform-dependent)
        let raw_display_handle = window.raw_display_handle();
        let display = unsafe {
            Display::new(raw_display_handle, DisplayApiPreference::Egl)
                .expect("Failed to create glutin Display")
        };

        // Choose OpenGL config
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24)
            .with_stencil_size(8)
            .with_transparency(false)
            .build();
        let config = unsafe {
            display.find_configs(template)
                .unwrap()
                .next()
                .expect("Failed to find a matching GL config")
        };

        // Create OpenGL context attributes
        let raw_window_handle = window.raw_window_handle();
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                major: 3,
                minor: 3,
            })))
            .build(Some(raw_window_handle));

        // Create GL context
        let not_current_gl_context = unsafe {
            display.create_context(&config, &context_attributes)
                .expect("Failed to create GL context")
        };

        // Create window surface attributes
        let size = window.inner_size();
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            size.width as u32,
            size.height as u32,
        );

        // Create GL surface for window
        let gl_surface = unsafe {
            display.create_window_surface(&config, &surface_attributes)
                .expect("Failed to create GL window surface")
        };

        // Make context current
        let gl_context = not_current_gl_context.make_current(&gl_surface)
            .expect("Failed to make GL context current");

        // Create Skia DirectContext for GPU
        let interface = gpu::gl::Interface::new_load_with(|s| {
            gl_context.get_proc_address(s) as *const _
        }).expect("Failed to create GL interface");

        let gr_context = gpu::DirectContext::new_gl(Some(interface), None)
            .expect("Failed to create Skia GPU DirectContext");

        // Create Skia Surface from GL framebuffer
        let fb_info = {
            use skia_safe::gpu::gl::FramebufferInfo;
            FramebufferInfo {
                fboid: 0, // default framebuffer
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let backend_render_target = gpu::BackendRenderTarget::new_gl(
            (size.width as i32, size.height as i32),
            0, // sample count
            8, // stencil bits
            fb_info,
        );

        let skia_surface = SkSurface::from_backend_render_target(
            &gr_context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        ).expect("Failed to create Skia surface");

        Self {
            gl_context,
            surface: gl_surface,
            gr_context,
            skia_surface,
            width: size.width as i32,
            height: size.height as i32,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width as i32 == self.width && height as i32 == self.height {
            return;
        }
        self.surface.resize(width, height);

        let fb_info = {
            use skia_safe::gpu::gl::FramebufferInfo;
            FramebufferInfo {
                fboid: 0,
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let backend_render_target = gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            0,
            8,
            fb_info,
        );

        self.skia_surface = SkSurface::from_backend_render_target(
            &self.gr_context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        ).expect("Failed to recreate Skia surface");

        self.width = width as i32;
        self.height = height as i32;
    }

    pub fn draw(&mut self, widget: &mut dyn crate::widget::Widget) {
        let canvas = self.skia_surface.canvas();
        canvas.clear(skia_safe::colors::WHITE);

        widget.draw(canvas);

        self.skia_surface.flush_and_submit();
        self.gl_context.swap_buffers(&self.surface).expect("Failed to swap buffers");
    }
}

