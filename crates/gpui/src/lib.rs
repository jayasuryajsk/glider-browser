use std::sync::Arc;
use wgpu::{Device, Queue};

pub struct Application;

impl Application {
    pub fn new() -> Self {
        Application
    }

    pub fn run<F: FnOnce(&mut AppContext)>(self, f: F) {
        let mut cx = AppContext::new();
        f(&mut cx);
    }
}

pub struct AppContext;

impl AppContext {
    fn new() -> Self {
        AppContext
    }

    pub fn open_window<T: Render + 'static>(&mut self, _options: WindowOptions, mut content: T) {
        let mut rc = RenderContext;
        content.render(&mut rc);
    }

    pub fn gpu(&self) -> Gpu {
        Gpu
    }

    pub fn needs_repaint(&mut self) {}
}

pub struct Gpu;

impl Gpu {
    pub fn device(&self) -> Arc<Device> {
        unimplemented!()
    }

    pub fn queue(&self) -> Arc<Queue> {
        unimplemented!()
    }
}

pub trait Render {
    fn render(&mut self, cx: &mut RenderContext);
}

pub struct RenderContext;

#[derive(Default)]
pub struct WindowOptions {
    pub title: String,
    pub size: (u32, u32),
}

impl WindowOptions {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            size: (width, height),
        }
    }
}
