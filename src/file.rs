use crate::apple::Apple;
#[cfg(target_os = "emscripten")]
use crate::emscripten_wrappers::emscripten;
use crate::game::Game;
use crate::snek::Direction;
use crate::snek::Snek;

use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(not(target_os = "emscripten"))]
use directories::ProjectDirs;
#[cfg(not(target_os = "emscripten"))]
use std::fs::create_dir_all;
#[cfg(not(target_os = "emscripten"))]
use std::fs::File;
#[cfg(not(target_os = "emscripten"))]
use std::io::prelude::Write;

static BOX_SIZE: u32 = 15;

#[derive(Serialize, Deserialize)]
pub struct SnekData {
  pub direction: Direction,

  pub positions: Vec<(u32, u32, i8, i8)>,
  pub len: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
  pub snek_tick_speed_ms: u128,
  pub sneks: Vec<SnekData>,
  pub apples: Vec<Apple>,
}

#[cfg(not(target_os = "emscripten"))]
pub fn load<'a>(canvas: &Canvas<Window>, font: Font<'a, 'a>) -> Game<'a> {
  let project_dirs = ProjectDirs::from("", "ElykDeer", "snek").unwrap();
  let path = project_dirs.data_dir().join("save.dat");

  if let Ok(file_reader) = File::open(&path) {
    if let Ok(game_data) = serde_json::from_reader::<_, GameData>(file_reader) {
      let sneks = game_data
        .sneks
        .into_iter()
        .map(|s| Snek::load(s.direction, s.positions, s.len))
        .collect();

      return Game::load(
        canvas.window().size().0,
        canvas.window().size().1,
        font,
        BOX_SIZE,
        game_data.snek_tick_speed_ms,
        sneks,
        game_data.apples,
      );
    }
  }

  Game::new(canvas, font, BOX_SIZE)
}

#[cfg(target_os = "emscripten")]
pub fn load<'a>(canvas: &Canvas<Window>, font: Font<'a, 'a>) -> Game<'a> {
  let save_data = emscripten::fs::get_save_data();

  if let Ok(game_data) = serde_json::from_str::<GameData>(&save_data) {
    let sneks = game_data
      .sneks
      .into_iter()
      .map(|s| Snek::load(s.direction, s.positions, s.len))
      .collect();

    return Game::load(
      canvas.window().size().0,
      canvas.window().size().1,
      font,
      BOX_SIZE,
      game_data.snek_tick_speed_ms,
      sneks,
      game_data.apples,
    );
  }

  Game::new(canvas, font, BOX_SIZE)
}

#[cfg(not(target_os = "emscripten"))]
pub fn save<'a>(game_data: GameData) {
  let project_dirs = ProjectDirs::from("", "ElykDeer", "snek").unwrap();
  let path = project_dirs.data_dir();

  if create_dir_all(path).is_ok() {
    let path = path.join("save.dat");

    if let Ok(mut file_writer) = File::create(&path) {
      if let Ok(game_json) = serde_json::to_string(&game_data) {
        file_writer
          .write_all(game_json.as_bytes())
          .expect("Unable to save game!");
      }
    } else {
      println!("Failed to create save file!");
    }
  } else {
    println!("Failed to create save directory {:?}!", path);
  }
}

#[cfg(target_os = "emscripten")]
pub fn save<'a>(game_data: GameData) {
  if let Ok(game_json) = serde_json::to_string(&game_data) {
    emscripten::fs::save(&game_json);
  }
}
