use std::cell::{Ref, RefCell, RefMut};

use wgpu::SurfaceTexture;

use super::RenderState;

pub struct RenderContext<'a> {
    state: &'a RenderState,
    surface_view: wgpu::TextureView,
    depth_view: Ref<'a, wgpu::TextureView>,
    encoder: RefCell<wgpu::CommandEncoder>,
}

impl<'a> RenderContext<'a> {
    pub fn new(state: &'a RenderState, surface_texture: &SurfaceTexture) -> Self {
        let encoder = state.device().create_command_encoder(&Default::default());
        let surface_view = surface_texture.texture.create_view(&Default::default());
        let depth_view = state.depth_view();

        Self {
            state,
            encoder: RefCell::new(encoder),
            surface_view,
            depth_view,
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        self.state.device()
    }

    pub fn queue(&self) -> &wgpu::Queue {
        self.state.queue()
    }

    pub fn encoder(&self) -> RefMut<'_, wgpu::CommandEncoder> {
        self.encoder.borrow_mut()
    }

    pub fn surface_view(&self) -> &wgpu::TextureView {
        &self.surface_view
    }

    pub fn depth_view(&self) -> &wgpu::TextureView {
        &self.depth_view
    }

    pub fn finish(self) -> wgpu::CommandBuffer {
        self.encoder.into_inner().finish()
    }
}
