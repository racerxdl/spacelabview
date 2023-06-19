use std::num::NonZeroU32;

use bevy::{prelude::Image, render::texture::TextureFormatPixelInfo};
use image::{GenericImageView, ImageBuffer, Luma, Rgba, Rgb, DynamicImage};

use super::gpu;

pub struct Texture {
    texture_view: wgpu::TextureView,
    format: wgpu::TextureFormat,
    texture: wgpu::Texture,
}

impl Texture {
    pub fn new(
        gpu_device: &gpu::Gpu,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        label: Option<&str>,
    ) -> Self {
        let texture = gpu_device.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Texture {
            texture_view,
            format,
            texture,
        }
    }

    pub fn from_file(
        gpu_device: &gpu::Gpu,
        path: &str,
        format: wgpu::TextureFormat,
        label: Option<&str>,
    ) -> Self {
        let img = image::open(path).unwrap();
        let dimensions = img.dimensions();
        let width = dimensions.0;
        let height = dimensions.1;

        let texture = gpu_device.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture = Texture {
            texture_view,
            format,
            texture,
        };
        match img {
            image::DynamicImage::ImageRgb8(img) => {
                let v = convert_rgb8_to_rgba8(&img, width, height);
                texture.upload_data(&gpu_device.queue, v.as_slice());
            }
            image::DynamicImage::ImageRgba8(img) => {
                texture.upload_data(&gpu_device.queue, img.as_raw());
            }
            image::DynamicImage::ImageLuma16(img) => {
                let v = convert_luma16_to_float(&img, width, height);
                texture.upload_float_data(&gpu_device.queue, v.as_slice());
            }
            _ => panic!("Unsupported texture format for file {}", path),
        };

        texture
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn upload_data(&self, queue: &wgpu::Queue, data: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * self.texture.size().width),
                rows_per_image: NonZeroU32::new(self.texture.size().height),
            },
            self.texture.size(),
        );
    }

    pub fn upload_float_data(&self, queue: &wgpu::Queue, data: &[f32]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(data),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * self.texture.size().width),
                rows_per_image: NonZeroU32::new(self.texture.size().height),
            },
            self.texture.size(),
        );
    }

    pub fn binding_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }

    pub fn binding_type(&self, access: wgpu::StorageTextureAccess) -> wgpu::BindingType {
        wgpu::BindingType::StorageTexture {
            access,
            format: self.format,
            view_dimension: wgpu::TextureViewDimension::D2,
        }
    }

    pub fn width(&self) -> u32 {
        self.texture.size().width
    }
    pub fn height(&self) -> u32 {
        self.texture.size().height
    }

    pub async fn save_to_file(&self, gpu_device: &gpu::Gpu, path: &str) {
        let width = self.width();
        let height = self.height();
        let u32_size = std::mem::size_of::<u32>() as u32;
        let output_buffer_size = (u32_size * width * height) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };

        let output_buffer = gpu_device.device.create_buffer(&output_buffer_desc);
        let tex = &self.texture();
        let texsize = tex.size();

        let mut encoder = gpu_device.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Texture Exporter"),
        });
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(u32_size * width),
                    rows_per_image: NonZeroU32::new(height),
                },
            },
            texsize,
        );
        gpu_device.queue.submit(Some(encoder.finish()));
        // We need to scope the mapping variables so that we can
        // unmap the buffer
        {
            let buffer_slice = output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            gpu_device.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, data).unwrap();
            buffer.save(path).unwrap();
        }
        output_buffer.unmap();
    }
}

fn convert_luma16_to_float(luma: &ImageBuffer<Luma<u16>, Vec<u16>>, width: u32, height: u32) -> Vec<f32> {
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    let mut min_val = 1000000.0;
    let mut max_val = -10000000.0;
    for pixel in luma.pixels() {
        let luma_value = (pixel[0] as f32) / 65535.0;
        if luma_value < min_val {
            min_val = luma_value;
        }
        if luma_value > max_val {
            max_val = luma_value;
        }
        rgba.push(luma_value);
        // let rgba_pixel = Rgba([luma_value, luma_value, luma_value, 1.0]);
        // rgba.extend_from_slice(&rgba_pixel.0);
    }
    println!("min: {}, max: {}", min_val, max_val);
    rgba
}

fn convert_rgb8_to_rgba8(rgb: &ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32) -> Vec<u8> {
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);

    for pixel in rgb.pixels() {
        let rgba_pixel = Rgba([pixel[0], pixel[1], pixel[2], 255]);
        rgba.extend_from_slice(&rgba_pixel.0);
    }
    rgba
}


pub fn image_from_bevy(bimg: &Image) -> DynamicImage { //
    let width = bimg.texture_descriptor.size.width;
    let height = bimg.texture_descriptor.size.height;
    let pixel_size = bimg.texture_descriptor.format.pixel_size();
    match pixel_size {
        4 => {
            let img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height,bimg.data.clone()).unwrap();
            DynamicImage::ImageRgba8(img)
        },
        3 => {
            let img : ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height,bimg.data.clone()).unwrap();
            DynamicImage::ImageRgb8(img)
        }
        2 => {
            let c = bimg.data.chunks_exact(2).map(|x| u16::from_ne_bytes([x[0], x[1]])).collect::<Vec<u16>>();
            let img : ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(width, height,c).unwrap();
            DynamicImage::ImageLuma16(img)
        }
        1 => {
            let img : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height,bimg.data.clone()).unwrap();
            DynamicImage::ImageLuma8(img)
        }
        _ => panic!("Invalid pixel size")
    }
}