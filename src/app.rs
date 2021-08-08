#![allow(unused)]
use super::graphics;
use futures::executor::block_on;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct App {
    pub graphic: graphics::GraphicBundle,
}

impl App {
    pub fn new(window: &Window) -> Self {
        let graphic = block_on(graphics::GraphicBundle::new(&window));
        Self { graphic }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.graphic.resize(new_size)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.graphic.render()
    }
    #[inline]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.graphic.window_bundle.size
    }
}
