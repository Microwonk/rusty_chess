use bevy::{prelude::*};
use crate::consts::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..64 {
        create_square(&mut commands, i);
    }
}

fn create_square(
    commands: &mut Commands,
    index: i32,
) {
    let curr_square: Square = Square::new(index);

    commands.spawn(curr_square.to_sprite());
}

struct Square {
    row: i32,
    col: i32,
    color: SquareColor,
}

#[derive(Clone, Copy)]
enum SquareColor {
    White,
    Black,
}

impl Square {
    fn new(index: i32) -> Self {
        let row = index / BOARD_SIZE;
        let col = index % BOARD_SIZE;

        Square {
            row,
            col,
            color: Square::gen_col(row, col),
        }
    }

    fn gen_col(row: i32, col: i32) -> SquareColor {
        if (row + col) % 2 == 0 {
            SquareColor::White
        } else {
            SquareColor::Black
        }
    }

    fn to_sprite(&self) -> SpriteBundle {
        let translation = Vec3::new(
            (self.col - BOARD_SIZE / 2) as f32 * SQUARE_SIZE,
            (self.row - BOARD_SIZE / 2) as f32 * SQUARE_SIZE,
            0.0,
        );

        SpriteBundle {
            sprite: Sprite {
                color: self.color.into(),
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(translation),
            ..Default::default()
        }
    }
}

impl From<SquareColor> for Color {
    fn from(color: SquareColor) -> Self {
        match color {
            SquareColor::White => Color::rgba(0.25, 0.25, 0.25, 1.0),
            SquareColor::Black => Color::rgba(0.75, 0.75, 0.75, 1.0),
        }
    }
}