// This file contains the logical part of an application

use super::graphics;
use super::graphics::Vertex;
use futures::executor::block_on;
use std::time::Instant;
use winit::event::*;
use winit::window::Window;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [ -0.5,  0.5,  0.0 ], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ -0.5, -0.5,  0.0 ], color: [0.0, 1.0, 0.0] },
    Vertex { position: [  0.5, -0.5,  0.0 ], color: [0.0, 0.0, 1.0] },
    Vertex { position: [  0.5,  0.5,  0.0 ], color: [0.0, 0.0, 0.0] },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
    0 // pading
];

pub struct App {
    pub graphic: graphics::GraphicBundle,
    pub last_update: Instant,
}

impl App {
    pub fn new(window: &Window) -> Self {
        let graphic = block_on(graphics::GraphicBundle::new(&window, VERTICES, INDICES));
        Self {
            graphic,
            last_update: Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.graphic.resize(new_size)
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
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
