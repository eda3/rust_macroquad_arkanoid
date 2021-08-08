use macroquad::prelude::*;

#[macroquad::main("Arkanoid")]
async fn main() {
  const BLOCKS_W: usize = 10;
  const BLOCKS_H: usize = 10;
  const SCR_W: f32 = 20.;
  const SCR_H: f32 = 20.;

  let mut blocks: [[bool; BLOCKS_W]; BLOCKS_H] = [[true; BLOCKS_W]; BLOCKS_H];

  set_camera(&Camera2D {
    zoom: vec2(1.0 / SCR_W * 2.0, -1.0 / SCR_H * 2.0),
    target: vec2(SCR_W / 2.0, SCR_H / 2.0),
    ..Default::default()
  });

  loop {
    clear_background(SKYBLUE);

    // ブロックの描画
    for j in 0..BLOCKS_H {
      for i in 0..BLOCKS_W {
        if blocks[j][i] {
          let block_w = SCR_W / BLOCKS_W as f32;
          let block_h = 7.0 / BLOCKS_H as f32;
          let block_x = i as f32 * block_w + 0.05;
          let block_y = j as f32 * block_h + 0.05;

          draw_rectangle(block_x, block_y, block_w - 0.1, block_h - 0.1, DARKBLUE);
        }
      }
    }

    next_frame().await
  }
}
