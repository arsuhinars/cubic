use std::rc::Rc;

use super::{RenderContext, RenderState};

pub mod clear;

pub trait RenderStage {
    fn render(&self, context: &RenderContext);
}

pub trait RenderStageSetup: RenderStage {
    type Params;

    fn setup(render_state: Rc<RenderState>, params: Self::Params) -> Self;
}
