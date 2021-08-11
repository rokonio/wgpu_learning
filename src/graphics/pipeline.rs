// This file contain the pipeline stuff for the graphic part of a program

use super::*;

pub struct RenderPipelineBundle {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipelineBundle {
    pub fn new(window_bundle: &WindowBundle, shader_src: &str) -> Self {
        let shader_desc = wgpu::ShaderModuleDescriptor {
            label: Some("Shaders"),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            flags: wgpu::ShaderFlags::all(),
        };
        let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            ..Default::default()
        };

        let shader = window_bundle.device.create_shader_module(&shader_desc);
        let pipeline_layout = window_bundle
            .device
            .create_pipeline_layout(&pipeline_layout_desc);

        let vertex = wgpu::VertexState {
            module: &shader,
            entry_point: "main",
            buffers: &[],
        };
        let fragment = wgpu::FragmentState {
            module: &shader,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: window_bundle.sc_desc.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        };

        use wgpu::{Face::*, FrontFace::*, PolygonMode::*, PrimitiveTopology::*};
        let primitive = wgpu::PrimitiveState {
            topology: TriangleList,
            strip_index_format: None,
            front_face: Ccw,
            cull_mode: Some(Back),
            clamp_depth: false,
            polygon_mode: Fill,
            conservative: false,
        };
        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let render_pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex,
            primitive,
            depth_stencil: None,
            multisample,
            fragment: Some(fragment),
        };
        Self {
            render_pipeline: window_bundle
                .device
                .create_render_pipeline(&render_pipeline_desc),
        }
    }
}
