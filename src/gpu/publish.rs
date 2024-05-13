use std::mem::size_of;

use crate::preproc::AtlasView;

use super::GpuGlyphData;
pub fn create_atlas_texture(atlas: AtlasView, device: &wgpu::Device) -> wgpu::Texture {
    let size = wgpu::Extent3d {
        width: atlas.width,
        height: atlas.height,
        depth_or_array_layers: 1,
    };
    let texture_desc = wgpu::TextureDescriptor {
        label: Some("Atlas Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
    };
    device.create_texture(&texture_desc)
}

pub fn write_atlas_texture(atlas: AtlasView, texture: &wgpu::Texture, queue: &wgpu::Queue) {
    let bytes_per_pixel = 4 * size_of::<u8>() as u32;
    let bytes_per_row = bytes_per_pixel * atlas.width;
    let data_layout = wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(bytes_per_row),
        rows_per_image: Some(atlas.height),
    };
    let mut data = vec![0u8; (bytes_per_row * atlas.height) as usize];
    let mut write_pixel = |x: u32, y: u32, color: [u8; 4]| {
        let index = bytes_per_pixel * x + bytes_per_row * y;

        for i in 0..3 {
            data[index as usize + i] = color[i];
        }
    };
    for glyph in atlas.entries.values() {
        for (i, color) in glyph.glyph.image.data.chunks_exact(4).enumerate() {
            let local_x = i as u32 % atlas.width;
            let local_y = i as u32 / atlas.height;

            let x = glyph.x + local_x;
            let y = glyph.y + local_y;

            write_pixel(x, y, color.try_into().unwrap());
        }
    }
    queue.write_texture(texture.as_image_copy(), &data, data_layout, texture.size());
}

pub fn create_atlas_buffer(atlas: AtlasView, device: &wgpu::Device) -> wgpu::Buffer {
    let size = (size_of::<GpuGlyphData>() * atlas.entries.len()) as wgpu::BufferAddress;
    let buffer_desc = wgpu::BufferDescriptor {
        label: Some("Atlas Buffer"),
        size,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: false,
    };
    device.create_buffer(&buffer_desc)
}

pub fn write_atlas_buffer(atlas: AtlasView, buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
    let mut data = vec![GpuGlyphData::default(); atlas.entries.len()];
    for glyph in atlas.entries.values() {
        let placement = glyph.glyph.image.placement;
        let glyph_data = GpuGlyphData {
            texture_x: glyph.x,
            texture_y: glyph.y,
            width: placement.width,
            height: placement.height,
            left: placement.left,
            top: placement.top,
            advance_x: glyph.glyph.advance_width,
        };
        data[glyph.id as usize] = glyph_data;
    }
    queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&data));
}
