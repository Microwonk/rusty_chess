use std::vec;

use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageHandle, ImageFileFormat, ImageSmoothingMode};
use speedy2d::shape::Rect;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use crate::consts::{NUM_FILES, NUM_RANKS, NUM_SQUARES};

const WINDOW_DIMENSION: u32 = 640;
const SQUARE_DIMENSION: usize = WINDOW_DIMENSION as usize / 8;
const TITLE: &str = "森 Rusty Chess";

pub fn init() {
    let window = Window::new_centered(TITLE, (WINDOW_DIMENSION, WINDOW_DIMENSION)).unwrap();

    window.run_loop(ChessWinHandler{
        board: BoardView::new(),
        components: vec![],
        selected_component: None,
        mouse_position: Vector2::ZERO,
    });
}

pub struct ChessWinHandler {
    board: BoardView,
    components: Vec<DraggableChessPiece>,
    selected_component: Option<usize>,
    mouse_position: Vector2<f32>,
}

impl WindowHandler for ChessWinHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        
        self.draw_chess_board(helper, graphics);


        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_mouse_button_down(
            &mut self,
            _helper: &mut WindowHelper<()>,
            _button: speedy2d::window::MouseButton
        ) {
            self.selected_component = self
                .components
                .iter()
                .position(|component| component.contains_point(self.mouse_position));

            if let Some(index) = self.selected_component {
                let selected_component = &mut self.components[index];
                selected_component.is_dragging = true;
                selected_component.offset = self.mouse_position - selected_component.position;
            }
        }

    fn on_mouse_button_up(
            &mut self,
            _helper: &mut WindowHelper<()>,
            _button: speedy2d::window::MouseButton
    ) {
        if let Some(index) = self.selected_component {
            self.components[index].is_dragging = false;
        }
        self.selected_component = None;
    }
    

    fn on_mouse_move( &mut self, _helper: &mut WindowHelper<()>, position: Vector2<f32>) {
        self.mouse_position = position;

        if let Some(index) = self.selected_component {
            let selected_component = &mut self.components[index];
            if selected_component.is_dragging {
                selected_component.position = position - selected_component.offset;
            }
        }   
    }
}

#[derive(Debug, PartialEq,  Clone)]
struct BoardView {
    squares: Vec<SquareView>,
}

impl BoardView {
    fn new() -> Self {
        let mut squares: Vec<SquareView> = vec![];

        let dark: Color = Color::from_int_rgb(155, 132, 75);
        let light: Color = Color::from_int_rgb(196, 189, 175);

        for row in 0..NUM_RANKS {
            for col in 0..NUM_FILES {
                let rect = Rect::from_tuples(
                    (
                        (col * SQUARE_DIMENSION) as f32,
                        (row * SQUARE_DIMENSION) as f32,
                    ),
                    (
                        ((col + 1) * SQUARE_DIMENSION) as f32,
                        ((row + 1) * SQUARE_DIMENSION) as f32,
                    ),
                );
                let color = if (row + col) % 2 == 0 { dark } else { light };
                squares.push(SquareView::new(rect, color, (row + col) as u8));
            }
        }

        Self {
            squares: squares,
        }
    }
}


#[derive(Debug, PartialEq,  Clone)]
struct SquareView {
    shape: Rect,
    color: Color,
    index: u8,
}

impl SquareView {
    fn new(shape: Rect, color: Color, index: u8) -> Self {
        Self {
            shape,
            color,
            index,
        }
    }
}

impl ChessWinHandler {

    fn draw_chess_board(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        for s in self.board.squares.iter() {
            graphics.draw_rectangle(s.shape.to_owned(), s.color)
        }
    }

    fn draw_pieces(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        for component in &self.components {
            graphics.draw_rectangle_image(
                Rect::new(component.position, component.size),
                &component.image,
            );
        }
    }
}

struct DraggableChessPiece {
    image: ImageHandle,
    position: Vector2<f32>,
    size: Vector2<f32>,
    is_dragging: bool,
    offset: Vector2<f32>,
    board_index: u8,
}

impl DraggableChessPiece {
    fn new(image: ImageHandle, position: Vector2<f32>, size: Vector2<f32>, board_index: u8) -> Self {
        Self {
            image,
            position,
            size,
            is_dragging: false,
            offset : Vector2::ZERO,
            board_index,
        }
    }

    fn contains_point(&self, point: Vector2<f32>) -> bool {
        let rect = Rect::new(self.position, self.size);
        rect.contains(point)
    }
}

