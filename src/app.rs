use std::{fs::File, io::Read, iter, path::Path, rc::Rc, sync::Arc};

use serde::Deserialize;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::{
    render::{
        RenderChain, RenderContext, RenderState,
        stage::clear::{ClearStage, ClearStageParams},
    },
    utils::clock::FrameClock,
};

#[derive(Deserialize)]
pub struct AppConfig {
    pub resolution: [u32; 2],
    pub max_frame_rate: f64,
}

impl AppConfig {
    pub fn load_from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;

        Ok(toml::from_str(&buf)?)
    }
}

pub struct App {
    config: Box<AppConfig>,
    state: Option<AppState>,
}

struct AppState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_default_config: wgpu::SurfaceConfiguration,
    frame_clock: FrameClock,
    render_state: Rc<RenderState>,
    render_chain: RenderChain,
}

impl App {
    pub fn new(config: Box<AppConfig>) -> Self {
        Self {
            config,
            state: None,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)?;

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_inner_size(PhysicalSize::<u32>::from(self.config.resolution))
            .with_resizable(false)
            .with_title("Cubic");

        match event_loop
            .create_window(window_attributes)
            .map_err(anyhow::Error::new)
            .and_then(|window| pollster::block_on(AppState::new(&self.config, window)))
        {
            Ok(mut state) => {
                state.reconfigure_surface();
                self.state = Some(state);
            }
            Err(err) => {
                println!("Failed to open window: {}", err);
                event_loop.exit();
            }
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.state = None;
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            if let Err(err) = state.on_render() {
                println!("Failed to render frame: {}", err);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                if let Some(state) = &mut self.state {
                    state.reconfigure_surface();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    if let Err(err) = state.on_render() {
                        println!("Failed to render frame: {}", err);
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}

impl AppState {
    async fn new(config: &AppConfig, window: Window) -> anyhow::Result<Self> {
        let window = Arc::new(window);
        let (render_state, surface, default_surface_config) =
            RenderState::new(window.clone()).await?;
        let render_state = Rc::new(render_state);

        let render_chain = RenderChain::builder(render_state.clone())
            .stage::<ClearStage>(ClearStageParams {
                color: wgpu::Color::WHITE,
                depth: 1.0,
            })
            .build();

        Ok(Self {
            window,
            surface,
            surface_default_config: default_surface_config,
            frame_clock: FrameClock::new(config.max_frame_rate),
            render_state,
            render_chain,
        })
    }

    fn reconfigure_surface(&mut self) {
        let window_size = self.window.inner_size();
        self.surface.configure(
            self.render_state.device(),
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                width: window_size.width,
                height: window_size.height,
                ..self.surface_default_config.clone()
            },
        );
        self.render_state.resize_depth_texture(window_size.into());
    }

    fn on_render(&mut self) -> anyhow::Result<()> {
        let surface_texture = self.surface.get_current_texture()?;
        let context = RenderContext::new(&self.render_state, &surface_texture);

        self.render_chain.render(&context);

        self.render_state
            .queue()
            .submit(iter::once(context.finish()));
        self.window.pre_present_notify();
        surface_texture.present();
        self.frame_clock.wait_next_frame();

        Ok(())
    }
}
