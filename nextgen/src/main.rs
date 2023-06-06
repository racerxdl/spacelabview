use std::{fs::File, io::Read, ptr::null, time::SystemTime};

use gpu::texture::Texture;
use renderdoc::{RenderDoc, V110, V141};
use spacelab::{
    lutgen_gpu::gpu_generate_slope_inner, matcolormap::PlanetMaterials,
    material_gpu::generate_material_gpu,
};

use crate::spacelab::{
    lutgen::{generate_latlut, CUBEMAP},
    lutgen_gpu::{gpu_generate_latlut, gpu_generate_latlut_inner},
};

pub mod gpu;
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

fn main() {
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
    let planetDefinitions: PlanetMaterials = load_json_file("../luts/matcolormap.json").unwrap();
    //println!("{:#?} planetDefinitions", planetDefinitions);

    // {
    //     let start = SystemTime::now();
    //     for face in CUBEMAP.iter() {
    //         generate_latlut(face.to_string(), 2048, 2048);
    //     }
    //     let delta = SystemTime::now().duration_since(start).unwrap();

    //     println!(
    //         "CPU Time elapsed: {}.{:03} seconds",
    //         delta.as_secs(),
    //         delta.subsec_millis()
    //     );
    // }
    // {
    //     let start = SystemTime::now();
    //         futures::executor::block_on(gpu_generate_latlut(face.to_string(), 2048, 2048));
    //     }

    // }

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

        let slope =
            futures::executor::block_on(gpu_generate_slope_inner(&gpu_device, &heightmap)).unwrap();
        let material = futures::executor::block_on(generate_material_gpu(
            &gpu_device,
            &materialmap,
            &heightmap,
            &latlut,
            &slope,
            &planetDefinitions.0["Planet Agaris"],
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
