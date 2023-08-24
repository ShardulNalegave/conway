
// ===== Imports =====
use nannou_egui::Egui;
// ===================

pub const SCREEN_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 800;
pub const BLOCK_WH: u32 = 40;
pub const NUM_BLOCKS_WIDTH: u32 = SCREEN_WIDTH / BLOCK_WH;
pub const NUM_BOCKS_HEIGHT: u32 = SCREEN_HEIGHT / BLOCK_WH;

pub struct State {
  pub egui: Egui,
  pub game: GameState,
}

impl State {
  pub fn new(egui: Egui) -> Self {
    Self {
      egui,
      game: GameState::new(),
    }
  }
}

pub struct GameState {
  pub blocks: [[bool; NUM_BLOCKS_WIDTH as usize]; NUM_BOCKS_HEIGHT as usize],
  pub playing: bool,
}

impl GameState {
  pub fn new() -> Self {
    Self {
      blocks: [[false; NUM_BLOCKS_WIDTH as usize]; NUM_BOCKS_HEIGHT as usize],
      playing: false,
    }
  }
}