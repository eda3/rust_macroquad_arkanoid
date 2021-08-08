use macroquad::prelude::*;

#[macroquad::main("Arkanoid")]
async fn main() {
  loop {
    clear_background(SKYBLUE);

    next_frame().await
  }
}
