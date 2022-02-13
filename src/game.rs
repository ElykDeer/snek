use crate::apple::Apple;
use crate::file::{save, GameData};
use crate::snek::Snek;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use std::time::Instant;

pub struct GameState {
    pub window_width: u32,
    pub window_height: u32,
    pub box_size: u32,
    pub snek_tick_speed_ms: u128,
    pub last_save_time: Instant,
}

pub struct Game<'a> {
    game_state: GameState,
    font: Font<'a, 'a>,
    paused: bool,

    sneks: Vec<Snek>,
    apples: Vec<Apple>,
}

impl<'a> Game<'a> {
    pub fn new(canvas: &Canvas<Window>, font: Font<'a, 'a>, box_size: u32) -> Self {
        let (window_width, window_height) = canvas.window().size();

        Self {
            game_state: GameState {
                window_width,
                window_height,
                box_size,
                snek_tick_speed_ms: 50,
                last_save_time: Instant::now(),
            },

            font,
            paused: true,

            sneks: vec![Snek::new(window_width, window_height, box_size)],
            apples: vec![Apple::new(window_width, window_height, box_size)],
        }
    }

    pub fn load(
        window_width: u32,
        window_height: u32,
        font: Font<'a, 'a>,
        box_size: u32,
        snek_tick_speed_ms: u128,
        sneks: Vec<Snek>,
        apples: Vec<Apple>,
    ) -> Self {
        Self {
            game_state: GameState {
                window_width,
                window_height,
                box_size,
                snek_tick_speed_ms,
                last_save_time: Instant::now(),
            },

            font,
            paused: true,

            sneks,
            apples,
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.paused = !self.paused;
                // self.timer = Instant::now(); // TODO : Reset all snek timers?
            }
            _ => {
                for snek in &mut self.sneks {
                    snek.process_event(&event);
                }
            }
        }
    }

    pub fn tick(&mut self) {
        // Check if game's been saved, save
        if (Instant::now() - self.game_state.last_save_time).as_secs() > 5 {
            save(self.into());
        }

        if self.paused {
            // Check interactions between game objects
            //   If a snake has eaten an apple:
            'apples: for apple in &mut self.apples {
                if !apple.eaten {
                    for snek in &mut self.sneks {
                        if snek.position() == (apple.x, apple.y) {
                            snek.len += 1;
                            apple.eaten = true;
                            continue 'apples;
                        }
                    }
                }
            }

            for snek in &mut self.sneks {
                snek.tick(&self.game_state);
            }

            for apple in &mut self.apples {
                apple.tick(&self.game_state);
            }
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        // Draw Score
        let score = self.sneks.iter().fold(0, |acc, snek| acc + snek.len - 3);
        let text_texture = texture_creator
            .create_texture_from_surface(
                self.font
                    .render(&format!("{}", score))
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

        // Draw sneks
        for snek in &self.sneks {
            snek.draw(&self.game_state, canvas);
        }

        // Draw apples
        for apple in &self.apples {
            apple.draw(&self.game_state, canvas);
        }
    }
}

impl<'a> Into<GameData> for Game<'a> {
    fn into(self) -> GameData {
        let sneks = self.sneks.iter().map(|s| s.into()).collect();

        GameData {
            snek_tick_speed_ms: self.game_state.snek_tick_speed_ms,
            sneks,
            apples: self.apples.clone(),
        }
    }
}

impl<'a> Into<GameData> for &mut Game<'a> {
    fn into(self) -> GameData {
        let sneks = self.sneks.iter().map(|s| s.into()).collect();

        GameData {
            snek_tick_speed_ms: self.game_state.snek_tick_speed_ms,
            sneks,
            apples: self.apples.clone(),
        }
    }
}
