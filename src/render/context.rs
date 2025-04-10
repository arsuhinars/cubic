use std::cell::{Ref, RefCell, RefMut};

use wgpu::SurfaceTexture;

use super::RenderState;

pub struct RenderContext<'a> {
    encoder: RefCell<wgpu::CommandEncoder>,
    surface_view: wgpu::TextureView,
    depth_view: Ref<'a, wgpu::TextureView>,
}

impl<'a> RenderContext<'a> {
    pub fn new(state: &'a RenderState, surface_texture: &SurfaceTexture) -> Self {
        let encoder = state.device().create_command_encoder(&Default::default());
        let surface_view = surface_texture.texture.create_view(&Default::default());
        let depth_view = state.depth_view();

        Self {
            encoder: RefCell::new(encoder),
            surface_view,
            depth_view,
        }
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
