use bevy::{
    asset::Handle,
    camera::visibility::Visibility,
    color::Color,
    ecs::{bundle::Bundle, component::Component, query::With, resource::Resource, system::Query},
    text::{Font, Justify, TextFont, TextLayout},
    ui::{widget::Text, AlignItems, BackgroundColor, JustifyContent, Node, Val},
    utils::default,
};

use crate::plugins::startup_plugin::GameEntity;
#[derive(Resource, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
}
#[derive(Component)]
pub struct GameOverUI;

pub fn gameover_ui() -> impl Bundle {
    (
        GameOverUI,
        GameEntity,
        Visibility::Hidden,
        BackgroundColor(Color::linear_rgba(0., 0., 0., 0.5)),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}

pub fn gameover_text(font: Handle<Font>) -> impl Bundle {
    (
        Text::new("Game Over!\nPress R to restart"),
        TextFont {
            font,
            font_size: 60.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
    )
}

pub fn show_gameover(mut query: Query<&mut Visibility, With<GameOverUI>>) {
    let mut visibility = query.single_mut().unwrap();
    *visibility = Visibility::Visible;
}
pub mod game_state;
pub mod scoreboard;
use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, query::With, system::Query},
    sprite::Text2d,
    text::{Font, Justify, TextFont, TextLayout},
    transform::components::Transform,
    utils::default,
};

#[derive(Component)]
pub struct Scoreboard;
#[derive(Component)]
pub struct Score(pub u64);
#[derive(Bundle)]
pub struct ScoreboardBundle {
    entity: Scoreboard,
    score: Score,
    pub text: Text2d,
    text_font: TextFont,
    text_layout: TextLayout,
    position: Transform,
}
impl ScoreboardBundle {
    pub fn new(font: Handle<Font>, window_height: f32) -> Self {
        Self {
            entity: Scoreboard,
            score: Score(0),
            text: Text2d::new("0"),
            text_font: TextFont {
                font,
                font_size: 67.0,
                ..default()
            },
            text_layout: TextLayout::new_with_justify(Justify::Center),
            position: Transform::from_xyz(0., window_height, 1.),
        }
    }
}

pub fn increment_score(mut query: Query<(&mut Score, &mut Text2d), With<Scoreboard>>, points: u64) {
    let (mut score, mut text) = query.single_mut().unwrap();
    if points > 0 {
        score.0 += 100 * 2_u64.pow((points - 1_u64).try_into().unwrap());

        *text = Text2d::new(score.0.to_string());
    }
}
pub mod grid;
pub mod grid_matrix;
pub mod rotate_tetrimino;
pub mod tetrimino;
pub mod tetrimino_shadow;
pub mod tetrimino_square;
use bevy::{
    asset::Handle,
    camera::visibility::{InheritedVisibility, Visibility},
    ecs::{
        entity::Entity,
        hierarchy::Children,
        query::With,
        system::{Commands, Query},
    },
    math::Vec3,
    mesh::{Mesh, Mesh2d},
    prelude::Component,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    board::tetrimino_square::TetriminoSquare,
    config::grid::grid_config::{CELL_BORDER_THICKNESS, CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellState {
    Full,
    Empty,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    state: CellState,
    color: Handle<ColorMaterial>,
}

impl Cell {
    pub fn new(state: CellState, color: Handle<ColorMaterial>) -> Self {
        Self { state, color }
    }
    pub fn get_state(&self) -> &CellState {
        &self.state
    }
}

#[derive(Component, Debug, Clone)]
pub struct GridMatrix {
    width: usize,
    height: usize,
    mesh: Handle<Mesh>,
    color: Handle<ColorMaterial>,
    cells: Vec<Cell>,
}
impl GridMatrix {
    pub fn new(
        width: usize,
        height: usize,
        mesh: Handle<Mesh>,
        color: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            width,
            height,
            color: color.clone(),
            mesh,
            cells: vec![Cell::new(CellState::Empty, color); width * height],
        }
    }
    pub fn spawn_cells(&self, commands: &mut Commands, parent_entity: Entity, z_offset: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y).unwrap();
                let cell = &self.cells[index];

                commands.entity(parent_entity).with_children(|parent| {
                    parent.spawn((
                        Mesh2d(self.mesh.clone()),
                        MeshMaterial2d(cell.color.clone()),
                        Transform::from_xyz(
                            x as f32 * CELL_SIZE,
                            -(y as f32 * CELL_SIZE),
                            z_offset,
                        )
                        .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
                        Visibility::default(),
                        InheritedVisibility::default(),
                    ));
                });
            }
        }
    }
    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
    pub fn get_cell_state(&self, x: usize, y: usize) -> Option<&CellState> {
        if let Some(index) = self.index(x, y) {
            return Some(self.cells[index].get_state());
        }
        None
    }

    pub fn fill_cell(&mut self, x: usize, y: usize, color: Handle<ColorMaterial>) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = Cell::new(CellState::Full, color);
        }
    }
    pub fn empty_cell(&mut self, x: usize, y: usize) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = Cell::new(CellState::Empty, self.color.clone());
        }
    }

    pub fn move_down_cell(&mut self, x: usize, y: usize) {
        if let Some((higher_index, lower_index)) = self.index(x, y).zip(self.index(x, y + 1)) {
            self.cells.swap(higher_index, lower_index);
        }
    }
    pub fn place_tetrimino(
        &mut self,
        parent_translation: Vec3,
        children: &Children,
        squares_query: Query<
            (Entity, &MeshMaterial2d<ColorMaterial>, &Transform),
            With<TetriminoSquare>,
        >,
    ) {
        for child in children.iter() {
            if let Ok((_square_entity, color, transform)) = squares_query.get(*child) {
                let x_index =
                    ((parent_translation.x + transform.translation.x) / CELL_SIZE) as usize;
                let y_index =
                    ((parent_translation.y + transform.translation.y).abs() / CELL_SIZE) as usize;
                self.fill_cell(x_index, y_index, color.0.clone());
            }
        }
    }
    pub fn check_if_full(self) -> bool {
        if let Some(index) = self.index(self.width - 1, 2) {
            return self.cells[0..=index]
                .iter()
                .any(|cell| *cell.get_state() == CellState::Full);
        }
        false
    }

    pub fn empty_rows(&mut self, rows: Option<Vec<usize>>) -> u64 {
        let mut counter: u64 = 0;
        if let Some(mut full_rows) = rows {
            full_rows.sort_unstable();
            for y_index in full_rows {
                counter += 1;
                for x in 0..self.width {
                    self.empty_cell(x, y_index);
                }
                for y in (0..y_index).rev() {
                    for x in 0..self.width {
                        self.move_down_cell(x, y);
                    }
                }
            }
        }
        counter
    }

    pub fn check_full_rows(&self) -> Option<Vec<usize>> {
        let mut full_rows: Vec<usize> = Vec::new();
        for y in 0..ROW_AMOUNT as usize {
            if self.cells[self.index(0, y).unwrap()
                ..=self.index(COLUMN_AMOUNT as usize - 1_usize, y).unwrap()]
                .iter()
                .all(|cell| *cell.get_state() == CellState::Full)
            {
                full_rows.push(y);
            }
        }
        if !full_rows.is_empty() {
            return Some(full_rows);
        }
        None
    }
}
use crate::{
    board::grid_matrix::GridMatrix,
    config::grid::grid_config::{COLUMN_AMOUNT, ROW_AMOUNT},
    plugins::startup_plugin::GameEntity,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;

pub fn spawn_grid(
    commands: &mut Commands,
    cell_mesh: &Handle<Mesh>,
    background_cell_material: &Handle<ColorMaterial>,
    x_offset: f32,
    y_offset: f32,
) -> Entity {
    let grid_entity = commands
        .spawn((
            Grid,
            GameEntity,
            Transform::from_xyz(x_offset, y_offset, 0.),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    let grid_matrix_data = GridMatrix::new(
        COLUMN_AMOUNT as usize,
        ROW_AMOUNT as usize,
        cell_mesh.clone(),
        background_cell_material.clone(),
    );

    let grid_matrix_entity = commands
        .spawn((
            grid_matrix_data.clone(),
            Transform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    commands.entity(grid_entity).add_child(grid_matrix_entity);

    grid_matrix_data.spawn_cells(commands, grid_matrix_entity, 0.0);

    grid_entity
}
use crate::board::tetrimino_square::spawn_tetrinimo_children;
use crate::board::tetrimino_square::TetriminoVariant;
use crate::config::grid::grid_config::CELL_SIZE;
use crate::config::tetrimino::variants::VARIANTS;
use crate::plugins::assets_plugin::TetriminoAssets;
use crate::plugins::startup_plugin::GameEntity;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Tetrimino;
#[derive(Bundle)]
pub struct TetriminoBundle {
    tetrimino: Tetrimino,
    transform: Transform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}

impl TetriminoBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            tetrimino: Tetrimino,
            transform: Transform::from_xyz(x, y, 1.),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
        }
    }
}

pub fn spawn_tetrimino(
    commands: &mut Commands,
    parent: Entity,
    tetrimino_assets: &ResMut<TetriminoAssets>,
) {
    let mut rng = rand::rng();
    let variant: &TetriminoVariant = &VARIANTS[rng.random_range(0..=6)];
    let mesh = &tetrimino_assets.cell_mesh;
    let material = &tetrimino_assets.material(*variant);
    let tetrimino = commands
        .spawn(tetrimino(variant, mesh, material, 3. * CELL_SIZE, 0.))
        .id();
    commands.entity(parent).add_child(tetrimino);
}

pub fn tetrimino(
    variant: &TetriminoVariant,
    cell_mesh: &Handle<Mesh>,
    cell_color: &Handle<ColorMaterial>,
    x: f32,
    y: f32,
) -> impl Bundle {
    (
        TetriminoBundle::new(x, y),
        GameEntity,
        spawn_tetrinimo_children(variant, cell_mesh, cell_color),
    )
}
use bevy::prelude::*;

use crate::config::{
    grid::grid_config::{CELL_BORDER_THICKNESS, CELL_SIZE},
    tetrimino::static_cells::{
        tetrimino_i_cells::TETRIMINO_I_CELLS, tetrimino_j_cells::TETRIMINO_J_CELLS,
        tetrimino_l_cells::TETRIMINO_L_CELLS, tetrimino_o_cells::TETRIMINO_O_CELLS,
        tetrimino_s_cells::TETRIMINO_S_CELLS, tetrimino_t_cells::TETRIMINO_T_CELLS,
        tetrimino_z_cells::TETRIMINO_Z_CELLS,
    },
};
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum TetriminoVariant {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
}
pub fn tetrimino_cell_data(variant: &TetriminoVariant) -> [[Vec3; 4]; 4] {
    match variant {
        TetriminoVariant::I => TETRIMINO_I_CELLS,
        TetriminoVariant::O => TETRIMINO_O_CELLS,
        TetriminoVariant::T => TETRIMINO_T_CELLS,
        TetriminoVariant::S => TETRIMINO_S_CELLS,
        TetriminoVariant::Z => TETRIMINO_Z_CELLS,
        TetriminoVariant::J => TETRIMINO_J_CELLS,
        TetriminoVariant::L => TETRIMINO_L_CELLS,
    }
}

#[derive(Component)]
pub struct TetriminoSquare {
    pub child_id: usize,
    rotation: usize,
    next_rotation: usize,
    cells: [[Vec3; 4]; 4],
}
impl TetriminoSquare {
    pub fn new(child_id: usize, cells: [[Vec3; 4]; 4]) -> Self {
        Self {
            child_id,
            rotation: 0,
            next_rotation: 1,
            cells,
        }
    }
    pub fn rotate(&mut self) {
        if self.rotation < 3 {
            self.rotation += 1;
        } else {
            self.rotation = 0;
        }
        if self.next_rotation < 3 {
            self.next_rotation += 1;
        } else {
            self.next_rotation = 0;
        }
    }
    pub fn get_rotation(&self) -> Transform {
        Transform::from_translation(self.cells[self.rotation][self.child_id] * CELL_SIZE)
            .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
    }
    pub fn get_next_rotation(&self) -> Transform {
        Transform::from_translation(self.cells[self.next_rotation][self.child_id] * CELL_SIZE)
            .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
    }
}

#[derive(Bundle)]
pub struct TetriminoSquareBundle {
    pub tetrimino_square: TetriminoSquare,
    pub cell_mesh: Mesh2d,
    cell_color: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

impl TetriminoSquareBundle {
    pub fn new(
        variant: &TetriminoVariant,
        cell_mesh: &Handle<Mesh>,
        cell_color: &Handle<ColorMaterial>,
        rotation: usize,
        child_id: usize,
    ) -> Self {
        let cells = tetrimino_cell_data(variant);
        let tetrimino_square = TetriminoSquare::new(child_id, cells);
        Self {
            tetrimino_square,
            cell_mesh: Mesh2d(cell_mesh.clone()),
            cell_color: MeshMaterial2d(cell_color.clone()),
            transform: Transform::from_translation(cells[rotation][child_id] * CELL_SIZE)
                .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
        }
    }
}

pub fn spawn_tetrinimo_children(
    variant: &TetriminoVariant,
    cell_mesh: &Handle<Mesh>,
    cell_color: &Handle<ColorMaterial>,
) -> impl Bundle {
    children![
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 0),
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 1),
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 2),
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 3),
    ]
}
use bevy::{
    camera::visibility::Visibility,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::Event,
        hierarchy::Children,
        query::{With, Without},
        system::{Commands, Query},
    },
    math::Vec3,
    transform::components::Transform,
};

use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    plugins::controls::collision::check_lowest_collision,
};
#[derive(Event)]
pub struct UpdateShadow;

#[derive(Component)]
pub struct TetriminoShadow;

pub fn tetrimino_shadow() -> impl Bundle {
    (TetriminoShadow, Visibility::Visible, Transform::default())
}

pub fn update_shadow(
    mut commands: Commands,
    mut shadow_query: Query<(Entity, &mut Transform), With<TetriminoShadow>>,
    tetrimino_query: Query<(Entity, &Transform), (With<Tetrimino>, Without<TetriminoShadow>)>,
    children_of: Query<&Children>,
    squares: Query<
        (&mut TetriminoSquare, &mut Transform),
        (Without<Tetrimino>, Without<TetriminoShadow>),
    >,
    grid_matrix: Query<&GridMatrix>,
) {
    let (shadow, mut shadow_transform) = shadow_query.single_mut().expect("Shadow not found");
    let (tetrimino, tetrimino_transform) = tetrimino_query.single().unwrap();
    let matrix = grid_matrix.single().unwrap();
    if let Ok(children) = children_of.get(tetrimino) {
        let child_positions: Vec<Vec3> = children
            .iter()
            .filter_map(|child| squares.get(*child).ok())
            .map(|(_, t)| t.translation)
            .collect();

        commands.entity(shadow).add_children(children);

        shadow_transform.translation = tetrimino_transform.translation;
        shadow_transform.translation.y -=
            check_lowest_collision(matrix, tetrimino_transform.translation, &child_positions);
    }
}
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};
mod board;
mod config;
mod game;
mod plugins;
mod scoreboard;

use crate::{
    game::game_state::GameState,
    plugins::{
        assets_plugin::AssetsPlugin, controls::controls_plugin::ControlsPlugin,
        gravity_plugin::GravityPlugin, observers_plugin::ObserversPlugin,
        startup_plugin::StartupPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetri-rs".into(),
                name: Some("Tetri-rs".into()),
                resolution: WindowResolution::new(450, 900).with_scale_factor_override(1.0),
                resizable: false,
                prevent_default_event_handling: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState::Playing)
        .add_plugins(AssetsPlugin)
        .add_plugins(ObserversPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(GravityPlugin)
        .run();
}
pub mod grid;
pub mod tetrimino;
pub mod static_cells;
pub mod variants;
use crate::board::tetrimino_square::TetriminoVariant;

pub const VARIANTS: [TetriminoVariant; 7] = [
    TetriminoVariant::I,
    TetriminoVariant::O,
    TetriminoVariant::T,
    TetriminoVariant::S,
    TetriminoVariant::Z,
    TetriminoVariant::J,
    TetriminoVariant::L,
];
use bevy::prelude::Vec3;

pub const TETRIMINO_S_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
    ],
    // rotation 1
    [
        Vec3::new(0., 0., 0.),
        Vec3::new(0., -1., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
    ],
    // rotation 2
    [
        Vec3::new(1., 0., 0.),
        Vec3::new(2., 0., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(0., -1., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., 0., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(2., -2., 0.),
    ],
];
pub mod tetrimino_i_cells;
pub mod tetrimino_j_cells;
pub mod tetrimino_l_cells;
pub mod tetrimino_o_cells;
pub mod tetrimino_s_cells;
pub mod tetrimino_t_cells;
pub mod tetrimino_z_cells;
use bevy::prelude::Vec3;

pub const TETRIMINO_J_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(0., -1., 0.),
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 1
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(1., -3., 0.),
    ],
    // rotation 2
    [
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
        Vec3::new(2., -3., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(1., -3., 0.),
        Vec3::new(0., -3., 0.),
    ],
];
use bevy::prelude::Vec3;

pub const TETRIMINO_T_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 1
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., -1., 0.),
        Vec3::new(0., -2., 0.),
    ],
    // rotation 2
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(2., 0., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., 0., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(2., -2., 0.),
    ],
];
use bevy::prelude::Vec3;

pub const TETRIMINO_L_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
        Vec3::new(2., -1., 0.),
    ],
    // rotation 1
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(1., -3., 0.),
        Vec3::new(2., -3., 0.),
    ],
    // rotation 2
    [
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
        Vec3::new(0., -3., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(1., -3., 0.),
        Vec3::new(0., -1., 0.),
    ],
];
use bevy::prelude::Vec3;

pub const TETRIMINO_Z_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(0., -1., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 1
    [
        Vec3::new(1., 0., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(0., -1., 0.),
        Vec3::new(0., -2., 0.),
    ],
    // rotation 2
    [
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
    ],
    // rotation 3
    [
        Vec3::new(2., 0., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
    ],
];
use bevy::prelude::Vec3;

pub const TETRIMINO_O_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 1
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 2
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
];
use bevy::prelude::Vec3;

pub const TETRIMINO_I_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(0., -1., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(3., -1., 0.),
    ],
    // rotation 1
    [
        Vec3::new(2., 0., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(2., -2., 0.),
        Vec3::new(2., -3., 0.),
    ],
    // rotation 2
    [
        Vec3::new(0., -2., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
        Vec3::new(3., -2., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., 0., 0.),
        Vec3::new(1., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(1., -3., 0.),
    ],
];
pub const CELL_SIZE: f32 = 30.0;
pub const CELL_BORDER_THICKNESS: f32 = 2.0;
pub const ROW_AMOUNT: f32 = 20.0;
pub const COLUMN_AMOUNT: f32 = 10.0;
pub mod grid_config;
#![allow(clippy::needless_pass_by_value)]

use bevy::{app::App, prelude::*};

use crate::plugins::observers::{
    handle_on_placed::handle_on_placed, handle_on_restart::handle_on_restart,
    on_update_shadow::on_update_shadow,
};

pub struct ObserversPlugin;

impl Plugin for ObserversPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_on_placed);
        app.add_observer(handle_on_restart);
        app.add_observer(on_update_shadow);
    }
}
pub mod assets_plugin;
pub mod controls;
pub mod gravity_plugin;
pub mod observers;
pub mod observers_plugin;
pub mod startup_plugin;
#![allow(clippy::needless_pass_by_value)]
use crate::board::tetrimino::spawn_tetrimino;
use crate::board::tetrimino_shadow::{tetrimino_shadow, UpdateShadow};
use crate::config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT};
use crate::game::game_state::{gameover_text, gameover_ui, GameState};
use crate::plugins::assets_plugin::{AssetLoading, BackgroundAssets, TetriminoAssets, UiFont};
use crate::scoreboard::scoreboard::ScoreboardBundle;
use bevy::prelude::*;

use crate::board::grid::spawn_grid;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup.after(AssetLoading));
    }
}
#[derive(Component)]
pub struct GameEntity;
pub fn game_setup(
    mut commands: Commands,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    ui_font: Res<UiFont>,
) {
    // Spawn centered 2D camera
    let font = ui_font.font.clone();
    commands.spawn((Camera2d, Transform::default(), GameEntity));
    let cell_mesh = &background_assets.cell_mesh;
    let background_material = &background_assets.background_material;

    // Center the grid in world space
    let grid_width = COLUMN_AMOUNT * CELL_SIZE;
    let grid_height = -ROW_AMOUNT * CELL_SIZE;

    let x_offset = -grid_width / 2.0 + CELL_SIZE / 2.0;
    let y_offset = -grid_height / 2.0;

    commands.spawn((
        ScoreboardBundle::new(font.clone(), -grid_height / 1.5),
        GameEntity,
    ));
    let grid = spawn_grid(
        &mut commands,
        cell_mesh,
        background_material,
        x_offset,
        y_offset,
    );

    spawn_tetrimino(&mut commands, grid, &tetrimino_assets);
    commands.spawn(gameover_ui()).with_children(|parent| {
        parent.spawn(gameover_text(font));
    });
    commands.spawn(tetrimino_shadow());
}

pub fn restart(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    query: Query<Entity, With<GameEntity>>,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    ui_font: Res<UiFont>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    game_setup(commands, background_assets, tetrimino_assets, ui_font);
    *game_state = GameState::Playing;
}
use bevy::{ math::Vec3};

use crate::{
    board::grid_matrix::{ CellState, GridMatrix},
    config::grid::grid_config::{CELL_SIZE, ROW_AMOUNT}
};

pub fn check_tetrimino_collision(
    matrix: &GridMatrix,
    parent_position: Vec3,
    child_positions: &[Vec3],
    x_offset: f32,
    y_offset: f32,
) -> bool {
    for block in child_positions {
        let x_index: usize = ((parent_position.x + block.x).abs() / CELL_SIZE + x_offset) as usize;
        let y_index: usize = ((parent_position.y + block.y).abs() / CELL_SIZE + y_offset) as usize;
        if y_index == ROW_AMOUNT as usize{
            return true
        }
        if let Some(cell_state) = matrix.get_cell_state(x_index, y_index) && *cell_state==CellState::Full {
            return true
        }
    }
    false
}

pub fn check_lowest_collision(    
    matrix: &GridMatrix,
    parent_position: Vec3,
    child_positions: &[Vec3],
    
    ) -> f32{
    let mut y_offset :f32 = 0.;
    loop{
        if check_tetrimino_collision(matrix, parent_position, child_positions, 0., y_offset+1.){
            return y_offset * CELL_SIZE;
        }
        else{
            y_offset+=1.
        }
    }
}
use bevy::prelude::*;

use crate::plugins::controls::handle_input::handle_input;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}
pub mod boundary_checks;
pub mod collision;
pub mod controls_plugin;
pub mod handle_input;
pub mod handle_movement;
pub mod handle_rotation;
#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    plugins::controls::{
        boundary_checks::corrected_translation_rotation, collision::check_tetrimino_collision,
    },
};
use bevy::prelude::*;

pub fn handle_rotate(
    grid_matrix: Query<&GridMatrix>,
    query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    mut squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
) {
    let (entity, mut transform) = query.into_inner();
    let children = children_of.get(entity).unwrap();
    let matrix = grid_matrix.single().unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.1.translation)
        .collect();

    let mut movement_vectors = Vec::new();

    for child in children.iter() {
        if let Ok((square, _transform)) = squares.get_mut(child) {
            movement_vectors.push(square.get_next_rotation().translation);
        }
    }
    if check_tetrimino_collision(matrix, transform.translation, &movement_vectors, 0., 0.) {
        return;
    }
    movement_vectors.clear();
    for child in children.iter() {
        if let Ok((mut square, mut transform)) = squares.get_mut(child) {
            let position = square.get_rotation();
            let next_position = square.get_next_rotation();
            let movement_vector = next_position.translation - position.translation;
            square.rotate();
            *transform = next_position;
            movement_vectors.push(movement_vector);
        }
    }
    transform.translation =
        corrected_translation_rotation(transform.translation, &child_positions, &movement_vectors);
}
use crate::config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT};
use bevy::prelude::Vec3;

pub fn corrected_translation(
    tetrimino_position: Vec3,
    children_positions: &[Vec3],
    movement_vector: &Vec3,
) -> Vec3 {
    let mut new_position: Vec3 = tetrimino_position + movement_vector;
    for child_position in children_positions {
        let adjusted_position = child_position + new_position;
        new_position.x += left_border_correction(adjusted_position.x);
        new_position.x += right_border_correction(adjusted_position.x);
        new_position.y += down_border_correction(adjusted_position.y);
    }

    new_position
}
pub fn corrected_translation_rotation(
    tetrimino_position: Vec3,
    children_positions: &[Vec3],
    movement_vectors: &[Vec3],
) -> Vec3 {
    let mut new_position: Vec3 = tetrimino_position;

    for (i, &movement_vector) in movement_vectors.iter().enumerate() {
        let child_position = children_positions[i];
        let adjusted_position = new_position + child_position + movement_vector;

        new_position.x += left_border_correction(adjusted_position.x);
        new_position.x += right_border_correction(adjusted_position.x);
        new_position.y += down_border_correction(adjusted_position.y);
    }

    new_position
}

fn left_border_correction(x: f32) -> f32 {
    if x < -CELL_SIZE {
        return 2. * CELL_SIZE;
    }
    if x < 0. {
        return CELL_SIZE;
    }
    0.
}
fn right_border_correction(x: f32) -> f32 {
    if x > CELL_SIZE * COLUMN_AMOUNT {
        return -2. * CELL_SIZE;
    }
    if x > CELL_SIZE * (COLUMN_AMOUNT - 1.) {
        return -CELL_SIZE;
    }
    0.
}
fn down_border_correction(y: f32) -> f32 {
    if y < -CELL_SIZE * ROW_AMOUNT {
        return 2. * CELL_SIZE;
    }
    if y < -CELL_SIZE * (ROW_AMOUNT - 1.) {
        return CELL_SIZE;
    }

    0.
}
#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::CELL_SIZE,
    plugins::{
        controls::{
            boundary_checks::corrected_translation,
            collision::{check_lowest_collision, check_tetrimino_collision},
            handle_input::Movement,
        },
        observers::handle_on_placed::TetriminoPlaced,
    },
};
use bevy::prelude::*;

pub fn handle_move(
    commands: &mut Commands,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    movement: Movement,
    grid_matrix: Query<&GridMatrix>,
) {
    let (entity, mut transform) = tetrimino_query.into_inner();
    let children = children_of.get(entity).unwrap();
    let matrix = grid_matrix.single().unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.1.translation)
        .collect();

    let movement_vec = match movement {
        Movement::Right => {
            if check_tetrimino_collision(matrix, transform.translation, &child_positions, 1., 0.) {
                return;
            }
            Vec3::new(CELL_SIZE, 0., 0.)
        }
        Movement::Left => {
            if check_tetrimino_collision(matrix, transform.translation, &child_positions, -1., 0.) {
                return;
            }
            Vec3::new(-CELL_SIZE, 0., 0.)
        }

        Movement::Down => {
            if check_tetrimino_collision(matrix, transform.translation, &child_positions, 0., 1.) {
                commands.trigger(TetriminoPlaced);
                return;
            }

            Vec3::new(0., -CELL_SIZE, 0.)
        }
        Movement::Space => {
            let new_y_value =
                check_lowest_collision(matrix, transform.translation, &child_positions);
            transform.translation.y -= new_y_value;
            commands.trigger(TetriminoPlaced);
            return;
        }
    };

    let new_position =
        corrected_translation(transform.translation, &child_positions, &movement_vec);

    transform.translation = new_position;
}
#![allow(clippy::needless_pass_by_value)]
use bevy::{
    ecs::{
        entity::Entity,
        hierarchy::Children,
        query::{With, Without},
        system::{Commands, Query, Res, Single},
    },
    input::{keyboard::KeyCode, ButtonInput},
    transform::components::Transform,
};

use crate::{
    board::{
        grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_shadow::UpdateShadow,
        tetrimino_square::TetriminoSquare,
    },
    game::game_state::GameState,
    plugins::{
        controls::{handle_movement::handle_move, handle_rotation::handle_rotate},
        observers::handle_on_restart::Restart,
    },
};

pub enum Movement {
    Left,
    Right,
    Down,
    Space,
}

pub fn handle_input(
    game_state: Res<GameState>,
    mut commands: Commands,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    grid_matrix: Query<&GridMatrix>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if *game_state == GameState::GameOver {
        if input.just_pressed(KeyCode::KeyR) {
            commands.trigger(Restart);
        }
        return;
    }
    let mut movement: Option<Movement> = None;

    if let Some(key) = input.get_just_pressed().next() {
        match key {
            KeyCode::KeyW => {
                handle_rotate(grid_matrix, tetrimino_query, children_of, squares);
                return;
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => movement = Some(Movement::Left),
            KeyCode::KeyD | KeyCode::ArrowRight => movement = Some(Movement::Right),
            KeyCode::KeyS | KeyCode::ArrowDown => movement = Some(Movement::Down),
            KeyCode::Space => movement = Some(Movement::Space),

            _ => {}
        }
    }

    if let Some(movement) = movement {
        handle_move(
            &mut commands,
            tetrimino_query,
            children_of,
            squares,
            movement,
            grid_matrix,
        );
    }
    commands.trigger(UpdateShadow);
}
use bevy::color::palettes::tailwind::{
    BLUE_500, CYAN_500, GREEN_500, ORANGE_500, PURPLE_500, RED_500, YELLOW_500,
};
use bevy::prelude::*;

use crate::board::tetrimino_square::TetriminoVariant;

#[derive(Resource)]
pub struct UiFont {
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct BackgroundAssets {
    pub cell_mesh: Handle<Mesh>,
    pub background_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct TetriminoAssets {
    pub cell_mesh: Handle<Mesh>,
    pub materials: [Handle<ColorMaterial>; 7],
}

impl TetriminoAssets {
    #[inline]
    pub fn material(&self, variant: TetriminoVariant) -> Handle<ColorMaterial> {
        self.materials[variant as usize].clone()
    }
}

pub struct AssetsPlugin;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct AssetLoading;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets.in_set(AssetLoading));
    }
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let cell_mesh = meshes.add(Rectangle::default());

    commands.insert_resource(BackgroundAssets {
        cell_mesh: cell_mesh.clone(),
        background_material: materials.add(ColorMaterial::default()),
    });

    commands.insert_resource(TetriminoAssets {
        cell_mesh,
        materials: [
            materials.add(ColorMaterial::from_color(CYAN_500)),
            materials.add(ColorMaterial::from_color(YELLOW_500)),
            materials.add(ColorMaterial::from_color(PURPLE_500)),
            materials.add(ColorMaterial::from_color(GREEN_500)),
            materials.add(ColorMaterial::from_color(RED_500)),
            materials.add(ColorMaterial::from_color(BLUE_500)),
            materials.add(ColorMaterial::from_color(ORANGE_500)),
        ],
    });
    let font = asset_server.load("fonts/BoldPixels.ttf");

    commands.insert_resource(UiFont { font });
}
#![allow(clippy::needless_pass_by_value)]
use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        hierarchy::Children,
        query::{With, Without},
        resource::Resource,
        system::{Commands, Query, Res, ResMut, Single},
    },
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};

use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    game::game_state::GameState,
    plugins::controls::{handle_input::Movement, handle_movement::handle_move},
};

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, gravity);
    }
}

#[derive(Resource)]
struct GravityTimer(Timer);

fn gravity(
    game_state: Res<GameState>,
    mut timer: ResMut<GravityTimer>,
    time: ResMut<Time>,
    mut commands: Commands,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    grid_matrix: Query<&GridMatrix>,
) {
    if *game_state == GameState::GameOver {
        return;
    }
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        handle_move(
            &mut commands,
            tetrimino_query,
            children_of,
            squares,
            Movement::Down,
            grid_matrix,
        );
    }
}
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
    plugins::assets_plugin::TetriminoAssets,
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
pub mod handle_on_placed;
pub mod handle_on_restart;
pub mod on_update_shadow;
use bevy::{
    ecs::{
        entity::Entity,
        hierarchy::Children,
        observer::On,
        query::{With, Without},
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::board::{
    grid_matrix::GridMatrix,
    tetrimino::Tetrimino,
    tetrimino_shadow::{update_shadow, TetriminoShadow, UpdateShadow},
    tetrimino_square::TetriminoSquare,
};

pub fn on_update_shadow(
    _event: On<UpdateShadow>,
    commands: Commands,
    shadow_query: Query<(Entity, &mut Transform), With<TetriminoShadow>>,
    tetrimino_query: Query<(Entity, &Transform), (With<Tetrimino>, Without<TetriminoShadow>)>,
    children_of: Query<&Children>,
    squares: Query<
        (&mut TetriminoSquare, &mut Transform),
        (Without<Tetrimino>, Without<TetriminoShadow>),
    >,
    grid_matrix: Query<&GridMatrix>,
) {
    update_shadow(
        commands,
        shadow_query,
        tetrimino_query,
        children_of,
        squares,
        grid_matrix,
    );
}
use bevy::ecs::{
    entity::Entity,
    event::Event,
    observer::On,
    query::With,
    system::{Commands, Query, Res, ResMut},
};

use crate::{
    game::game_state::GameState,
    plugins::{
        assets_plugin::{BackgroundAssets, TetriminoAssets, UiFont},
        startup_plugin::{restart, GameEntity},
    },
};

#[derive(Event)]
pub struct Restart;

pub fn handle_on_restart(
    _event: On<Restart>,
    commands: Commands,
    game_state: ResMut<GameState>,
    query: Query<Entity, With<GameEntity>>,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    ui_font: Res<UiFont>,
) {
    restart(
        commands,
        game_state,
        query,
        background_assets,
        tetrimino_assets,
        ui_font,
    );
}
