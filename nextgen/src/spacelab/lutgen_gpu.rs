use std::{borrow::Cow, fs};

use bytemuck::{Pod, Zeroable};
use image::{ImageBuffer, Rgba};
use std::mem;
use wgpu::util::DeviceExt;

use crate::gpu::{gpu, texture::Texture};

const WORKGROUP_SIZE: (u32, u32) = (8, 8);

fn face_to_num(face: &str) -> u32 {
    match face {
        "front" => 0,
        "back" => 1,
        "down" => 2,
        "up" => 3,
        "left" => 4,
        "right" => 5,
        _ => panic!("Invalid face"),
    }
}

pub async fn gpu_generate_latlut(face: &str, width: u32, height: u32) -> Option<Texture> {
    let gpu = gpu::open_default().await?;

    gpu_generate_latlut_inner(&gpu, face, width, height).await
}

pub async fn gpu_generate_latlut_inner(
    gpu_device: &gpu::Gpu,
    face: &str,
    width: u32,
    height: u32,
) -> Option<Texture> {
    let device = &gpu_device.device;
    let queue = &gpu_device.queue;

    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("latlutgen.wgsl"))),
    });

    let gen_params = LatLutGenParams::new(device, face_to_num(face), width, height);
    let texture = Texture::new(
        gpu_device,
        width,
        height,
        wgpu::TextureFormat::Rgba8Unorm,
        Some("LatLutGen Output"),
    );

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("LatLutGen Bindings"),
        entries: &[
            // Face Number
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: gen_params.binding_type(),
                count: None,
            },
            // Output Texture
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    access: wgpu::StorageTextureAccess::WriteOnly,
                },
                count: None,
            },
        ],
    });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("latlutgen compute pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main",
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: gen_params.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: texture.binding_resource(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("LatLutGen Encoder"),
    });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("LatLutGen Pass"),
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("compute latlutgen");
        let xdim = width + WORKGROUP_SIZE.0 - 1;
        let xgroups = xdim / WORKGROUP_SIZE.0;
        let ydim = height + WORKGROUP_SIZE.1 - 1;
        let ygroups = ydim / WORKGROUP_SIZE.1;

        cpass.dispatch_workgroups(xgroups, ygroups, 1);
    }

    queue.submit(Some(encoder.finish()));


    Some(texture)
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct _LatLutGenParams {
    face_num: u32,
    width: u32,
    height: u32,
}

struct LatLutGenParams {
    param_buf: wgpu::Buffer,
}

impl LatLutGenParams {
    pub fn new(device: &wgpu::Device, face_num: u32, width: u32, height: u32) -> Self {
        let params = _LatLutGenParams {
            face_num: face_num,
            width: width,
            height: height,
        };
        let param_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("parameters buffer"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        LatLutGenParams { param_buf }
    }

    pub fn binding_resource(&self) -> wgpu::BindingResource {
        self.param_buf.as_entire_binding()
    }

    pub fn binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: wgpu::BufferSize::new(mem::size_of::<_LatLutGenParams>() as _),
        }
    }
}

// Slope Generation

pub async fn gpu_generate_slope_inner(gpu_device: &gpu::Gpu, heightmap: &Texture) -> Option<Texture> {
    let device = &gpu_device.device;
    let queue = &gpu_device.queue;
    let width = heightmap.width();
    let height = heightmap.height();
    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("slopegen.wgsl"))),
    });

    let texture = Texture::new(
        gpu_device,
        width,
        height,
        wgpu::TextureFormat::Rgba8Unorm,
        Some("SlopeGen Output"),
    );

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("SlopeGen Bindings"),
        entries: &[
            // Face Number
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    multisampled: false,
                },
                count: None,
            },
            // Output Texture
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    access: wgpu::StorageTextureAccess::WriteOnly,
                },
                count: None,
            },
        ],
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("slopegen compute pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main",
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: heightmap.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: texture.binding_resource(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("SlopeGen Encoder"),
    });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("SlopeGen Pass"),
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("compute slopegen");
        let xdim = width + WORKGROUP_SIZE.0 - 1;
        let xgroups = xdim / WORKGROUP_SIZE.0;
        let ydim = height + WORKGROUP_SIZE.1 - 1;
        let ygroups = ydim / WORKGROUP_SIZE.1;

        cpass.dispatch_workgroups(xgroups, ygroups, 1);
    }

    queue.submit(Some(encoder.finish()));

    Some(texture)
}
