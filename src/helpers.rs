use std::time::Instant;

#[cfg(not(target_os = "emscripten"))]
pub fn frame_wait(t1: Instant) {
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
pub fn frame_wait(t1: Instant) {
  use crate::emscripten_wrappers::emscripten;

  let frame_time = (Instant::now() - t1).as_millis();
  if frame_time < u32::MAX as u128 {
    let fps = 1_000u32 / 60u32;
    if fps > frame_time as u32 {
      emscripten::sleep(fps - frame_time as u32);
    }
  }
}
