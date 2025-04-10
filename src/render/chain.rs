use std::rc::Rc;

use super::{
    RenderContext, RenderState,
    stage::{RenderStage, RenderStageSetup},
};

pub struct RenderChainBuilder {
    render_state: Rc<RenderState>,
    stages: Vec<Box<dyn RenderStage>>,
}

pub struct RenderChain {
    stages: Vec<Box<dyn RenderStage>>,
}

impl RenderChainBuilder {
    pub fn stage<T: RenderStageSetup + 'static>(
        mut self,
        params: <T as RenderStageSetup>::Params,
    ) -> Self {
        self.stages
            .push(Box::new(T::setup(self.render_state.clone(), params)));
        self
    }

    pub fn build(self) -> RenderChain {
        RenderChain {
            stages: self.stages,
        }
    }
}

impl RenderChain {
    pub fn builder(render_state: Rc<RenderState>) -> RenderChainBuilder {
        RenderChainBuilder {
            render_state,
            stages: Vec::new(),
        }
    }

    pub fn render(&mut self, context: &RenderContext) {
        for stage in &self.stages {
            stage.render(context);
        }
    }
}
