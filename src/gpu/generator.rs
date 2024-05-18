use std::{iter, mem::size_of, num::NonZeroU64};

use wgpu::{include_wgsl, util::DeviceExt};

use super::{
    command::{Command, CommandList},
    FontData, GpuGlyphData, LineSize, Text, Vertex,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct IndexData {
    vertices: u32,
    indices: u32,
}

pub struct GenerationPass {
    pub font_data: wgpu::Buffer,
    pub num_indices: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::ComputePipeline,
    pub lines: u32,
    pub line_length: u32,
    pub num_indices_reader: wgpu::Buffer,
}

impl GenerationPass {
    pub fn new(
        device: &wgpu::Device,
        text: &Text,
        glyph_data: &wgpu::Buffer,
        layout_buffer: &wgpu::Buffer,
    ) -> Self {
        let visibility = wgpu::ShaderStages::COMPUTE;
        // text, size, glyph data, layout, font data, vertex, index, num indices
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Generation Pass Bind Group Layout"),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            NonZeroU64::new(size_of::<FontData>() as u64).unwrap(),
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            NonZeroU64::new(size_of::<Vertex>() as u64).unwrap(),
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(NonZeroU64::new(size_of::<u32>() as u64).unwrap()),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 7,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            NonZeroU64::new(size_of::<IndexData>() as u64).unwrap(),
                        ),
                    },
                    count: None,
                },
            ],
        });
        let font_data = FontData {
            line_height: text.line_height,
        };
        let font_data = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Font Data"),
            contents: bytemuck::cast_slice(&[font_data]),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (4 * text.lines * text.line_length) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: (4 * text.lines * text.line_length) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });
        let num_indices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Num Indices Buffer"),
            size: size_of::<IndexData>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let num_indices_reader_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Num Indices Reader Buffer"),
            size: num_indices_buffer.size(),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Generation Pass Bind Group"),
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
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: font_data.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: num_indices_buffer.as_entire_binding(),
                },
            ],
        });
        let shader_module = device.create_shader_module(include_wgsl!("shaders/generator.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Generation Pass Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Generation Pass Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });

        Self {
            font_data,
            vertex_buffer,
            index_buffer,
            bind_group_layout,
            bind_group,
            pipeline,
            lines: text.lines,
            line_length: text.line_length,
            num_indices: num_indices_buffer,
            num_indices_reader: num_indices_reader_buffer,
        }
    }
    pub async fn get_num_indices(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> u32 {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Get Num Indices Encoder"),
        });
        encoder.copy_buffer_to_buffer(
            &self.num_indices,
            0,
            &self.num_indices_reader,
            0,
            self.num_indices.size(),
        );
        queue.submit(iter::once(encoder.finish()));
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        let buffer_slice = self.num_indices_reader.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();
        let data = buffer_slice.get_mapped_range();

        let index_data: &[IndexData] = bytemuck::cast_slice(&*data);

        index_data[0].indices
    }
}

impl Command for GenerationPass {
    // step 1. generate the coordinates of the vertices of each of the two triangles of each glyph
    // step 2. use an atomic bump allocator to put the vertices and indices into respective buffers
    fn push_buffers(&self, device: &wgpu::Device, commands: &mut CommandList) {
        let workgroups = (self.line_length / 64).max(1);
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Generation Pass Encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Generation Pass Compute Pass"),
                timestamp_writes: None,
            });
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.dispatch_workgroups(workgroups, self.lines, 1);
        }
        commands.push(encoder.finish());
    }
}
