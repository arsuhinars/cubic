pub use config::AppConfig;
pub use context::AppContext;
use state::AppState;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod config;
mod context;
mod state;

pub struct App {
    config: Box<AppConfig>,
    state: Option<AppState>,
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
            Ok(state) => {
                state
                    .render_state
                    .resize_surface(state.window.inner_size().into());
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
            WindowEvent::Resized(size) => {
                if let Some(state) = &mut self.state {
                    state.render_state.resize_surface(size.into());
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

        if let Some(state) = &mut self.state {
            let _ = state
                .ui_state
                .winit_state()
                .on_window_event(&state.window, &event);
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if let Some(state) = &mut self.state {
                    state.ui_state.winit_state().on_mouse_motion(delta);
                }
            }
            _ => (),
        }
    }
}
