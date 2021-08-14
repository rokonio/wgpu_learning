// This file contains the logical part of an application

use super::graphics;
use super::graphics::Vertex;
use crate::camera::Camera;
use crate::camera::CameraController;
use futures::executor::block_on;
use std::time::Instant;
use winit::event::*;
use winit::window::Window;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    0
];

pub struct App {
    pub graphic: graphics::GraphicBundle,
    pub last_update: Instant,
    pub camera: Camera,
    pub camera_controller: CameraController,
}

impl App {
    pub fn new(window: &Window) -> Self {
        let graphic = block_on(graphics::GraphicBundle::new(&window, VERTICES, INDICES));
        Self {
            camera: Camera::default(&graphic.window_bundle),
            graphic,
            last_update: Instant::now(),
            camera_controller: CameraController::new(0.2),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.graphic.resize(new_size)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {
        let since_last_update = self.last_update.elapsed();
        // Do stuff...
        self.camera_controller.update_camera(&mut self.camera);
        self.graphic.update(&self.camera, since_last_update);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.graphic.render()
    }
    #[inline]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.graphic.window_bundle.size
    }
}
