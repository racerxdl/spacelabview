use std::{borrow::Cow, mem};

use bevy::log;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::gpu::{self, gpu::Gpu, texture::Texture};

use super::matcolormap::{PlanetMaterial, ORE_COLORS};

const WORKGROUP_SIZE: (u32, u32) = (8, 8);

#[derive(Copy, Clone, Pod, Zeroable, Default)]
#[repr(C)]
struct GPUMaterialRule {
    pub id: u32,
    pad0: [f32; 3], // vec4 is aligned at 16 byte boundary, so we
    pub color: [f32; 4],
    // All are min,max
    pub height: [f32; 2],
    pub latitude: [f32; 2],
    pub slope: [f32; 2],
    pad1: [f32; 2] // GPU Requires this to be 16 byte aligned, this gives exactly 64 bytes per entry
}

#[derive(Copy, Clone, Pod, Zeroable, Default)]
#[repr(C)]
struct GPUOreMap {
    pub id: u32,
    pad0: [f32; 3], // vec4 is aligned at 16 byte boundary, so we
    pub color : [f32; 4],
}

impl PlanetMaterial {
    // Generate a list of the complex materials for the GPU.
    fn complex_materials_to_gpu(&self) -> Vec<GPUMaterialRule> {
        let mut gpu_materials = Vec::new();
        for (_, material) in &self.complex_materials {
            for rule in &material.rules {
                gpu_materials.push(GPUMaterialRule {
                    id: material.id as u32,
                    color: [
                        (rule.layers[0].r as f32) / 255.0,
                        (rule.layers[0].g as f32) / 255.0,
                        (rule.layers[0].b as f32) / 255.0,
                        1.0,
                    ],
                    height: [rule.min_height, rule.max_height],
                    latitude: [rule.latitude_min, rule.latitude_max],
                    slope: [rule.slope_min, rule.slope_max],
                    ..GPUMaterialRule::default()
                });
            }
        }
        if gpu_materials.len() == 0 {
            // Add dummy so it doesn't break shaders
            gpu_materials.push(GPUMaterialRule {
                id: 999, // Materials IDs are only up to 255
                color: [0.0, 0.0, 0.0, 0.0],
                height: [0.0, 0.0],
                latitude: [0.0, 0.0],
                slope: [0.0, 0.0],
                ..GPUMaterialRule::default()
            });
        }

        gpu_materials
    }

    fn simple_materials_to_gpu(&self) -> Vec<GPUMaterialRule> {
        let mut gpu_materials = Vec::new();
        for (id, material) in &self.simple_materials {
            gpu_materials.push(GPUMaterialRule {
                id: id.parse::<u32>().unwrap(),
                color: [
                    (material.r as f32) / 255.0,
                    (material.g as f32) / 255.0,
                    (material.b as f32) / 255.0,
                    1.0,
                ],
                // Simple materials doesnt use this
                height: [0.0, 0.0],
                latitude: [0.0, 0.0],
                slope: [0.0, 0.0],
                ..GPUMaterialRule::default()
            });
        }
        if gpu_materials.len() == 0 {
            // Add dummy so it doesn't break shaders
            gpu_materials.push(GPUMaterialRule {
                id: 999, // Materials IDs are only up to 255
                color: [0.0, 0.0, 0.0, 0.0],
                height: [0.0, 0.0],
                latitude: [0.0, 0.0],
                slope: [1.0, 1.0],
                ..GPUMaterialRule::default()
            });
        }
        gpu_materials
    }

    fn default_material_to_gpu(&self) -> Vec<GPUMaterialRule> {
        let mut gpu_materials = Vec::new();
        let material = &self.default_material;
        gpu_materials.push(GPUMaterialRule {
            id: 0,
            color: [
                (material.r as f32) / 255.0,
                (material.g as f32) / 255.0,
                (material.b as f32) / 255.0,
                1.0,
            ],
            // Simple materials doesnt use this
            height: [0.0, 0.0],
            latitude: [0.0, 0.0],
            slope: [0.0, 0.0],
            ..GPUMaterialRule::default()
        });
        gpu_materials
    }

    fn ore_map_to_gpu(&self) -> Vec<GPUOreMap> {
        let mut gpu_ores = Vec::new();

        for (id, ore) in &self.ores {
            let ore_name = ore.ore_type.clone().unwrap();
            if !ORE_COLORS.contains_key(&ore_name) {
                println!("Ore {} has no color mapping", ore_name);
                continue;
            }

            let color = ORE_COLORS.get(&ore_name).unwrap();
            gpu_ores.push(GPUOreMap {
                id: ore.value.unwrap() as u32,
                color: [
                    (color.0 as f32) / 255.0,
                    (color.1 as f32) / 255.0,
                    (color.2 as f32) / 255.0,
                    1.0,
                ],
                ..GPUOreMap::default()
            });
        }
        gpu_ores
    }
}

#[derive(Debug)]
struct MaterialRuleData {
    param_buf: wgpu::Buffer,
}

impl MaterialRuleData {
    pub fn new(device: &wgpu::Device, name: &str, data: Vec<GPUMaterialRule>) -> Self {
        let param_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            contents: bytemuck::cast_slice(data.as_ref()),
            usage: wgpu::BufferUsages::STORAGE,
        });

        MaterialRuleData {
            param_buf,
        }
    }

    pub fn binding_resource(&self) -> wgpu::BindingResource {
        self.param_buf.as_entire_binding()
    }

    pub fn binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: wgpu::BufferSize::new(mem::size_of::<GPUMaterialRule>() as _),
        }
    }
}


#[derive(Debug)]
struct OreMapData {
    param_buf: wgpu::Buffer,
}

impl OreMapData {
    pub fn new(device: &wgpu::Device, name: &str, data: Vec<GPUOreMap>) -> Self {
        let param_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            contents: bytemuck::cast_slice(data.as_ref()),
            usage: wgpu::BufferUsages::STORAGE,
        });

        OreMapData {
            param_buf,
        }
    }

    pub fn binding_resource(&self) -> wgpu::BindingResource {
        self.param_buf.as_entire_binding()
    }

    pub fn binding_type(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: wgpu::BufferSize::new(mem::size_of::<GPUOreMap>() as _),
        }
    }
}


pub async fn generate_material_gpu(
    gpu_device: &Gpu,
    materialmap: &Texture,
    heightmap: &Texture,
    latlut: &Texture,
    normalmap: &Texture,
    slopemap: &Texture,
    materials: &PlanetMaterial,
) -> Option<Texture> {
    let device = &gpu_device.device;
    let queue = &gpu_device.queue;

    let width = heightmap.width();
    let height = heightmap.height();
    if latlut.width() != width
        || latlut.height() != height
        || normalmap.width() != width
        || normalmap.height() != height
        || materialmap.width() != width
        || materialmap.height() != height
        || slopemap.width() != width
        || slopemap.height() != height
    {
        panic!("LatLut, Heightmap, Slope Map and Material Map must be the same size!");
    }

    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("materialgen.wgsl"))),
    });

    let texture = Texture::new(
        gpu_device,
        width,
        height,
        wgpu::TextureFormat::Rgba8Unorm,
        Some("PlanetMaterial Generator Output"),
    );

    let complex_materials = MaterialRuleData::new(
        device,
        "ComplexMaterials",
        materials.complex_materials_to_gpu(),
    );
    let simple_materials =
        MaterialRuleData::new(device, "SimpleMaterials", materials.simple_materials_to_gpu());
    let default_materials =
        MaterialRuleData::new(device, "DefaultMaterials", materials.default_material_to_gpu());

    let ore_mapping = OreMapData::new(device, "OreMapping", materials.ore_map_to_gpu());

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("PlanetMaterial Generator Bindings"),
        entries: &[
            // Default Materials
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: default_materials.binding_type(),
                count: None,
            },
            // Simple Materials
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: simple_materials.binding_type(),
                count: None,
            },
            // Simple Materials
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: complex_materials.binding_type(),
                count: None,
            },
            // Ore Mapping
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: ore_mapping.binding_type(),
                count: None,
            },
            // Material Map
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    multisampled: false,
                },
                count: None,
            },
            // Height Map
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    multisampled: false,
                },
                count: None,
            },
            // LatLut
            wgpu::BindGroupLayoutEntry {
                binding: 6,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    multisampled: false,
                },
                count: None,
            },
            // normal
            wgpu::BindGroupLayoutEntry {
                binding: 7,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    multisampled: false,
                },
                count: None,
            },
            // slope
            wgpu::BindGroupLayoutEntry {
                binding: 8,
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
                binding: 9,
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
        label: Some("PlanetMaterial Generator pipeline layout"),
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
                resource: default_materials.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: simple_materials.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: complex_materials.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: ore_mapping.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: materialmap.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: heightmap.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 6,
                resource: latlut.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 7,
                resource: normalmap.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 8,
                resource: slopemap.binding_resource(),
            },
            wgpu::BindGroupEntry {
                binding: 9,
                resource: texture.binding_resource(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("PlanetMaterial Generator Encoder"),
    });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("PlanetMaterial Generator Pass"),
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("compute planet material");
        let xdim = width + WORKGROUP_SIZE.0 - 1;
        let xgroups = xdim / WORKGROUP_SIZE.0;
        let ydim = height + WORKGROUP_SIZE.1 - 1;
        let ygroups = ydim / WORKGROUP_SIZE.1;

        cpass.dispatch_workgroups(xgroups, ygroups, 1);
    }

    queue.submit(Some(encoder.finish()));

    Some(texture)
}
