use egui::{Label, Response, Ui, WidgetText};

pub trait UiUtils {
    fn text(&mut self, text: impl Into<WidgetText>) -> Response;
}

impl UiUtils for Ui {
    fn text(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Label::new(text).selectable(false))
    }
}
