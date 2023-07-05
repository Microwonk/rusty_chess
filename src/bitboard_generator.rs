use speedy2d::{Window, window::{WindowHandler, WindowHelper}, dimen::Vector2, Graphics2D, color::Color, shape::Rect};

use crate::graphics::BoardView;

const WINDOW_DIMENSION: u32 = 640;

pub fn init() {
    let window = Window::new_centered("Generator", (WINDOW_DIMENSION, WINDOW_DIMENSION)).unwrap();

    window.run_loop(BitboardWinHandler{
        board_view: BoardView::new(),
        mouse_position: Vector2::ZERO,
    });
}

struct BitboardWinHandler {
    board_view: BoardView,
    mouse_position: Vector2<f32>,
}

impl WindowHandler for BitboardWinHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        
        self.draw_chess_board(helper, graphics);

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}

impl BitboardWinHandler {
    fn draw_chess_board(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        for s in self.board_view.squares.iter() {
            graphics.draw_rectangle(s.shape.to_owned(), s.color)
        }
    }
}