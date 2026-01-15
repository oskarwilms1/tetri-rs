use bevy::{
    ecs::{entity::Entity, event::Event, observer::On},
    prelude::*,
};

use crate::{
    board::{
        grid::Grid,
        grid_matrix::GridMatrix,
        tetrimino::{spawn_tetrimino, Tetrimino},
        tetrimino_square::TetriminoSquare,
    },
    game::game_state::{show_gameover, GameOverUI, GameState},
    plugins::{assets_plugin::TetriminoAssets, observers::shadow_update::UpdateShadow},
    scoreboard::scoreboard::{increment_score, Score, Scoreboard},
};
#[derive(Event)]
pub struct TetriminoPlaced;

pub fn handle_on_placed(
    _event: On<TetriminoPlaced>,
    mut game_state: ResMut<GameState>,
    game_ui_query: Query<&mut Visibility, With<GameOverUI>>,
    mut commands: Commands,
    parent_query: Query<Entity, With<Grid>>,
    matrix_query: Query<(Entity, &Children, &mut GridMatrix), With<GridMatrix>>,
    child_query: Query<(Entity, &Transform, &Children), With<Tetrimino>>,
    squares_query: Query<
        (Entity, &MeshMaterial2d<ColorMaterial>, &Transform),
        With<TetriminoSquare>,
    >,
    tetrimino_assets: ResMut<TetriminoAssets>,
    scoreboard_query: Query<(&mut Score, &mut Text2d), With<Scoreboard>>,
) {
    let (tetrimino, tetrimino_position, tetrimino_children) =
        child_query.single().expect("No tetrimino active");

    match update_matrix(
        &mut commands,
        matrix_query,
        tetrimino_position.translation,
        tetrimino_children,
        squares_query,
    ) {
        Ok(points) => {
            handle_respawn(commands, tetrimino, parent_query, &tetrimino_assets);
            increment_score(scoreboard_query, points);
        }
        Err(_) => {
            *game_state = GameState::GameOver;
            show_gameover(game_ui_query);
        }
    }
}

fn handle_respawn(
    mut commands: Commands,
    entity: Entity,
    parent_query: Query<Entity, With<Grid>>,
    tetrimino_assets: &ResMut<TetriminoAssets>,
) {
    let parent = parent_query.single().expect("Grid not found");
    commands.entity(entity).despawn();
    spawn_tetrimino(&mut commands, parent, tetrimino_assets);

    commands.trigger(UpdateShadow);
}
fn update_matrix(
    commands: &mut Commands,
    mut matrix_query: Query<(Entity, &Children, &mut GridMatrix), With<GridMatrix>>,
    tetrimino_position: Vec3,
    tetrimino_children: &Children,
    squares_query: Query<
        (Entity, &MeshMaterial2d<ColorMaterial>, &Transform),
        With<TetriminoSquare>,
    >,
) -> Result<u64, ()> {
    let (matrix_entity, matrix_children, mut matrix) =
        matrix_query.single_mut().expect("GridMatrix not found");

    matrix.place_tetrimino(tetrimino_position, tetrimino_children, squares_query);
    let full_rows = matrix.check_full_rows();
    let amount_emptied: u64 = matrix.empty_rows(full_rows);
    for child in matrix_children.iter() {
        commands.entity(child).despawn();
    }

    matrix.spawn_cells(commands, matrix_entity, 0.0);
    if matrix.clone().check_if_full() {
        return Err(());
    }
    Ok(amount_emptied)
}
