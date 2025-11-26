use bevy::prelude::*;

#[derive(Bundle)]
pub struct Background {
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
}

impl Background {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        width: f32,
        heigth: f32,
    ) -> Self {
        Self {
            mesh: Mesh2d(meshes.add(Rectangle::new(width, heigth))),
            mesh_material: MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
        }
    }
}
