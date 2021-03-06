use super::*;
use crate::camera::Camera;
use nalgebra_glm as glm;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self::new()
    }
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: glm::Mat4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct UniformBundle {
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub uniforms: Uniforms,
}

impl UniformBundle {
    pub fn new(window_bundle: &WindowBundle, camera: &Camera) -> Self {
        use wgpu::{
            util::{BufferInitDescriptor, DeviceExt},
            BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType::Buffer,
            BufferBindingType::Uniform,
            BufferUsage, ShaderStage,
        };
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(camera);

        let uniform_slice = &[uniforms];
        let buffer_desc = BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(uniform_slice),
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        };
        let bind_group_layout_desc = &BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::VERTEX,
                ty: Buffer {
                    ty: Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        };

        let uniform_buffer = window_bundle.device.create_buffer_init(&buffer_desc);
        let uniform_bind_group_layout = window_bundle
            .device
            .create_bind_group_layout(bind_group_layout_desc);

        let bind_group_desc = &BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        };

        let uniform_bind_group = window_bundle.device.create_bind_group(bind_group_desc);
        Self {
            uniform_bind_group_layout,
            uniform_bind_group,
            uniform_buffer,
            uniforms,
        }
    }
}
