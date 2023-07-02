use std::vec;

use crate::chess::board::Board;
use crate::chess::consts::*;
use crate::chess::pieces::{Piece, PieceColor};
use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageHandle, ImageFileFormat, ImageSmoothingMode};
use speedy2d::shape::Rect;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

const WINDOW_DIMENSION: u32 = 640;
const SQUARE_DIMENSION: usize = WINDOW_DIMENSION as usize / 8;
const TITLE: &str = "森 Rusty Chess";
const B_B: &str = "src/sprites/b_black.png";
const W_B: &str = "src/sprites/b_white.png";
const B_K: &str = "src/sprites/k_black.png";
const W_K: &str = "src/sprites/k_white.png";
const B_N: &str = "src/sprites/n_black.png";
const W_N: &str = "src/sprites/n_white.png";
const B_P: &str = "src/sprites/p_black.png";
const W_P: &str = "src/sprites/p_white.png";
const B_Q: &str = "src/sprites/q_black.png";
const W_Q: &str = "src/sprites/q_white.png";
const B_R: &str = "src/sprites/r_black.png";
const W_R: &str = "src/sprites/r_white.png";

pub fn init() {
    let window = Window::new_centered(TITLE, (WINDOW_DIMENSION, WINDOW_DIMENSION)).unwrap();

    window.run_loop(ChessWinHandler{
        board: Board::default(),
        board_view: BoardView::new(),
        pieces: vec![],
        selected_component: None,
        mouse_position: Vector2::ZERO,
        is_button_down: false,
    });
}

pub struct ChessWinHandler {
    board: Board,
    board_view: BoardView,
    pieces: Vec<DraggableChessPiece>,
    selected_component: Option<usize>,
    mouse_position: Vector2<f32>,
    is_button_down: bool,
}

impl WindowHandler for ChessWinHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        
        self.draw_chess_board(helper, graphics);

        self.draw_pieces(helper, graphics);

        // Request that we draw another frame once this one has finished
    }

    fn on_start(
            &mut self,
            helper: &mut WindowHelper<()>,
            _info: speedy2d::window::WindowStartupInfo
        ) {

        self.read_board();
        helper.set_resizable(false);
        helper.request_redraw();
    }

    fn on_mouse_button_down(
            &mut self,
            helper: &mut WindowHelper<()>,
            _button: speedy2d::window::MouseButton
        ) {
            self.is_button_down = true;
            self.selected_component = self
                .pieces
                .iter()
                .position(|component| component.contains_point(self.mouse_position, &self));

            if let Some(index) = self.selected_component {
                let selected_component = &mut self.pieces[index];
                selected_component.is_dragging = true;
                selected_component.offset = self.mouse_position - selected_component.position;
            }
        }

    fn on_mouse_button_up(
            &mut self,
            helper: &mut WindowHelper<()>,
            _button: speedy2d::window::MouseButton
    ) {
        self.is_button_down = false;
        if let Some(index) = self.selected_component {

            //TODO: logic for behaviour ->

            let board_index = self.translate_mouse_pos_to_index()
            .unwrap_or(self.pieces[index].board_index); // returns to base index if an error occurs

            self.pieces[index].is_dragging = false;

            self.pieces[index].board_index = board_index;

            self.snap_piece(index, board_index);
        }
        self.selected_component = None;
    }
    

    fn on_mouse_move( &mut self, helper: &mut WindowHelper<()>, position: Vector2<f32>) {
        self.mouse_position = position;

        if let Some(index) = self.selected_component {
            let selected_component = &mut self.pieces[index];
            if selected_component.is_dragging {
                selected_component.position = position - selected_component.offset;
            }
        }   
        if self.is_button_down {
            helper.request_redraw();
        }
    }
}
impl ChessWinHandler {
    /// translates the position of the mouse in the window to a board index that can be used
    /// if it is not on the screen, it will give a None
    fn translate_mouse_pos_to_index(&self) -> Option<u8> {
        // i8s are used, so that the index isnt auto translated that we can detect off board circumstances
        let col = (self.mouse_position.x / SQUARE_DIMENSION as f32) as i8;
        let row = (self.mouse_position.y / SQUARE_DIMENSION as f32) as i8;

        if col < 8 && row < 8 && col >= 0 && row >= 0 {
            // Calculate the square index (assuming the board is 8x8)
            let square_index = row * 8 + col;
            Some(square_index as u8)
        } else {
            None
        }
    }
    /// snaps the board index the mouse is over
    fn snap_piece(&mut self, index: usize, board_index: u8) {
        self.pieces[index].position = self.board_view.squares.get(board_index as usize).unwrap().shape.top_left().clone();
    }

    /// iterates over all the squares in the struct BoardView and renders it
    fn draw_chess_board(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        for s in self.board_view.squares.iter() {
            graphics.draw_rectangle(s.shape.to_owned(), s.color)
        }
    }

    /// iterates over all pieces (draggable) in the Vec and renders them
    fn draw_pieces(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        for component in &self.pieces {
            let image:ImageHandle = graphics.create_image_from_file_path(
                Some(ImageFileFormat::PNG), 
                ImageSmoothingMode::Linear,
                &component.image_path).unwrap();

            graphics.draw_image(
                component.position,
                &image,
            );
        }
    }

    /// def. not final, may go
    fn read_board(&mut self) {
        // TODO: check the logic and represent the actual board.

        for (index, p) in self.board.pieces.iter().enumerate() {
            let mut curr_bit: u64 = 1;
            for i in 0..NUM_SQUARES {
                if (p & curr_bit) != 0 {

                    // can probably be made simpler, idk -> TODO?

                    if (p & (self.board.colors[0] & curr_bit)) != 0 { // white
                        self.pieces.push(DraggableChessPiece::new(
                            match_image_path(index, PieceColor::White).unwrap(),
                            Vector2 { 
                                x: (i * SQUARE_DIMENSION) as f32,
                                y: ((i / 8) * SQUARE_DIMENSION) as f32 
                            },
                            Vector2::ZERO,
                            i as u8,
                        ));
                    }
                    else if (p & (self.board.colors[1] & curr_bit)) != 0 { // black
                        self.pieces.push(DraggableChessPiece::new(
                            match_image_path(index, PieceColor::Black).unwrap(),
                            Vector2 { 
                                x: (i * SQUARE_DIMENSION) as f32,
                                y: ((i / 8) * SQUARE_DIMENSION) as f32 
                            },
                            Vector2{ x: SQUARE_DIMENSION as f32, y: SQUARE_DIMENSION as f32},
                            i as u8,
                        ));
                    }
                }
                // shift the current bit we are on
                curr_bit <<= 1;
            }
        }  
    }
}

fn match_image_path(index: usize, color: PieceColor) -> Option<String> {
    Some(match Piece::from(index) {
        Piece::Pawn => if color == PieceColor::Black { B_P.to_string() } else { W_P.to_string() },
        Piece::Bishop => if color == PieceColor::Black { B_B.to_string() } else { W_B.to_string() },
        Piece::Knight => if color == PieceColor::Black { B_N.to_string() } else { W_N.to_string() },
        Piece::Rook => if color == PieceColor::Black { B_R.to_string() } else { W_R.to_string() },
        Piece::Queen => if color == PieceColor::Black { B_Q.to_string() } else { W_Q.to_string() },
        Piece::King => if color == PieceColor::Black { B_K.to_string() } else { W_K.to_string() },
    })
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
            squares,
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

struct DraggableChessPiece {
    image_path: String,
    position: Vector2<f32>,
    size: Vector2<f32>,
    is_dragging: bool,
    offset: Vector2<f32>,
    board_index: u8,
}

impl DraggableChessPiece {
    fn new(image_path: String, position: Vector2<f32>, size: Vector2<f32>, board_index: u8) -> Self {
        Self {
            image_path,
            position,
            size,
            is_dragging: false,
            offset : Vector2::ZERO,
            board_index,
        }
    }

    fn contains_point(&self, point: Vector2<f32>, cwh: &&mut ChessWinHandler) -> bool {
        let rect = &cwh.board_view.squares.get(self.board_index as usize).unwrap().shape;
        rect.contains(point)
    }
}

