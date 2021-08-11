use super::graphics;
use futures::executor::block_on;
use std::time::Instant;
use winit::event::*;
use winit::window::Window;

pub struct App {
    pub graphic: graphics::GraphicBundle,
    pub last_update: Instant,
}

impl App {
    pub fn new(window: &Window) -> Self {
        let graphic = block_on(graphics::GraphicBundle::new(&window));
        Self {
            graphic,
            last_update: Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.graphic.resize(new_size)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
            ..
        } = event
        {
            self.graphic.colored = !self.graphic.colored;
            return true;
        }
        false
    }

    pub fn update(&mut self) {
        let _since_last_update = self.last_update.elapsed();
        // Do stuff...

        self.last_update = Instant::now();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.graphic.render()
    }
    #[inline]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.graphic.window_bundle.size
    }
}
