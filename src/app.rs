use super::graphics;
use futures::executor::block_on;
use std::time::Instant;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct App {
    pub graphic: graphics::GraphicBundle,
    pub last_update: Instant,
    pub bg_color: wgpu::Color,
}

impl App {
    pub fn new(window: &Window) -> Self {
        let graphic = block_on(graphics::GraphicBundle::new(&window));
        Self {
            graphic,
            last_update: Instant::now(),
            bg_color: Default::default(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.graphic.resize(new_size)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.bg_color = wgpu::Color {
                    r: position.x / self.size().width as f64,
                    g: position.y / self.size().height as f64,
                    b: 0.0,
                    a: 1.0,
                };
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        let _since_last_update = self.last_update.elapsed();
        // Do stuff...

        self.last_update = Instant::now();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.graphic.render(self.bg_color)
    }
    #[inline]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.graphic.window_bundle.size
    }
}
