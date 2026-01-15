use bevy::{
    asset::Handle,
    camera::visibility::{InheritedVisibility, Visibility},
    ecs::{
        bundle::Bundle,
        children,
        component::Component,
        entity::Entity,
        hierarchy::Children,
        query::{With, Without},
        system::Query,
    },
    math::Vec3,
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::{CELL_BORDER_THICKNESS, CELL_SIZE},
    plugins::{controls::collision::check_lowest_collision, startup_plugin::GameEntity},
};

#[derive(Component)]
pub struct TetriminoShadow;
#[derive(Component)]
pub struct ShadowSquare;
#[derive(Bundle)]
pub struct ShadowSquareBundle {
    id: ShadowSquare,
    transform: Transform,
    mesh: Mesh2d,
    color: MeshMaterial2d<ColorMaterial>,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}
impl ShadowSquareBundle {
    pub fn new(mesh: Handle<Mesh>, color: Handle<ColorMaterial>) -> Self {
        Self {
            id: ShadowSquare,
            transform: Transform::default()
                .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
            mesh: Mesh2d(mesh.clone()),
            color: MeshMaterial2d(color.clone()),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
        }
    }
}

pub fn tetrimino_shadow(mesh: Handle<Mesh>, color: Handle<ColorMaterial>) -> impl Bundle {
    (
        TetriminoShadow,
        GameEntity,
        Visibility::Visible,
        Transform::from_xyz(3. * CELL_SIZE, 0., 0.5),
        children![
            ShadowSquareBundle::new(mesh.clone(), color.clone()),
            ShadowSquareBundle::new(mesh.clone(), color.clone()),
            ShadowSquareBundle::new(mesh.clone(), color.clone()),
            ShadowSquareBundle::new(mesh.clone(), color.clone()),
        ],
    )
}
pub fn update_shadow(
    mut shadow_query: Query<(Entity, &mut Transform), With<TetriminoShadow>>,
    tetrimino_query: Query<(Entity, &mut Transform), (With<Tetrimino>, Without<TetriminoShadow>)>,
    square_query: Query<
        &mut Transform,
        (
            With<TetriminoSquare>,
            Without<Tetrimino>,
            Without<TetriminoShadow>,
        ),
    >,
    mut shadow_squares_query: Query<
        &mut Transform,
        (
            With<ShadowSquare>,
            Without<TetriminoSquare>,
            Without<TetriminoShadow>,
            Without<Tetrimino>,
        ),
    >,
    children_query: Query<&Children>,
    matrix_query: Query<&GridMatrix>,
) {
    let (shadow_entity, mut shadow_transform) = shadow_query.single_mut().unwrap();
    let (tetrimino_entity, tetrimino_transform) = tetrimino_query.single().unwrap();
    let matrix = matrix_query.single().unwrap();

    let tetrimino_children = children_query.get(tetrimino_entity).unwrap();
    let shadow_children = children_query.get(shadow_entity).unwrap();

    let child_positions: Vec<Vec3> = tetrimino_children
        .iter()
        .filter_map(|child| square_query.get(*child).ok())
        .map(|t| t.translation)
        .collect();

    for (position, shadow_child_entity) in child_positions.iter().zip(shadow_children.iter()) {
        if let Ok(mut shadow_transform) = shadow_squares_query.get_mut(*shadow_child_entity) {
            shadow_transform.translation = *position;
        }
    }

    shadow_transform.translation = tetrimino_transform.translation;

    shadow_transform.translation.y -=
        check_lowest_collision(matrix, tetrimino_transform.translation, &child_positions);
}
