use std::rc::Rc;

use crate::render::{RenderContext, RenderState};

use super::{RenderStage, RenderStageSetup};

pub struct ClearStageParams {
    pub color: wgpu::Color,
    pub depth: f32,
}

pub struct ClearStage {
    params: ClearStageParams,
}

impl RenderStageSetup for ClearStage {
    type Params = ClearStageParams;

    fn setup(_: Rc<RenderState>, params: Self::Params) -> Self {
        Self { params }
    }
}

impl RenderStage for ClearStage {
    fn render(&self, context: &RenderContext) {
        let mut encoder = context.encoder();

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &context.surface_view(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.params.color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &context.depth_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.params.depth),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
    }
}
