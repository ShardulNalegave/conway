
pub mod state;
pub mod ui;
pub mod graphics;

// ===== Imports =====
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou_egui::{self, Egui};

use crate::{
  state::{State, SCREEN_WIDTH, SCREEN_HEIGHT},
  ui::render_ui,
  graphics::render_graphics,
};
use crate::state::BLOCK_WH;
// ===================

fn main() {
  nannou::app(get_state).update(update).run();
}

fn get_state(app: &App) -> State {
  // Create window
  let window_id = app
    .new_window()
    .view(view)
    .event(event)
    .raw_event(raw_window_event)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .resizable(false)
    .maximized(false)
    .build()
    .unwrap();
  let window = app.window(window_id).unwrap();

  let egui = Egui::from_window(&window);

  State::new(egui)
}

fn update(_app: &App, state: &mut State, update: Update) {
  let egui = &mut state.egui;
  let game_state = &mut state.game;

  egui.set_elapsed_time(update.since_start);
  let ctx = egui.begin_frame();

  render_ui(ctx, game_state);
}

fn view(app: &App, state: &State, frame: Frame) {
  let mut draw = app.draw();
  render_graphics(&mut draw, &state.game, app.window_rect());

  draw.to_frame(app, &frame).unwrap();
  state.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, state: &mut State, event: &nannou::winit::event::WindowEvent) {
  // Let egui handle things like keyboard and mouse input.
  state.egui.handle_raw_event(event);
}

fn event(app: &App, state: &mut State, event: WindowEvent) {
  let win = app.window_rect();

  match event {
    MousePressed(_btn) => {
      if !state.game.playing {
        let x = ((app.mouse.x + (win.w() / 2.0)) / (BLOCK_WH as f32)) as usize;
        let y = ((app.mouse.y + (win.h() / 2.0)) / (BLOCK_WH as f32)) as usize;
        state.game.blocks[y][x] = !state.game.blocks[y][x];
      }
    },
    KeyReleased(key) => match key {
      VirtualKeyCode::Q => { app.quit() },
      _ => {},
    },
    _ => {},
  }
}