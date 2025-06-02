use std::sync::Arc;

use servo_webview::{WebView, WebViewBuilder};
use wgpu::{Device, Queue, TextureView};

pub struct WebPage {
    inner: WebView,
}

impl WebPage {
    pub fn new(
        url: &str,
        device: Arc<Device>,
        queue: Arc<Queue>,
    ) -> Self {
        let inner = WebViewBuilder::new(device, queue)
            .url(url)
            .build()
            .unwrap();
        Self { inner }
    }

    pub fn next_frame(&mut self) -> Option<TextureView> {
        self.inner.next_frame()
    }
}
