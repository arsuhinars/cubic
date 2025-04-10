use egui::{Color32, Frame};
use winit::window::Window;

use crate::{
    app::AppContext,
    render::{RenderContext, RenderState},
    utils::UiUtils,
};

pub struct UiState {
    context: egui::Context,
    renderer: egui_wgpu::Renderer,
    winit_state: egui_winit::State,
    viewport_info: egui::ViewportInfo,
}

impl UiState {
    pub fn new(render_state: &RenderState, window: &Window) -> Self {
        let context = egui::Context::default();
        context.set_theme(egui::ThemePreference::Dark);

        let renderer = egui_wgpu::Renderer::new(
            render_state.device(),
            render_state.surface_format(),
            Some(render_state.depth_format()),
            1,
            false,
        );

        let winit_state = egui_winit::State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            None,
            None,
            None,
        );

        let mut viewport_info = Default::default();
        egui_winit::update_viewport_info(&mut viewport_info, &context, window, true);

        Self {
            context,
            renderer,
            winit_state,
            viewport_info,
        }
    }

    pub fn winit_state(&mut self) -> &mut egui_winit::State {
        &mut self.winit_state
    }

    pub fn render(
        &mut self,
        window: &Window,
        app_context: &AppContext,
        render_context: &RenderContext,
    ) -> Vec<wgpu::CommandBuffer> {
        egui_winit::update_viewport_info(&mut self.viewport_info, &self.context, window, false);
        let mut input = self.winit_state.take_egui_input(window);
        input
            .viewports
            .insert(egui::ViewportId::ROOT, self.viewport_info.clone());

        let output = self
            .context
            .clone()
            .run(input, |ctx| self.draw(app_context, ctx));
        let primitives = self
            .context
            .tessellate(output.shapes, output.pixels_per_point);
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: window.inner_size().into(),
            pixels_per_point: output.pixels_per_point,
        };

        for (id, delta) in output.textures_delta.set {
            self.renderer.update_texture(
                render_context.device(),
                render_context.queue(),
                id,
                &delta,
            );
        }
        let cmd_buf = self.renderer.update_buffers(
            render_context.device(),
            render_context.queue(),
            &mut render_context.encoder(),
            &primitives,
            &screen_descriptor,
        );

        let mut encoder = render_context.encoder();
        let mut render_pass = encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_context.surface_view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: render_context.depth_view(),
                    depth_ops: None,
                    stencil_ops: None,
                }),
                ..Default::default()
            })
            .forget_lifetime();

        self.renderer
            .render(&mut render_pass, &primitives, &screen_descriptor);

        cmd_buf
    }

    fn draw(&mut self, app_context: &AppContext, ctx: &egui::Context) {
        let frame = Frame::central_panel(&ctx.style()).fill(Color32::TRANSPARENT);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let fps = 1.0 / app_context.delta_time;
            let render_rate = 1.0 / app_context.render_time;
            ui.text(format!("FPS: {:.2}, render rate: {:.2}", fps, render_rate));
            ui.text(format!(
                "frame time: {:.2} ms., render time: {:.2} ms.",
                app_context.delta_time * 1000.0,
                app_context.render_time * 1000.0
            ));
        });
    }
}
