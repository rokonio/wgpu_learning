// This file contain the graphic stuff for a program

use super::*;
use winit::window::Window;

pub struct GraphicBundle {
    pub window_bundle: WindowBundle,
    pub pipeline_bundle: RenderPipelineBundle,
    pub vertex_bundle: VertexBundle,
    pub texture_bundle: TextureBundle,
}

impl GraphicBundle {
    pub async fn new(window: &Window, vertices: &[Vertex], indices: &[u16]) -> Self {
        let window_bundle = WindowBundle::new(&window).await;
        let vertex_bundle = VertexBundle::new(&window_bundle, vertices, indices);
        let texture_bundle = TextureBundle::new(
            &window_bundle,
            include_bytes!("../../assets/happy-tree.png"),
        );

        let pipeline_bundle = RenderPipelineBundle::new(
            &window_bundle,
            &texture_bundle,
            include_str!("../shaders/shader.wgsl"),
        );
        Self {
            window_bundle,
            pipeline_bundle,
            vertex_bundle,
            texture_bundle,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_bundle.resize(new_size);
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
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        let vert = &self.vertex_bundle;
        render_pass.set_pipeline(&self.pipeline_bundle.render_pipeline);
        render_pass.set_bind_group(0, &self.texture_bundle.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vert.vertex_buffer.slice(..));
        render_pass.set_index_buffer(vert.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..vert.num_indices, 0, 0..1);

        drop(render_pass);
        win.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    #[inline]
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_bundle.size
    }
}
