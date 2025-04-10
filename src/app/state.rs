use std::{iter, rc::Rc, sync::Arc, time::Instant};

use winit::window::Window;

use crate::{
    render::{
        RenderChain, RenderContext, RenderState,
        stage::clear::{ClearStage, ClearStageParams},
    },
    ui::UiState,
    utils::FrameClock,
};

use super::{config::AppConfig, context::AppContext};

pub struct AppState {
    render_chain: RenderChain,
    pub ui_state: UiState,
    pub render_state: Rc<RenderState>,
    pub window: Arc<Window>,
    frame_clock: FrameClock,
    context: Box<AppContext>,
}

impl AppState {
    pub async fn new(config: &AppConfig, window: Window) -> anyhow::Result<Self> {
        let window = Arc::new(window);
        let render_state = Rc::new(RenderState::new(window.clone()).await?);
        let render_chain = RenderChain::builder(render_state.clone())
            .stage::<ClearStage>(ClearStageParams {
                color: wgpu::Color::WHITE,
                depth: 1.0,
            })
            .build();
        let ui_state = UiState::new(&render_state, &window);

        Ok(Self {
            window,
            frame_clock: FrameClock::new(config.max_frame_rate),
            render_state,
            render_chain,
            ui_state,
            context: Box::new(AppContext::new()),
        })
    }

    pub fn on_render(&mut self) -> anyhow::Result<()> {
        let start_time = Instant::now();

        let surface_texture = self.render_state.surface_texture()?;
        let render_context = RenderContext::new(&self.render_state, &surface_texture);

        self.render_chain.render(&render_context);

        let ui_cmd_buf = self
            .ui_state
            .render(&self.window, &self.context, &render_context);

        self.render_state
            .queue()
            .submit(iter::once(render_context.finish()).chain(ui_cmd_buf));
        self.context.render_time = (Instant::now() - start_time).as_secs_f32();
        self.window.pre_present_notify();
        surface_texture.present();

        self.frame_clock.wait_next_frame();
        self.context.delta_time = (Instant::now() - start_time).as_secs_f32();

        Ok(())
    }
}
