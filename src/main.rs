mod font;
mod gpu;
mod preproc;

use std::{iter, mem::size_of, path::Path};

use anyhow::Result;
use gpu::{
    command::{Command, CommandList},
    generator::GenerationPass,
    layout::LayoutPass,
    publish::{
        create_atlas_buffer, create_atlas_texture, publish_text, write_atlas_buffer,
        write_atlas_texture,
    },
    render::RenderPass,
};
use image::RgbaImage;

fn main() -> Result<()> {
    pollster::block_on(run())
}

async fn run() -> Result<()> {
    let mut buf = Vec::new();
    let mut loader = font::Loader::system(&mut buf);
    let query = fontdb::Query {
        families: &[fontdb::Family::SansSerif],
        weight: fontdb::Weight::NORMAL,
        stretch: fontdb::Stretch::Normal,
        style: fontdb::Style::Normal,
    };
    let font = loader.load_font(&query)?;
    let atlas = preproc::Atlas::new(1024, 1024);
    let mut proc = preproc::Preprocessor::new(font, atlas, 12.);
    proc.add_str("Hello World!")?;

    let (device, queue) = load_gpu().await?;
    let atlas_texture = create_atlas_texture(proc.atlas.as_atlas_view(), &device);
    write_atlas_texture(proc.atlas.as_atlas_view(), &atlas_texture, &queue);
    let glyph_data_buffer = create_atlas_buffer(proc.atlas.as_atlas_view(), &device);
    write_atlas_buffer(proc.atlas.as_atlas_view(), &glyph_data_buffer, &queue);
    let text = publish_text(&proc.text, &device, &queue);

    let layout_pass = LayoutPass::new(&device, &text, &glyph_data_buffer);
    let generate_pass = GenerationPass::new(
        &device,
        &text,
        &glyph_data_buffer,
        &layout_pass.layout_buffer,
    );

    let mut commands = CommandList::default();
    layout_pass.push_buffers(&device, &mut commands);
    generate_pass.push_buffers(&device, &mut commands);
    commands.submit(&queue);

    let render_output = make_output_texture(&device);

    let render_pass = RenderPass::new(
        &device,
        &render_output,
        &generate_pass.vertex_buffer,
        &generate_pass.index_buffer,
        &atlas_texture,
        generate_pass.get_num_indices(&device, &queue).await,
    );

    render_pass.render(&device, &queue);

    save_output_texture(&render_output, &device, &queue, "output.png").await;

    Ok(())
}

async fn load_gpu() -> Result<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .unwrap();
    let (device, queue) = adapter.request_device(&Default::default(), None).await?;
    Ok((device, queue))
}

fn make_output_texture(device: &wgpu::Device) -> wgpu::Texture {
    let desc = wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    };
    device.create_texture(&desc)
}

async fn save_output_texture(
    texture: &wgpu::Texture,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    file: impl AsRef<Path>,
) {
    let buffer_size = (size_of::<u32>() * 256 * 256) as wgpu::BufferAddress;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Output Read Encoder"),
    });
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTextureBase {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some((size_of::<u32>() * 256) as u32),
                rows_per_image: Some(256),
            },
        },
        texture.size(),
    );
    queue.submit(iter::once(encoder.finish()));
    {
        let buffer_slice = buffer.slice(..);

        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| tx.send(result).unwrap());
        device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        let image = RgbaImage::from_raw(256, 256, (&*data).to_owned()).unwrap();
        image.save("image.png").unwrap();
    }
}
