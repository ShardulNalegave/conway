
// ===== Imports =====
use nannou_egui::{egui, FrameCtx};
use crate::state::{GameState};
// ===================

pub fn render_ui(ctx: FrameCtx, game_state: &mut GameState) {
  egui::Window::new("Conway's Game of Life").show(&ctx, |ui| {
    if ui.button(if game_state.playing { "Pause" } else { "Play" }).clicked() {
      game_state.playing = !game_state.playing;
    }
  });
}