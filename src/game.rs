use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use rand::Rng;
use std::time::Instant;

static BOX_SIZE: u32 = 15; // divisions of the smaller axis of the screen
static GAME_TICK_SPEED: u128 = 50; // milliseconds
static WIGGLE: i8 = 2; // milliseconds

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct GameState<'a> {
    font: Font<'a, 'a>,
    paused: bool,
    timer: Instant,
    direction: Direction,
    changed_direction: bool,
    positions: Vec<(u32, u32, i8, i8)>,
    length: u32,
    offset: i8,
    d_offset: i8,
    apple: (u32, u32),
}

impl<'a> GameState<'a> {
    pub fn init(canvas: &Canvas<Window>, font: Font<'a, 'a>) -> Self {
        let (window_width, window_height) = canvas.window().size();

        let mut rng = rand::thread_rng();
        let apple = (
            rng.gen_range(0..window_width / BOX_SIZE) * BOX_SIZE,
            rng.gen_range(0..window_height / BOX_SIZE) * BOX_SIZE,
        );

        let x = window_width / BOX_SIZE / 2 * BOX_SIZE;
        let y = window_height / BOX_SIZE / 2 * BOX_SIZE;

        Self {
            font,
            paused: true,
            timer: Instant::now(),
            direction: Direction::Up,
            changed_direction: false,
            positions: vec![(x, y, 0, 0), (x, y, 0, 0), (x, y, 0, 0)],
            length: 3,
            offset: 0,
            d_offset: 1,
            apple,
        }
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.paused = false;
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
                self.paused = false;
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
                self.paused = false;
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
                self.paused = false;
                if self.direction != Direction::Right {
                    self.changed_direction = true;
                }
                self.direction = Direction::Right
            }

            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.paused = !self.paused;
                self.timer = Instant::now();
            }
            _ => {}
        }
    }

    pub fn tick(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        if !self.paused && (Instant::now() - self.timer).as_millis() > GAME_TICK_SPEED {
            self.timer = Instant::now();

            if self.offset == WIGGLE {
                self.d_offset = -1;
            } else if self.offset == -WIGGLE {
                self.d_offset = 1;
            }
            self.offset += self.d_offset;

            let (window_width, window_height) = canvas.window().size();
            let (prev_x, prev_y, _, _) = self.positions.last().unwrap().clone();
            if self.changed_direction {
                self.changed_direction = false;
                self.positions.pop();
                self.positions.push((prev_x, prev_y, 0, 0));
            }
            match &self.direction {
                Direction::Up => {
                    if prev_y != 0 {
                        self.positions
                            .push((prev_x, prev_y - BOX_SIZE, self.offset, 0));
                    } else {
                        self.positions.push((
                            prev_x,
                            BOX_SIZE * (window_height / BOX_SIZE),
                            self.offset,
                            0,
                        ));
                    }
                }
                Direction::Down => {
                    if (prev_y + BOX_SIZE) < window_height {
                        self.positions
                            .push((prev_x, prev_y + BOX_SIZE, self.offset, 0));
                    } else {
                        self.positions.push((prev_x, 0, self.offset, 0));
                    }
                }
                Direction::Left => {
                    if prev_x != 0 {
                        self.positions
                            .push((prev_x - BOX_SIZE, prev_y, 0, self.offset));
                    } else {
                        self.positions.push((
                            BOX_SIZE * (window_width / BOX_SIZE),
                            prev_y,
                            0,
                            self.offset,
                        ));
                    }
                }
                Direction::Right => {
                    if (prev_x + BOX_SIZE) < window_width {
                        self.positions
                            .push((prev_x + BOX_SIZE, prev_y, 0, self.offset));
                    } else {
                        self.positions.push((0, prev_y, 0, self.offset));
                    }
                }
            }

            // Got apple?
            if self.apple.0 == self.positions.last().unwrap().0
                && self.apple.1 == self.positions.last().unwrap().1
            {
                self.length += 1;
                let mut rng = rand::thread_rng();
                self.apple = (
                    rng.gen_range(0..canvas.window().size().0 / BOX_SIZE) * BOX_SIZE,
                    rng.gen_range(0..canvas.window().size().1 / BOX_SIZE) * BOX_SIZE,
                );
            }

            if self.positions.len() > self.length as usize {
                self.positions.rotate_left(1);
                self.positions.pop();
            }
        }

        // Draw Score
        let text_texture = texture_creator
            .create_texture_from_surface(
                self.font
                    .render(&format!("{}", self.positions.len()))
                    .blended(Color::BLACK)
                    .unwrap(),
            )
            .unwrap();
        let TextureQuery { width, height, .. } = text_texture.query();
        canvas
            .copy(
                &text_texture,
                None,
                Rect::new(
                    canvas.window().size().0 as i32 - width as i32 - 2,
                    2,
                    width,
                    height,
                ),
            )
            .unwrap();

        // Draw Snek
        let g_increment: f64 = 255.0 / self.positions.len() as f64;
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
                    BOX_SIZE,
                    BOX_SIZE,
                ))
                .unwrap();
        }

        // Draw apple
        canvas.set_draw_color(Color::RED);
        canvas
            .fill_rect(Rect::new(
                self.apple.0 as i32,
                self.apple.1 as i32,
                BOX_SIZE,
                BOX_SIZE,
            ))
            .unwrap();
    }
}
