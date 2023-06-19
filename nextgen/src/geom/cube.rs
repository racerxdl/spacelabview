use std::ops::Mul;

use bevy::{
    log,
    prelude::{
        shape::{self, Cube},
        Component, Mesh, Vec2, Vec3,
    },
    render::mesh::Indices,
};
use image::GrayImage;
use wgpu::PrimitiveTopology;

use crate::render::sampler::sample_displacement;

use super::data::{MeshData, VertexData};

pub struct Box {
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
    pub segments_width: u32,
    pub segments_height: u32,
    pub segments_depth: u32,
}

const FACES: [&str; 6] = ["px", "nx", "py", "ny", "pz", "nz"];

impl Box {
    pub fn new(
        size_x: f32,
        size_y: f32,
        size_z: f32,
        segments_width: u32,
        segments_height: u32,
        segments_depth: u32,
    ) -> Box {
        Box {
            size_x,
            size_y,
            size_z,
            segments_width,
            segments_height,
            segments_depth,
        }
    }

    pub fn get_face_mesh(&self, n: usize) -> MeshData {
        let x = match n {
            0 | 1 => self.size_z,
            2 | 3 => self.size_x,
            4 | 5 => self.size_x,
            _ => {
                log::error!("Invalid face index: {}", n);
                0.0
            }
        };
        let y = match n {
            0 | 1 => self.size_y,
            2 | 3 => self.size_z,
            4 | 5 => self.size_y,
            _ => {
                log::error!("Invalid face index: {}", n);
                0.0
            }
        };
        let z = match n {
            0 => self.size_x,
            1 => -self.size_x,
            2 => self.size_y,
            3 => -self.size_y,
            4 => self.size_z,
            5 => -self.size_z,
            _ => {
                log::error!("Invalid face index: {}", n);
                0.0
            }
        };

        build_plane(
            FACES[n],
            x,
            y,
            z,
            self.segments_depth,
            self.segments_height,
            n,
        )
    }

    pub fn to_mesh_data(&self) -> Vec<MeshData> {
        let mut m: Vec<MeshData> = Vec::new();
        for i in 0..6 {
            m.push(self.get_face_mesh(i));
        }
        m
    }
}

fn build_plane(
    dir: &str,
    width: f32,
    height: f32,
    depth: f32,
    grid_x: u32,
    grid_y: u32,
    material_index: usize,
) -> (MeshData) {
    let mut m = MeshData::new();
    m.material_index = material_index;

    let segment_width = width / (grid_x as f32);
    let segment_height = height / (grid_y as f32);

    let width_half = width / 2.0;
    let height_half = height / 2.0;
    let depth_half = depth / 2.0;

    let grid_x1 = grid_x + 1;
    let grid_y1 = grid_y + 1;

    for iy in 0..grid_y1 {
        let y = (iy as f32) * segment_height - height_half;
        for xi in 0..grid_x1 {
            // Vertex
            let x = (xi as f32) * segment_width - width_half;
            let u = xi as f32 / grid_x as f32;
            let v = (iy as f32 / grid_y as f32);
            let n = if depth > 0.0 { 1.0 } else { -1.0 };

            let vdata = match dir {
                // 'z', 'y', 'x', - 1, - 1,
                "px" => VertexData::new(
                    Vec3::new(depth_half, -y, -x),
                    Vec3::new(n, 0.0, 0.0),
                    Vec2::new(u, v),
                ),

                // 'z', 'y', 'x', 1, - 1,
                "nx" => VertexData::new(
                    Vec3::new(depth_half, -y, x),
                    Vec3::new(n, 0.0, 0.0),
                    Vec2::new(u, v),
                ),

                // 'x', 'z', 'y', 1, 1,
                "py" => VertexData::new(
                    Vec3::new(x, depth_half, y),
                    Vec3::new(0.0, n, 0.0),
                    Vec2::new(u, v),
                ),

                // 'x', 'z', 'y', 1, - 1
                "ny" => VertexData::new(
                    Vec3::new(x, depth_half, -y),
                    Vec3::new(0.0, n, 0.0),
                    Vec2::new(u, v),
                ),

                // 'x', 'y', 'z', 1, - 1,
                "pz" => VertexData::new(
                    Vec3::new(x, -y, depth_half),
                    Vec3::new(0.0, 0.0, n),
                    Vec2::new(u, v),
                ),

                // 'x', 'y', 'z', - 1, - 1,
                "nz" => VertexData::new(
                    Vec3::new(-x, -y, depth_half),
                    Vec3::new(0.0, 0.0, n),
                    Vec2::new(u, v),
                ),

                _default => panic!("Invalid direction"),
            };
            m.add_vertex(vdata);

            if iy == grid_y || xi == grid_x {
                continue;
            }
            // Indices
            let a = xi + grid_x1 * iy;
            let b = xi + grid_x1 * (iy + 1);
            let c = (xi + 1) + grid_x1 * (iy + 1);
            let d = (xi + 1) + grid_x1 * iy;

            // faces
            m.add_index(a);
            m.add_index(b);
            m.add_index(d);

            m.add_index(b);
            m.add_index(c);
            m.add_index(d);
        }
    }
    m
}

#[derive(Debug, Clone, Component)]
pub struct CubeSphere {
    pub radius: f32,
    pub subdivisions: u32,
    pub hill_params: [f32; 2],
    heightmaps: [Option<image::DynamicImage>; 6],
}

impl CubeSphere {
    pub fn new(radius: f32, subdivisions: u32) -> Self {
        CubeSphere {
            radius,
            subdivisions,
            heightmaps: [None, None, None, None, None, None],
            hill_params: [0.0, 0.0],
        }
    }

    pub fn set_hill_params(&mut self, min: f32, max: f32) {
        self.hill_params = [min, max];
    }

    pub fn set_heightmaps_from_result(
        &mut self,
        hm: [Result<image::DynamicImage, image::ImageError>; 6],
    ) {
        for (i, h) in hm.iter().enumerate() {
            match h {
                Ok(h) => {
                    self.heightmaps[i] = Some(h.clone());
                }
                Err(e) => {
                    log::error!("Error loading heightmap: {}", e);
                }
            }
        }
    }
    pub fn set_heightmaps(&mut self, hm: Vec<Option<image::DynamicImage>>) {
        for (i, h) in hm.iter().enumerate() {
            if i >= 6 {
                break;
            }
            self.heightmaps[i] = h.clone();
        }
    }

    pub fn get_face_mesh(&self, n: usize) -> MeshData {
        let mut meshdata = Box::new(
            self.radius / 2.0,
            self.radius / 2.0,
            self.radius / 2.0,
            self.subdivisions,
            self.subdivisions,
            self.subdivisions,
        )
        .get_face_mesh(n);

        let vertices: &mut Vec<VertexData> = meshdata.vertices.as_mut();
        let mindex = meshdata.material_index;

        // Adjust the mesh to be a sphere
        for vertex in vertices.iter_mut() {
            vertex.position = vertex.position.normalize();
            vertex.normal = vertex.position;
        }

        let hill_delta = self.hill_params[1] - self.hill_params[0];

        if hill_delta != 0.0 {
            if let Some(hmd) = self.heightmaps[mindex].as_ref() {
                for vertex in vertices.iter_mut() {
                    let u = vertex.uv.x;
                    let v = vertex.uv.y;

                    let h = sample_displacement(hmd, u, v);
                    vertex.position.x *= self.radius * (1.0 - self.hill_params[0] + h * hill_delta);
                    vertex.position.y *= self.radius * (1.0 - self.hill_params[0] + h * hill_delta);
                    vertex.position.z *= self.radius * (1.0 - self.hill_params[0] + h * hill_delta);
                }
            }
        }

        meshdata
    }
}

impl Default for CubeSphere {
    fn default() -> Self {
        CubeSphere::new(1.0, 1)
    }
}

impl From<CubeSphere> for Vec<MeshData> {
    fn from(cube: CubeSphere) -> Self {
        (0..6).map(|n| cube.get_face_mesh(n)).collect()
    }
}
