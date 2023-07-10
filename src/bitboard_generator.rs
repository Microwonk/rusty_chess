use std::vec;

use speedy2d::{Window, window::{WindowHandler, WindowHelper}, dimen::Vector2, Graphics2D, color::Color};

use crate::graphics::{BoardView, SQUARE_DIMENSION, WINDOW_DIMENSION};

pub fn init() {
    let window = Window::new_centered("Generator", (WINDOW_DIMENSION, WINDOW_DIMENSION)).unwrap();

    window.run_loop(BitboardWinHandler{
        board_view: BoardView::new(),
        mouse_position: Vector2::ZERO,
        clicked_indices: vec![],
    });
}

struct BitboardWinHandler {
    board_view: BoardView,
    mouse_position: Vector2<f32>,
    clicked_indices: Vec<usize>,
}

impl WindowHandler for BitboardWinHandler {
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
        _button: speedy2d::window::MouseButton) {

            let clicked_index = self.translate_mouse_pos_to_index().unwrap_or(0) as usize;

            if !self.clicked_indices.contains(&clicked_index) {
                self.clicked_indices.push(clicked_index);
            } else {
                self.clicked_indices.retain(|&i| i != clicked_index);
            }
        }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        self.mouse_position = position;
    }

    fn on_key_down(
            &mut self,
            _helper: &mut WindowHelper<()>,
            _virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
            scancode: speedy2d::window::KeyScancode
        ) {
        if scancode == 28 {
            self.indices_to_num();
        }
    }
}

impl BitboardWinHandler {
    fn draw_chess_board(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        for (i, s) in self.board_view.squares.iter().enumerate() {
            if self.clicked_indices.contains(&i) {
                graphics.draw_rectangle(s.shape.to_owned(), Color::from_int_rgb(250, 0, 0));
            } else {
                graphics.draw_rectangle(s.shape.to_owned(), s.color)
            }
        }
    }

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

    fn indices_to_num(&self) {
        let mut number: u64 = 0;

        for index in &self.clicked_indices {
            number |= 1 << index;
        }

        let bit_string = format!("{:064b}", number);
        let hex_string = format!("{:016X}", number);

        println!("{}", format!("Bit Board: {}\nHex Board: {}", bit_string, hex_string));
    } 
}