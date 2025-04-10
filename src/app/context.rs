pub struct AppContext {
    pub delta_time: f32,
    pub render_time: f32,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            render_time: 0.0,
        }
    }
}
