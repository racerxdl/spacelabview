use bevy::prelude::{PbrBundle, Bundle, Material, Handle, StandardMaterial, Component, Resource, Mesh, Assets, ResMut, SpatialBundle, Visibility, ComputedVisibility, Transform, GlobalTransform};

use crate::geom::data::MeshData;


#[derive(Resource)]
pub struct MultiMaterialGroup {
    pub meshes: Vec<MeshData>,
    pub materials: Vec<StandardMaterial>
    /*

     */
}

#[derive(Bundle, Debug, Default)]
pub struct MultiMaterialGroupBundle {
    // SpatialBundle
    pub visibility: Visibility,
    pub computed: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl MultiMaterialGroup {
    pub fn new(materials: Vec<StandardMaterial>, meshes: Vec<MeshData>) -> Self {
        MultiMaterialGroup {
            meshes,
            materials
        }
    }

    // pub fn ToPBR(self,
    //     mut meshes: &ResMut<Assets<Mesh>>,
    //     mut materials: &ResMut<Assets<StandardMaterial>>,) -> Vec<PbrBundle> {

    //     let mut bundles = Vec::new();

    //     for mesh in self.meshes {
    //         let mindex = mesh.material_index;
    //         bundles.push(PbrBundle{
    //             mesh: meshes.as_mut().add(Mesh::from(mesh)),
    //             material: materials.add(self.materials[mindex].clone()),
    //             ..Default::default()
    //         })
    //     }

    //     bundles
    // }
}