use crate::game::GameState;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Apple {
  pub x: u32,
  pub y: u32,
  pub eaten: bool,
}

impl Apple {
  pub fn new(window_width: u32, window_height: u32, box_size: u32) -> Self {
    let mut rng = rand::thread_rng();
    Self {
      x: rng.gen_range(0..window_width / box_size) * box_size,
      y: rng.gen_range(0..window_height / box_size) * box_size,
      eaten: false,
    }
  }

  pub fn tick(&mut self, game_state: &GameState) {
    if self.eaten {
      self.eaten = false;
      let mut rng = rand::thread_rng();

      self.x =
        rng.gen_range(0..game_state.window_width / game_state.box_size) * game_state.box_size;

      self.y =
        rng.gen_range(0..game_state.window_height / game_state.box_size) * game_state.box_size;
    }
  }

  pub fn draw(&self, game_state: &GameState, canvas: &mut Canvas<Window>) {
    // Draw apple
    canvas.set_draw_color(Color::RED);
    canvas
      .fill_rect(Rect::new(
        self.x as i32,
        self.y as i32,
        game_state.box_size,
        game_state.box_size,
      ))
      .unwrap();
  }
}
