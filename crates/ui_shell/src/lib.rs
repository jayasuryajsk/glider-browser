use gpui::{self, AppContext, Application, Render, RenderContext};

pub struct Root;

impl Render for Root {
    fn render(&mut self, _cx: &mut RenderContext) {}
}

pub fn run_window<F>(init: F)
where
    F: FnOnce(&mut gpui::AppContext) + 'static,
{
    gpui::Application::new().run(move |cx| {
        init(cx);
    });
}
