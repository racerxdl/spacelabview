use std::{f32::consts::PI, fs, path::Path};

use image::{ImageBuffer, Rgb};
use nalgebra as na;

pub const CUBEMAP: [&str; 6] = ["front", "back", "down", "up", "left", "right"];
pub const RAD2DEG: f32 = 360.0 / (PI * 2.0);

pub fn pixel_to_latitude(
    face: &str,
    x_pixel: u32,
    y_pixel: u32,
    face_texture_width: u32,
    face_texture_height: u32,
) -> u8 {
    let u = (x_pixel as f32 + 0.5) / face_texture_width as f32 * 2.0 - 1.0;
    let v = (y_pixel as f32 + 0.5) / face_texture_height as f32 * 2.0 - 1.0;

    let point: na::Vector3<f32>;
    match face {
        "up" => point = na::Vector3::new(u, 1.0, -v),
        "down" => point = na::Vector3::new(u, -1.0, v),
        "left" => point = na::Vector3::new(-1.0, v, -u),
        "right" => point = na::Vector3::new(1.0, v, u),
        "back" => point = na::Vector3::new(-u, v, 1.0),
        "front" => point = na::Vector3::new(u, v, -1.0),
        _ => panic!("Invalid face"),
    }

    let point_on_sphere = point.normalize();
    let latitude = point_on_sphere.y.asin();
    let latitude_degrees = latitude.to_degrees();

    latitude_degrees as u8
}

pub fn generate_latlut(face: String, width: u32, height: u32) {
    let lat_lut_path = format!("luts/latlut_{}.png", face);
    if let Ok(_) = fs::metadata(&lat_lut_path) {
        println!("Cached LUT at {}", lat_lut_path);
        // Continue with the rest of your logic using latlutimg
    } else {
        //println!("Computing LUT");
        let mut latlutimg: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        for (x, y, pixel) in latlutimg.enumerate_pixels_mut() {
            let lat = pixel_to_latitude(face.as_str(), x, y, width, height);
            *pixel = Rgb([lat, lat, lat]);
        }
        latlutimg
            .save_with_format(&lat_lut_path, image::ImageFormat::Png)
            .unwrap();

        // println!("Done!");
    }
}

pub fn precompute_slope(im: &ImageBuffer<Rgb<u8>, Vec<u8>>, p: &str, base_asset_path: &str) {
    let width = im.width();
    let height = im.height();
    let slope_path = format!("{}{}_slope.png", base_asset_path, p);

    if Path::new(&slope_path).exists() {
        println!("Cached slope at {}", slope_path);
        return;
    }

    println!("Computing slope");
    let mut sm: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for y0 in 0..height {
        for x0 in 0..width {
            let x1 = (x0 + 1) % width;
            let y1 = (y0 + 1) % height;

            let z0 = im.get_pixel(x0, y0)[0] as i32;
            let z1 = im.get_pixel(x1, y1)[0] as i32;
            let delta_z = z1 - z0;
            let delta_x = x1 as f32 - x0 as f32;
            let delta_y = y1 as f32 - y0 as f32;
            let slope = (delta_z as f32
                / f32::sqrt(delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.pow(2) as f32))
            .asin()
                * RAD2DEG;
            let a = slope.abs() as u8;

            sm.put_pixel(x0, y0, Rgb([a, a, a]));
        }
    }

    sm.save_with_format(&slope_path, image::ImageFormat::Png)
        .unwrap();

    println!("Done!");
}

// On GPU
