use std::{fs::File, io::Read, ptr::null, time::SystemTime};

use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
use bevy_flycam::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use geom::cube::CubeSphere;
use gpu::texture::Texture;
use render::{
    multimaterialgroup::MultiMaterialGroup,
    planetplugin::{planet_update_system, PlanetData, PlanetSpec, PlanetBundle, default_mesh, PlanetPlugin},
};
use renderdoc::{RenderDoc, V110};
use spacelab::{matcolormap::PlanetMaterials, material_gpu::generate_material_gpu};
use wgpu::{Features, PrimitiveTopology};

use crate::spacelab::{
    lutgen::CUBEMAP, lutgen_gpu::gpu_generate_latlut_inner, normal::gpu_generate_normal_inner,
};

pub mod geom;
pub mod gpu;
pub mod render;
pub mod spaceengineers;
pub mod spacelab;

fn load_json_file<T>(path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: T = serde_json::from_str(&contents)?;

    Ok(data)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planet_spec = PlanetSpec {
        name: "agaris".to_owned(),
        radius: 1.0,
        hill_params: [-0.03, 0.03],
    };

    let hm = [
        asset_server.load("hm/left.jpg") as Handle<Image>,
        asset_server.load("hm/back.jpg"),
        asset_server.load("hm/up.jpg"),
        asset_server.load("hm/down.jpg"),
        asset_server.load("hm/front.jpg"),
        asset_server.load("hm/right.jpg"),
    ];

    let planetMaterials = vec![
        StandardMaterial {
            base_color_texture: Some(asset_server.load("left.jpg")),
            normal_map_texture: Some(asset_server.load("left_normal.jpg")),
            ..default()
        },
        StandardMaterial {
            base_color_texture: Some(asset_server.load("back.jpg")),
            normal_map_texture: Some(asset_server.load("back_normal.jpg")),
            ..default()
        },
        StandardMaterial {
            base_color_texture: Some(asset_server.load("up.jpg")),
            normal_map_texture: Some(asset_server.load("up_normal.jpg")),
            ..default()
        },
        StandardMaterial {
            base_color_texture: Some(asset_server.load("down.jpg")),
            normal_map_texture: Some(asset_server.load("down_normal.jpg")),
            ..default()
        },
        StandardMaterial {
            base_color_texture: Some(asset_server.load("front.jpg")),
            normal_map_texture: Some(asset_server.load("front_normal.jpg")),
            ..default()
        },
        StandardMaterial {
            base_color_texture: Some(asset_server.load("right.jpg")),
            normal_map_texture: Some(asset_server.load("right_normal.jpg")),
            ..default()
        },
    ];

    let planetMeshes = [
        meshes.add(default_mesh()),
        meshes.add(default_mesh()),
        meshes.add(default_mesh()),
        meshes.add(default_mesh()),
        meshes.add(default_mesh()),
        meshes.add(default_mesh()),
    ];

    let planetData = PlanetData {
        initialized: false,
        mesh: planetMeshes.clone(),
        heightmap: hm,
    };

    let parent = commands
        .spawn(PlanetBundle{
            name: Name::new(format!("Planet: {}", planet_spec.name.clone())),
            spec: planet_spec,
            data: planetData,
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..Default::default()
        })
        .id();

    for (mindex, mesh) in planetMeshes.iter().enumerate() {
        let child = commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(planetMaterials[mindex].clone()),
                    ..Default::default()
                },
                //Wireframe,
            ))
            .id();
        commands.entity(parent).push_children(&[child]);
    }

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                features: Features::POLYGON_MODE_LINE,
                ..default()
            },
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WireframePlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        .add_plugin(PlanetPlugin)
        .add_startup_system(setup)
        .run();
}

// Old main from generator
fn gen_main() {
    let mut rd: Option<RenderDoc<V110>> = RenderDoc::new().ok();

    let gpu_device = futures::executor::block_on(gpu::gpu::open_default()).unwrap();
    if rd.is_some() {
        rd.as_mut().unwrap().start_frame_capture(null(), null());
    }
    // println!("Hello, world!");
    // let file = File::open("../assets/Ares/Planet Agaris.sbc").expect("Failed to open XML file");
    // let reader = BufReader::new(file);
    // // Deserialize XML data into Definitions struct
    // let definitions: Definitions = from_reader(reader).expect("Failed to deserialize XML data");

    // Process the definitions object as needed
    // println!("{:#?}", definitions);

    // let matfiles: MatFile = load_json_file("../luts/matfiles.json").unwrap();
    // println!("{:#?} matFiles", matfiles);
    // let matcoloravg: MatColorAverage = load_json_file("../luts/matcoloravg.json").unwrap();
    // println!("{:#?} matColorAvg", matcoloravg);
    let planet_definitions: PlanetMaterials = load_json_file("../luts/matcolormap.json").unwrap();

    let start = SystemTime::now();
    for face in CUBEMAP.iter() {
        let latlut =
            futures::executor::block_on(gpu_generate_latlut_inner(&gpu_device, face, 2048, 2048))
                .unwrap();
        let heightmap = Texture::from_file(
            &gpu_device,
            format!("../assets/Ares/PlanetDataFiles/Planet Agaris/{}.png", face).as_str(),
            wgpu::TextureFormat::R32Float,
            Some("HeightMap"),
        );
        let materialmap = Texture::from_file(
            &gpu_device,
            format!(
                "../assets/Ares/PlanetDataFiles/Planet Agaris/{}_mat.png",
                face
            )
            .as_str(),
            wgpu::TextureFormat::Rgba8Unorm,
            Some("MaterialMap"),
        );

        let normal =
            futures::executor::block_on(gpu_generate_normal_inner(&gpu_device, &heightmap))
                .unwrap();

        futures::executor::block_on(
            normal.save_to_file(&gpu_device, format!("{}_normal.jpg", face).as_str()),
        );

        let material = futures::executor::block_on(generate_material_gpu(
            &gpu_device,
            &materialmap,
            &heightmap,
            &latlut,
            &normal,
            &planet_definitions.0["Planet Agaris"],
        ));
        futures::executor::block_on(
            material
                .unwrap()
                .save_to_file(&gpu_device, format!("{}.jpg", face).as_str()),
        );
    }
    let delta = SystemTime::now().duration_since(start).unwrap();

    println!(
        "GPU Compute Time elapsed: {}.{:03} seconds",
        delta.as_secs(),
        delta.subsec_millis()
    );
    if rd.is_some() {
        rd.unwrap().end_frame_capture(null(), null());
    }
}
