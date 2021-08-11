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

const VERTICES2: &[Vertex] = &[
    Vertex {
        position: [0.064_849_9, 0.473_387_54, 0.0],
        color: [-0.539_553_3, 0.981_370_15, -0.229_005_23],
    },
    Vertex {
        position: [-0.257_165_1, -0.425_222_6, 0.0],
        color: [-0.943_981_05, -0.940_584_5, -0.072_859_32],
    },
    Vertex {
        position: [-0.823_901_95, 0.480_195_88, 0.0],
        color: [-0.057_382_57, -0.590_910_43, 0.528_234_8],
    },
    Vertex {
        position: [-0.521_578_1, -0.483_975_68, 0.0],
        color: [0.821_092_96, -0.540_119_35, -0.583_434_46],
    },
    Vertex {
        position: [0.476_507_46, -0.985_589_27, 0.0],
        color: [0.467_114_54, 0.911_523_76, -0.838_063_1],
    },
];
#[rustfmt::skip]
const INDICES2: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    0
];

pub struct App {
    pub graphic: graphics::GraphicBundle,
    pub last_update: Instant,
}

impl App {
    pub fn new(window: &Window) -> Self {
        let graphic = block_on(graphics::GraphicBundle::new(
            &window, VERTICES, INDICES, VERTICES2, INDICES2,
        ));
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
            self.graphic.vertex1 = !self.graphic.vertex1;
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
