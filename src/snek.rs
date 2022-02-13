use crate::game::GameState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Instant;

#[derive(PartialEq, Eq)]
enum Direction {
  Up,
  Down,
  Right,
  Left,
}

pub struct Snek {
  timer: Instant,

  direction: Direction,
  changed_direction: bool,

  positions: Vec<(u32, u32, i8, i8)>,
  pub len: u32,

  offset: i8,
  d_offset: i8,
  wiggle: i8,
}

impl Snek {
  pub fn new(window_width: u32, window_height: u32, box_size: u32) -> Self {
    let x = window_width / box_size / 2 * box_size;
    let y = window_height / box_size / 2 * box_size;

    Self {
      timer: Instant::now(),
      direction: Direction::Up,
      changed_direction: false,
      positions: vec![(x, y, 0, 0), (x, y, 0, 0), (x, y, 0, 0)],
      len: 3,
      offset: 0,
      d_offset: 1,
      wiggle: 2,
    }
  }

  pub fn position(&self) -> (u32, u32) {
    (
      self.positions.last().unwrap().0,
      self.positions.last().unwrap().1,
    )
  }

  fn move_up(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if prev_y != 0 {
      self
        .positions
        .push((prev_x, prev_y - game_state.box_size, self.offset, 0));
    } else {
      self.positions.push((
        prev_x,
        game_state.box_size * (game_state.window_height / game_state.box_size),
        self.offset,
        0,
      ));
    }
  }

  fn move_down(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if (prev_y + game_state.box_size) < game_state.window_height {
      self
        .positions
        .push((prev_x, prev_y + game_state.box_size, self.offset, 0));
    } else {
      self.positions.push((prev_x, 0, self.offset, 0));
    }
  }

  fn move_left(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if prev_x != 0 {
      self
        .positions
        .push((prev_x - game_state.box_size, prev_y, 0, self.offset));
    } else {
      self.positions.push((
        game_state.box_size * (game_state.window_width / game_state.box_size),
        prev_y,
        0,
        self.offset,
      ));
    }
  }

  fn move_right(&mut self, game_state: &GameState, prev_x: u32, prev_y: u32) {
    if (prev_x + game_state.box_size) < game_state.window_width {
      self
        .positions
        .push((prev_x + game_state.box_size, prev_y, 0, self.offset));
    } else {
      self.positions.push((0, prev_y, 0, self.offset));
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
      } => {
        if self.direction != Direction::Up {
          self.changed_direction = true;
        }
        self.direction = Direction::Up
      }

      Event::KeyDown {
        keycode: Some(Keycode::Down),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::S),
        ..
      } => {
        if self.direction != Direction::Down {
          self.changed_direction = true;
        }
        self.direction = Direction::Down
      }

      Event::KeyDown {
        keycode: Some(Keycode::Left),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::A),
        ..
      } => {
        if self.direction != Direction::Left {
          self.changed_direction = true;
        }
        self.direction = Direction::Left
      }

      Event::KeyDown {
        keycode: Some(Keycode::Right),
        ..
      }
      | Event::KeyDown {
        keycode: Some(Keycode::D),
        ..
      } => {
        if self.direction != Direction::Right {
          self.changed_direction = true;
        }
        self.direction = Direction::Right
      }
      _ => (),
    }
  }

  pub fn tick(&mut self, game_state: &GameState) {
    if (Instant::now() - self.timer).as_millis() > game_state.snek_tick_speed_ms {
      self.timer = Instant::now();

      // TODO : Refactor animation out into rendering code
      if self.offset == self.wiggle {
        self.d_offset = -1;
      } else if self.offset == -self.wiggle {
        self.d_offset = 1;
      }
      self.offset += self.d_offset;

      let (prev_x, prev_y, _, _) = self.positions.last().unwrap().clone();
      // TODO : This is also technically animation code....refactor out
      if self.changed_direction {
        self.changed_direction = false;
        self.positions.pop();
        self.positions.push((prev_x, prev_y, 0, 0));
      }
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

  pub fn draw(&self, game_state: &GameState, canvas: &mut Canvas<Window>) {
    let g_increment: f64 = 255.0 / self.len as f64;
    let mut g: f64 = 32.0;
    for (x, y, dx, dy) in self.positions.iter() {
      canvas.set_draw_color(Color::RGB(0, g.floor() as u8, 20));
      if g + g_increment <= 255.0 {
        g += g_increment;
      }
      canvas
        .fill_rect(Rect::new(
          *x as i32 + *dx as i32,
          *y as i32 + *dy as i32,
          game_state.box_size,
          game_state.box_size,
        ))
        .unwrap();
    }
  }
}
