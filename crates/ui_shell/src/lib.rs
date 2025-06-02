use gpui::{AppContext, Application, Render, RenderContext, WindowOptions};

pub struct Root;

impl Render for Root {
    fn render(&mut self, _cx: &mut RenderContext) {}
}

pub fn run_window<F>(init: F)
where
    F: FnOnce(&mut AppContext),
{
    let app = Application::new();
    app.run(|cx| {
        init(cx);
        let opts = WindowOptions::new("Glider", 1280, 800);
        cx.open_window(opts, Root);
    });
}
