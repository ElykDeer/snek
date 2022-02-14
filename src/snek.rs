use crate::file::SnekData;
use crate::game::GameState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

pub struct Snek {
  timer: Instant,
  tick_speed_ms: u128,

  direction: Direction,
  positions: Vec<(u32, u32)>,
  pub len: u32,

  animation: SnekWiggleAnimation,
}

struct SnekWiggleAnimation {
  offsets: Vec<(i16, i16)>,
  total_wiggle: i16,
  t: f64,
}

impl SnekWiggleAnimation {
  fn tick(&mut self, positions: &[(u32, u32)]) {
    self.t += std::f64::consts::TAU / 9.0;
    self.t %= std::f64::consts::TAU;

    let step = std::f64::consts::TAU / 12.0;
    let mut offset_d = self.t;
    self.offsets = (0..positions.len())
      .into_iter()
      .map(|i| {
        // Update wiggle
        if i != positions.len() - 1 {
          offset_d += step;
        }

        // Check head
        if i == positions.len() - 1 {
          (0, 0)
        }
        // Check tail
        else if i == 0 {
          if positions[i].0 == positions[i + 1].0 && positions[i + 1].0 == positions[i + 2].0 {
            (
              (offset_d.sin() * self.total_wiggle as f64).floor() as i16,
              0,
            )
          } else if positions[i].1 == positions[i + 1].1 && positions[i + 1].1 == positions[i + 2].1
          {
            (
              0,
              (offset_d.sin() * self.total_wiggle as f64).floor() as i16,
            )
          } else {
            (0, 0)
          }
        } else {
          if positions[i - 1].0 == positions[i].0 && positions[i].0 == positions[i + 1].0 {
            (
              (offset_d.sin() * self.total_wiggle as f64).floor() as i16,
              0,
            )
          } else if positions[i - 1].1 == positions[i].1 && positions[i].1 == positions[i + 1].1 {
            (
              0,
              (offset_d.sin() * self.total_wiggle as f64).floor() as i16,
            )
          } else {
            (0, 0)
          }
        }
      })
      .collect();
  }
}

impl Snek {
  pub fn new(window_width: u32, window_height: u32, box_size: u32) -> Self {
    let x = window_width / box_size / 2 * box_size;
    let y = window_height / box_size / 2 * box_size;

    Self {
      timer: Instant::now(),
      tick_speed_ms: 50,
      direction: Direction::Up,
      positions: vec![(x, y), (x, y), (x, y)],
      len: 3,

      animation: SnekWiggleAnimation {
        offsets: vec![(0, 0), (0, 0), (0, 0)],
        total_wiggle: 2,
        t: 0.0,
      },
    }
  }

  pub fn load(
    direction: Direction,
    positions: Vec<(u32, u32)>,
    len: u32,
    tick_speed_ms: u128,
  ) -> Self {
    Self {
      timer: Instant::now(),
      tick_speed_ms,
      direction,
      positions,
      len,
      animation: SnekWiggleAnimation {
        offsets: (0..len).into_iter().map(|_| (0, 0)).collect(),
        total_wiggle: 2,
        t: 0.0,
      },
    }
  }

  pub fn position(&self) -> (u32, u32) {
    *self.positions.last().unwrap()
  }

  fn move_up(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if prev_y != 0 {
      self.positions.push((prev_x, prev_y - game_state.box_size));
    } else {
      self.positions.push((
        prev_x,
        game_state.box_size * (game_state.window_height / game_state.box_size),
      ));
    }
  }

  fn move_down(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if (prev_y + game_state.box_size) < game_state.window_height {
      self.positions.push((prev_x, prev_y + game_state.box_size));
    } else {
      self.positions.push((prev_x, 0));
    }
  }

  fn move_left(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if prev_x != 0 {
      self.positions.push((prev_x - game_state.box_size, prev_y));
    } else {
      self.positions.push((
        game_state.box_size * (game_state.window_width / game_state.box_size),
        prev_y,
      ));
    }
  }

  fn move_right(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if (prev_x + game_state.box_size) < game_state.window_width {
      self.positions.push((prev_x + game_state.box_size, prev_y));
    } else {
      self.positions.push((0, prev_y));
    }
  }

  pub fn process_event(&mut self, event: &Event) {
    match event {
      Event::KeyDown {
        keycode: Some(Keycode::Up),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::W),
        ..
      } => self.direction = Direction::Up,

      Event::KeyDown {
        keycode: Some(Keycode::Down),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::S),
        ..
      } => self.direction = Direction::Down,

      Event::KeyDown {
        keycode: Some(Keycode::Left),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::A),
        ..
      } => self.direction = Direction::Left,

      Event::KeyDown {
        keycode: Some(Keycode::Right),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::D),
        ..
      } => self.direction = Direction::Right,
      _ => (),
    }
  }

  pub fn tick(&mut self, game_state: &GameState) {
    if (Instant::now() - self.timer).as_millis() > self.tick_speed_ms {
      let (prev_x, prev_y) = self.positions.last().unwrap().clone();
      match &self.direction {
        Direction::Up => self.move_up(&game_state, prev_x, prev_y),
        Direction::Down => self.move_down(&game_state, prev_x, prev_y),
        Direction::Left => self.move_left(&game_state, prev_x, prev_y),
        Direction::Right => self.move_right(&game_state, prev_x, prev_y),
      }

      // Don't just grow forever
      if self.positions.len() > self.len as usize {
        self.positions.rotate_left(1);
        self.positions.pop();
      }
    }
  }

  pub fn tick_animations(&mut self) {
    if (Instant::now() - self.timer).as_millis() > self.tick_speed_ms {
      // TODO : THIS SHOULD NOT BE IN ANIMATIONS
      self.timer = Instant::now();

      self.animation.tick(self.positions.as_slice());
    }
  }

  pub fn draw(&self, game_state: &GameState, canvas: &mut Canvas<Window>) {
    let g_increment: f64 = 255.0 / self.len as f64;
    let mut g: f64 = 32.0;

    for i in 0..self.positions.len() {
      canvas.set_draw_color(Color::RGB(0, g.floor() as u8, 20));
      if g + g_increment <= 255.0 {
        g += g_increment;
      }
      canvas
        .fill_rect(Rect::new(
          self.positions[i].0 as i32 + self.animation.offsets[i].0 as i32,
          self.positions[i].1 as i32 + self.animation.offsets[i].1 as i32,
          game_state.box_size,
          game_state.box_size,
        ))
        .unwrap();
    }
  }
}

impl Into<SnekData> for &Snek {
  fn into(self) -> SnekData {
    SnekData {
      direction: self.direction.clone(),
      positions: self.positions.clone(),
      len: self.len,
      tick_speed_ms: self.tick_speed_ms,
    }
  }
}
