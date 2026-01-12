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
use rand::seq::index;

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
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
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

    pub fn empty_rows(&mut self, rows: Option<Vec<usize>>) {
        if let Some(mut full_rows) = rows {
            full_rows.sort_unstable();

            for y_index in full_rows {
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
