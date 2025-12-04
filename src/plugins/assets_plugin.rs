use bevy::color::palettes::tailwind::{
    BLUE_500, CYAN_500, GREEN_500, ORANGE_500, PURPLE_500, RED_500, YELLOW_500,
};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub cell_mesh: Handle<Mesh>,
    pub background_material: Handle<ColorMaterial>,
    pub tetrimino_i_material: Handle<ColorMaterial>,
    pub tetrimino_o_material: Handle<ColorMaterial>,
    pub tetrimino_t_material: Handle<ColorMaterial>,
    pub tetrimino_s_material: Handle<ColorMaterial>,
    pub tetrimino_z_material: Handle<ColorMaterial>,
    pub tetrimino_j_material: Handle<ColorMaterial>,
    pub tetrimino_l_material: Handle<ColorMaterial>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default())
            .add_systems(Startup, load_assets);
    }
}

pub fn load_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tetrimino_assets: ResMut<GameAssets>,
) {
    tetrimino_assets.cell_mesh = meshes.add(Rectangle::default());
    tetrimino_assets.background_material = materials.add(ColorMaterial::default());

    tetrimino_assets.tetrimino_i_material =
        materials.add(ColorMaterial::from_color(Color::from(CYAN_500)));
    tetrimino_assets.tetrimino_o_material =
        materials.add(ColorMaterial::from_color(Color::from(YELLOW_500)));
    tetrimino_assets.tetrimino_t_material =
        materials.add(ColorMaterial::from_color(Color::from(PURPLE_500)));
    tetrimino_assets.tetrimino_s_material =
        materials.add(ColorMaterial::from_color(Color::from(GREEN_500)));
    tetrimino_assets.tetrimino_z_material =
        materials.add(ColorMaterial::from_color(Color::from(RED_500)));
    tetrimino_assets.tetrimino_j_material =
        materials.add(ColorMaterial::from_color(Color::from(BLUE_500)));
    tetrimino_assets.tetrimino_l_material =
        materials.add(ColorMaterial::from_color(Color::from(ORANGE_500)));
}
