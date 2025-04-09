use std::sync::Arc;

use anyhow::anyhow;
use winit::window::Window;

pub struct RenderState {
    device: wgpu::Device,
    queue: wgpu::Queue,
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

        let render_state = Self { device, queue };

        Ok((render_state, surface, surface_config))
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
