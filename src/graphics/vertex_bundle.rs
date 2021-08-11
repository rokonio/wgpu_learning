use super::*;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct VertexBundle {
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
}

impl VertexBundle {
    pub fn new(window_bundle: &WindowBundle, vertices: &[Vertex]) -> Self {
        let vertex_buffer =
            window_bundle
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsage::VERTEX,
                });
        Self {
            vertex_buffer,
            num_vertices: vertices.len() as u32,
        }
    }
}

impl Vertex {
    const ATTR: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    pub const fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &Self::ATTR,
        }
    }
}
