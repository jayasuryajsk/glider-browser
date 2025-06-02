use browser_core::WebPage;
use gpui::Layout;

fn main() {
    let url = std::env::args().nth(1).unwrap_or_else(|| "https://www.google.com".to_string());

    ui_shell::run_window(move |cx| {
        let device = cx.gpu().device();
        let queue = cx.gpu().queue();
        let mut page = WebPage::new(&url, device, queue);
        cx.window().set_title("Glider");

        if let Some(tex) = page.next_frame() {
            cx.draw(tex, Layout::fit_parent());
            cx.needs_repaint();
        }
    });
}
