use super::WindowBundle;
use winit::window::Window;

pub struct GraphicBundle {
    pub window_bundle: WindowBundle,
}

impl GraphicBundle {
    pub async fn new(window: &Window) -> Self {
        let window_bundle = WindowBundle::new(&window).await;
        Self { window_bundle }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let win = &mut self.window_bundle;
        win.size = new_size;
        win.sc_desc.width = new_size.width;
        win.sc_desc.height = new_size.height;
        win.swap_chain = win.device.create_swap_chain(&win.surface, &win.sc_desc);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let win = &mut self.window_bundle;
        let frame = win.swap_chain.get_current_frame()?.output;
        // A lists of command to execute (like Renderpass)
        let mut encoder = win
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Contains all the method used for drawing
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        drop(render_pass);
        win.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    #[inline]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_bundle.size
    }
}