use std::{mem::size_of, num::NonZeroU64};

use wgpu::include_wgsl;

use super::{
    command::{Command, CommandList},
    GpuGlyphData, LineSize, Text,
};

pub struct LayoutPass {
    pub layout_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::ComputePipeline,
    pub lines: u32,
}

impl LayoutPass {
    pub fn new(device: &wgpu::Device, text: &Text, glyph_data: &wgpu::Buffer) -> Self {
        let visibility = wgpu::ShaderStages::COMPUTE;
        // text, size, glyph data, layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Layout Pass Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(NonZeroU64::new(size_of::<u32>() as u64).unwrap()),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            NonZeroU64::new(size_of::<LineSize>() as u64).unwrap(),
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            NonZeroU64::new(size_of::<GpuGlyphData>() as u64).unwrap(),
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(NonZeroU64::new(size_of::<f32>() as u64).unwrap()),
                    },
                    count: None,
                },
            ],
        });
        let layout_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Layout Buffer"),
            size: text.text.size(),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Layout Pass Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: text.text.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: text.size.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: glyph_data.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: layout_buffer.as_entire_binding(),
                },
            ],
        });
        let shader_module = device.create_shader_module(include_wgsl!("shaders/layout.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Layout Pass Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Layout Pass Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });

        Self {
            layout_buffer,
            bind_group_layout,
            bind_group,
            pipeline,
            lines: text.lines,
        }
    }
}

impl Command for LayoutPass {
    // step 1. create 2d buffer to store the starting position of each glyph
    // step 2. run an inclusive prefix sum, taking `advance_width` from `glyph_data` as the elements to scan with
    // step 3. pack it all into a compute shader
    fn push_buffers(&self, device: &wgpu::Device, commands: &mut CommandList) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Layout Pass Encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Layout Pass Compute Pass"),
                timestamp_writes: None,
            });
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.dispatch_workgroups(1, self.lines, 1);
        }
        commands.push(encoder.finish());
    }
}
