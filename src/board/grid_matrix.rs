use bevy::{
    ecs::{entity::Entity, hierarchy::Children, query::With, system::Query},
    math::Vec3,
    prelude::Component,
    transform::components::Transform,
};

use crate::{
    board::tetrimino_square::TetriminoSquare,
    config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT},
};

#[derive(Component, Debug)]
pub struct GridMatrix {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Full,
    Empty,
}
impl GridMatrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Empty; width * height],
        }
    }
    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if let Some(index) = self.index(x, y) {
            return Some(&self.cells[index]);
        }
        None
    }
    //pub fn empty_cell(&mut self, x: usize, y: usize) {
    //    if let Some(index) = self.index(x, y) {
    //        self.cells[index] = Cell::Empty;
    //    }
    //}
    pub fn fill_cell(&mut self, x: usize, y: usize) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = Cell::Full;
        }
    }
    pub fn place_tetrimino(
        &mut self,
        parent_translation: Vec3,
        children: &Children,
        squares_query: Query<(Entity, &Transform), With<TetriminoSquare>>,
    ) {
        for child in children.iter() {
            if let Ok((_square_entity, transform)) = squares_query.get(*child) {
                let x_index =
                    ((parent_translation.x + transform.translation.x) / CELL_SIZE) as usize;
                let y_index =
                    ((parent_translation.y + transform.translation.y).abs() / CELL_SIZE) as usize;
                self.fill_cell(x_index, y_index);
            }
        }
    }
    pub fn empty_rows(&mut self, rows: Option<Vec<usize>>) {
        if let Some(mut full_rows) = rows {
            full_rows.sort_unstable_by(|a, b| b.cmp(a));

            for &y in &full_rows {
                for x in 0..self.width {
                    if let Some(index) = self.index(x, y) {
                        self.cells[index] = Cell::Empty;
                    }
                }

                for above_y in (0..y).rev() {
                    for x in 0..self.width {
                        let from_index = self.index(x, above_y).unwrap();
                        let to_index = self.index(x, above_y + 1).unwrap();
                        self.cells[to_index] = self.cells[from_index].clone();
                    }
                }

                for x in 0..self.width {
                    let top_index = self.index(x, 0).unwrap();
                    self.cells[top_index] = Cell::Empty;
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
                .all(|cell| *cell == Cell::Full)
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
