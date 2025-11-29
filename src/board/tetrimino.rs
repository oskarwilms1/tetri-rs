use crate::config::tetrimino::static_cells::tetrimino_I_cells::TETRINIMO_I_CELLS;
use bevy::{
    color::Color,
    prelude::{ColorMaterial, Component, Vec3},
};
use std::collections::HashMap;

enum TetriminoVariant {
    I,
    //O,
    //T,
    //S,
    //Z,
    //J,
    //L,
}

#[derive(Component)]
pub struct TetriminoData {
    variant: TetriminoVariant,
    cells: HashMap<i32, [Vec3; 4]>,
    color: ColorMaterial,
}

impl TetriminoData {
    pub fn new(variant: TetriminoVariant) -> Self {
        match variant {
            TetriminoVariant::I => Self {
                variant,
                cells: TETRINIMO_I_CELLS.clone(),
                color: ColorMaterial::from_color(Color::BLACK),
            },
        }
    }
}

pub fn tetrimino(tetrimino_data: &TetriminoData) -> () {
    ()
}
