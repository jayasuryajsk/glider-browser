use browser_core::WebPage;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut headless = false;
    let mut url = "https://www.google.com".to_string();

    if let Some(arg) = args.next() {
        if arg == "--headless" {
            headless = true;
            if let Some(u) = args.next() {
                url = u;
            }
        } else {
            url = arg;
        }
    }

    if headless {
        return;
    }

    ui_shell::run_window(|cx| {
        let device = cx.gpu().device();
        let queue = cx.gpu().queue();
        let mut page = WebPage::new(&url, device, queue);
        if let Some(_tex) = page.next_frame() {
            cx.needs_repaint();
        }
    });
}
