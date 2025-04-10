use std::{
    cell::{Ref, RefCell},
    sync::Arc,
};

use anyhow::anyhow;
use winit::window::Window;

const DEPTH_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct RenderState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    depth_texture: RefCell<(wgpu::Texture, wgpu::TextureView)>,
}

impl RenderState {
    pub async fn new(
        window: Arc<Window>,
    ) -> anyhow::Result<(Self, wgpu::Surface<'static>, wgpu::SurfaceConfiguration)> {
        let instance = wgpu::Instance::new(&Default::default());
        let window_size = window.inner_size();
        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or_else(|| anyhow!("Failed to create wgpu adapter"))?;

        let surface_config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .ok_or_else(|| anyhow!("Failed to create surface config"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    ..Default::default()
                },
                None,
            )
            .await?;

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: window_size.width,
                height: window_size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&Default::default());

        let render_state = Self {
            device,
            queue,
            depth_texture: RefCell::new((depth_texture, depth_view)),
        };

        Ok((render_state, surface, surface_config))
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn resize_depth_texture(&self, size: [u32; 2]) {
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&Default::default());
        self.depth_texture.replace((depth_texture, depth_view));
    }

    pub fn depth_view(&self) -> Ref<'_, wgpu::TextureView> {
        Ref::map(self.depth_texture.borrow(), |(_, v)| v)
    }
}
