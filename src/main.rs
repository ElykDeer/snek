mod game;

#[cfg(target_os = "emscripten")]
mod emscripten_wrappers;
#[cfg(target_os = "emscripten")]
use emscripten_wrappers::emscripten;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::path::Path;
use std::time::Instant;

fn main() {
    #[cfg(target_os = "emscripten")]
    let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY", "1");

    #[cfg(target_os = "emscripten")]
    let (window_width, window_height) = emscripten::get_canvas_size();
    #[cfg(not(target_os = "emscripten"))]
    let (window_width, window_height) = (1200, 800);

    // Initialize graphics
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rust/SDL/Emscripten Template", window_width, window_height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();

    // Initialize fonts
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context
        .load_font(Path::new("assets/cruft.ttf"), 50)
        .unwrap();

    let mut game_state = game::GameState::init(&canvas, font);

    // After initializing everything, in the web version, we should delete the spinner from in front of the canvas
    #[cfg(target_os = "emscripten")]
    {
        emscripten::sleep(1500); // Just to demonstrate that the spinner works
        emscripten::exec("let spinner = document.getElementById('spinner'); spinner.remove();");
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'mainloop: loop {
        let t1 = Instant::now();

        #[cfg(target_os = "emscripten")]
        if canvas.window().size().0 != emscripten::get_canvas_size().0
            || canvas.window().size().1 != emscripten::get_canvas_size().1
        {
            canvas
                .window_mut()
                .set_size(
                    emscripten::get_canvas_size().0 as u32,
                    emscripten::get_canvas_size().1 as u32,
                )
                .unwrap();
        }

        // Clear screen
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Process this frame's events
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {
                    game_state.process_event(event);
                }
            }
        }

        // Draw
        game_state.tick(&mut canvas, &texture_creator);
        canvas.present();

        // Set framerate to ~60fps
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::thread;
            use std::time::Duration;
            let frame_time = (Instant::now() - t1).as_nanos();
            if frame_time < u32::MAX as u128 {
                let fps = 1_000_000_000u32 / 60u32;
                if fps > frame_time as u32 {
                    thread::sleep(Duration::new(0, fps - frame_time as u32));
                }
            }
        }
        #[cfg(target_os = "emscripten")]
        {
            let frame_time = (Instant::now() - t1).as_millis();
            if frame_time < u32::MAX as u128 {
                let fps = 1_000u32 / 60u32;
                if fps > frame_time as u32 {
                    emscripten::sleep(fps - frame_time as u32);
                }
            }
        }

        println!(
            "FPS: {}",
            1_000_000_000u128 / (Instant::now() - t1).as_nanos()
        );
    }
}
