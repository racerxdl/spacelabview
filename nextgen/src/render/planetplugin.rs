use bevy::{
    asset::LoadState,
    prelude::{
        App, AssetServer, Assets, Bundle, Component, ComputedVisibility, GlobalTransform, Handle,
        Image, Mesh, Name, Plugin, Query, Res, ResMut, StandardMaterial, Transform, Vec2, Vec3,
        Visibility, Changed,
    },
    reflect::Reflect,
    render::{render_resource::Texture, texture::TextureFormatPixelInfo},
    time::Time, log,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use image::{DynamicImage, GenericImage, ImageBuffer, Luma, Rgb, Rgba};
use wgpu::PrimitiveTopology;

use crate::{geom::cube::CubeSphere, gpu::texture::image_from_bevy};

#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct PlanetSpec {
    pub name: String,
    pub radius: f32,
    pub hill_params: [f32; 2],
}

pub fn get_surface_filename(n: u32) -> String {
    match n {
        0 => "left.png".to_owned(),
        1 => "back.png".to_owned(),
        2 => "up.png".to_owned(),
        3 => "down.png".to_owned(),
        4 => "front.png".to_owned(),
        5 => "right.png".to_owned(),
        _ => panic!("Invalid face number"),
    }
}

impl PlanetSpec {
    pub fn new(name: String, radius: f32, hill_params: [f32; 2]) -> Self {
        PlanetSpec {
            name,
            radius,
            hill_params,
        }
    }

    pub fn get_material_filename(&self, n: u32) -> String {
        format!("{}/{}", self.name, get_surface_filename(n))
    }
    pub fn get_heightmap_filename(&self, n: u32) -> String {
        format!("{}/hm_{}", self.name, get_surface_filename(n))
    }
    pub fn get_normal_filename(&self, n: u32) -> String {
        format!("{}/normal_{}", self.name, get_surface_filename(n))
    }
}

#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct PlanetData {
    pub initialized: bool,
    pub mesh: [Handle<Mesh>; 6],
    pub heightmap: [Handle<Image>; 6],
}

#[derive(Bundle, Default, Debug, Clone, Reflect)]
pub struct PlanetBundle {
    pub name: Name,
    pub spec: PlanetSpec,
    pub data: PlanetData,

    pub visibility: Visibility,
    pub computed: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub fn default_mesh() -> Mesh {
    let mut m = Mesh::new(PrimitiveTopology::TriangleList);

    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![Vec3::new(0.0, 0.0, 0.0)]);
    m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::new(0.0, 0.0, 0.0)]);
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![Vec2::new(0.0, 0.0)]);
    m.set_indices(Some(bevy::render::mesh::Indices::U32(vec![0, 0, 0])));
    m
}

pub fn planet_update_system(
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    mut query: Query<(&PlanetSpec, &mut PlanetData)>,
) {
    for (spec, mut data) in query.iter_mut() {
        // Check if already initialized
        if data.initialized {
            continue;
        }

        // Check if all images are loaded
        if server.get_group_load_state(data.heightmap.iter().map(|x| x.id())) != LoadState::Loaded {
            continue;
        }

        // Convert bevy images to DynamicImages
        let hm = data
            .heightmap
            .iter()
            .map(|x| Some(image_from_bevy(images.get(x).unwrap())))
            .collect::<Vec<Option<DynamicImage>>>();

        // Create cube sphere to update meshes
        let mut cs = CubeSphere::new(spec.radius, 256);
        cs.set_heightmaps(hm);
        cs.set_hill_params(spec.hill_params[0], spec.hill_params[1]);

        for i in 0..6 {
            // Update
            let mesh = meshes.get_mut(&data.mesh[i]);
            if let Some(mesh) = mesh {
                cs.get_face_mesh(i).update_mesh(mesh);
            }
        }

        // Update meshes

        data.initialized = true;
    }
}

pub fn planet_spec_change(
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    mut query: Query<
        // Components
        (&PlanetSpec, &mut PlanetData),
        Changed<PlanetSpec>
    >,
) {
    for (spec, mut data) in query.iter_mut() {
        log::info!("Planet spec changed: {:?}", spec);
        // Check if all images are loaded
        if server.get_group_load_state(data.heightmap.iter().map(|x| x.id())) != LoadState::Loaded {
            continue;
        }

        // Convert bevy images to DynamicImages
        let hm = data
            .heightmap
            .iter()
            .map(|x| Some(image_from_bevy(images.get(x).unwrap())))
            .collect::<Vec<Option<DynamicImage>>>();

        // Create cube sphere to update meshes
        let mut cs = CubeSphere::new(spec.radius, 256);
        cs.set_heightmaps(hm);
        cs.set_hill_params(spec.hill_params[0], spec.hill_params[1]);

        for i in 0..6 {
            // Update
            let mesh = meshes.get_mut(&data.mesh[i]);
            if let Some(mesh) = mesh {
                cs.get_face_mesh(i).update_mesh(mesh);
            }
        }

        // Update meshes

        data.initialized = true;
    }
}

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlanetSpec>()
            .register_type::<PlanetData>()
            .add_system(planet_update_system)
            .add_system(planet_spec_change);
    }
}
