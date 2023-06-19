use bevy::{prelude::{Vec3, Vec2, Mesh, Component}};
use bevy::reflect::{TypeUuid};
use wgpu::PrimitiveTopology;

#[derive(Debug, Clone, Default)]
pub struct VertexData {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl VertexData {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> VertexData {
        VertexData {
            position,
            normal,
            uv,
        }
    }
}

#[derive(Debug, TypeUuid, Clone)]
#[uuid = "00fc246a-eb82-49a0-826a-248421f1e88c"]
pub struct MeshData {
    pub vertices: Vec<VertexData>,
    pub indices: Vec<u32>,
    pub material_index: usize,
}

impl MeshData {
    pub fn new() -> MeshData {
        MeshData {
            vertices: Vec::new(),
            indices: Vec::new(),
            material_index: 0,
        }
    }

    pub fn add_vertex(&mut self, vertex: VertexData) {
        self.vertices.push(vertex);
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    pub fn update_mesh(&self, mesh: &mut Mesh) {
        let positions = self.vertices.iter().map(|v| v.position).collect::<Vec<_>>();
        let normals = self.vertices.iter().map(|v| v.normal).collect::<Vec<_>>();
        let uvs = self.vertices.iter().map(|v| v.uv).collect::<Vec<_>>();

        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);
        mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(self.indices.clone())));
    }
}

impl Default for MeshData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MeshData> for Mesh {
    fn from(m: MeshData) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let positions = m.vertices.iter().map(|v| v.position).collect::<Vec<_>>();
        let normals = m.vertices.iter().map(|v| v.normal).collect::<Vec<_>>();
        let uvs = m.vertices.iter().map(|v| v.uv).collect::<Vec<_>>();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(m.indices)));
        mesh
    }
}