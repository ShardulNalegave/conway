
// ===== Imports =====
use nannou::prelude::*;

use crate::state::{GameState, BLOCK_WH, NUM_BLOCKS_WIDTH, NUM_BOCKS_HEIGHT};
// ===================

pub fn render_graphics(draw: &mut Draw, game_state: &GameState, win: Rect) {
  draw.background().color(WHITE);

  for y in 0..NUM_BOCKS_HEIGHT {
    for x in 0..NUM_BLOCKS_WIDTH {
      let color = {
        if game_state.blocks[y][x] == 1 { BLACK } else { WHITE }
      };
      let text_color = {
        if color == BLACK { WHITE } else { BLACK }
      };

      draw.rect()
        .x(((BLOCK_WH / 2.0) + (BLOCK_WH * x as f32)) - (win.w() / 2.0))
        .y(((BLOCK_WH / 2.0) + (BLOCK_WH * y as f32)) - (win.h() / 2.0))
        .w_h(BLOCK_WH, BLOCK_WH)
        .color(color)
        .stroke_weight(3.0)
        .stroke(BLACK);

      if game_state.show_neighbours {
        draw.text(&format!("{:?}", game_state.get_num_live_neighbours(x, y)))
          .x(((BLOCK_WH / 2.0) + (BLOCK_WH * x as f32)) - (win.w() / 2.0))
          .y(((BLOCK_WH / 2.0) + (BLOCK_WH * y as f32)) - (win.h() / 2.0))
          .color(text_color);
      }
    }
  }
}