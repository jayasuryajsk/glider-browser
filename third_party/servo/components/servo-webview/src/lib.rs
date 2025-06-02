use std::sync::Arc;
use wgpu::{Device, Queue, TextureView};

pub struct WebView;

impl WebView {
    pub fn new(_url: &str, _device: Arc<Device>, _queue: Arc<Queue>) -> Self {
        WebView
    }

    pub fn next_frame(&mut self) -> Option<TextureView> {
        None
    }
}
