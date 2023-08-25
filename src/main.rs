
pub mod state;
pub mod graphics;

// ===== Imports =====
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

use crate::{
  state::{GameState, SCREEN_WIDTH, SCREEN_HEIGHT},
  graphics::render_graphics,
};
use crate::state::BLOCK_WH;
// ===================

fn main() {
  nannou::app(get_state).loop_mode(LoopMode::Wait).run();
}

fn get_state(app: &App) -> GameState {
  // Create window
  let _window_id = app
    .new_window()
    .title("Conway's Game Of Life")
    .view(view)
    .event(event)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .resizable(false)
    .maximized(false)
    .build()
    .unwrap();

  GameState::new()
}

fn view(app: &App, game_state: &GameState, frame: Frame) {
  let mut draw = app.draw();
  render_graphics(&mut draw, game_state, app.window_rect());

  draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, game_state: &mut GameState, event: WindowEvent) {
  let win = app.window_rect();

  match event {
    MousePressed(_btn) => {
      let x = ((app.mouse.x + (win.w() / 2.0)) / (BLOCK_WH as f32)) as usize;
      let y = ((app.mouse.y + (win.h() / 2.0)) / (BLOCK_WH as f32)) as usize;
      game_state.blocks[y][x] = if game_state.blocks[y][x] == 1 { 0 } else { 1 };
    },
    KeyReleased(key) => match key {
      VirtualKeyCode::Q => { app.quit() },
      VirtualKeyCode::S => { game_state.show_neighbours = !game_state.show_neighbours },
      VirtualKeyCode::N => { game_state.next_generation() },
      _ => {},
    },
    _ => {},
  }
}