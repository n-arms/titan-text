use std::{iter, mem::size_of, num::NonZeroU64};

use super::{
    command::{Command, CommandList},
    Vertex,
};

pub struct RenderPass<'a, 'g, 's> {
    pub surface: &'s wgpu::Texture,
    pub vertex_buffer: &'g wgpu::Buffer,
    pub index_buffer: &'g wgpu::Buffer,
    pub atlas_texture: &'a wgpu::Texture,
    pub render_pipeline: wgpu::RenderPipeline,
    pub num_indices: u32,
}

const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];

impl<'a, 'g, 's> RenderPass<'a, 'g, 's> {
    pub fn new(
        device: &wgpu::Device,
        surface: &'s wgpu::Texture,
        vertex_buffer: &'g wgpu::Buffer,
        index_buffer: &'g wgpu::Buffer,
        atlas_texture: &'a wgpu::Texture,
        num_indices: u32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/render.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let vertex_desc = wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        };
        let render_pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline Descriptor"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[vertex_desc],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };
        let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);

        Self {
            vertex_buffer,
            index_buffer,
            atlas_texture,
            render_pipeline,
            surface,
            num_indices,
        }
    }
    pub fn render(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let view = self
            .surface
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Pass Encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass Descriptor"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.render_pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            println!("calculated num indicies to be {}", self.num_indices);
            pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        queue.submit(iter::once(encoder.finish()));
    }
}
