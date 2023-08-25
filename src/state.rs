
// ===== Imports =====
// ===================

pub const SCREEN_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 800;
pub const BLOCK_WH: f32 = 40.0;
pub const NUM_BLOCKS_WIDTH: usize = (SCREEN_WIDTH / BLOCK_WH as u32) as usize;
pub const NUM_BOCKS_HEIGHT: usize = (SCREEN_HEIGHT / BLOCK_WH as u32) as usize;

pub struct GameState {
  pub blocks: [[u8; NUM_BLOCKS_WIDTH]; NUM_BOCKS_HEIGHT],
  pub show_neighbours: bool,
}

impl GameState {
  pub fn new() -> Self {
    Self {
      blocks: [[0; NUM_BLOCKS_WIDTH]; NUM_BOCKS_HEIGHT],
      show_neighbours: false,
    }
  }

  pub fn next_generation(&mut self) {
    let mut new_blocks = [[0; NUM_BLOCKS_WIDTH]; NUM_BOCKS_HEIGHT];

    for y in 0..NUM_BOCKS_HEIGHT {
      for x in 0..NUM_BLOCKS_WIDTH {
        new_blocks[y][x] = if self.block_should_live(x, y) { 1 } else { 0 };
      }
    }

    self.blocks = new_blocks;
  }

  fn block_should_live(&self, x: usize, y: usize) -> bool {
    let live_neighbours = self.get_num_live_neighbours(x, y);
    if self.blocks[y][x] == 1 {
      (2..=3).contains(&live_neighbours)
    } else {
      live_neighbours == 3
    }
  }

  pub fn get_num_live_neighbours(&self, x: usize, y: usize) -> u8 {
    if x == 0 && y == 0 { // bottom-left
      let up = self.blocks[y + 1][x];
      let right = self.blocks[y][x + 1];
      let top_right = self.blocks[y + 1][x + 1];
      up + right + top_right
    } else if x == 0 && y == (NUM_BOCKS_HEIGHT - 1) { // top-left
      let down = self.blocks[y - 1][x];
      let right = self.blocks[y][x + 1];
      let bottom_right = self.blocks[y - 1][x + 1];
      down + right + bottom_right
    } else if x == (NUM_BLOCKS_WIDTH - 1) && y == (NUM_BOCKS_HEIGHT - 1) { // top-right
      let down = self.blocks[y - 1][x];
      let left = self.blocks[y][x - 1];
      let bottom_left = self.blocks[y - 1][x - 1];
      down + left + bottom_left
    } else if x == (NUM_BLOCKS_WIDTH - 1) && y == 0 { // bottom-right
      let up = self.blocks[y + 1][x];
      let left = self.blocks[y][x - 1];
      let top_left = self.blocks[y + 1][x - 1];
      up + left + top_left
    } else if x == 0 { // left-edge
      let up = self.blocks[y + 1][x];
      let down = self.blocks[y - 1][x];
      let right = self.blocks[y][x + 1];
      let top_right = self.blocks[y + 1][x + 1];
      let bottom_right = self.blocks[y - 1][x + 1];
      up + down + right + top_right + bottom_right
    } else if x == (NUM_BLOCKS_WIDTH - 1) { // right-edge
      let up = self.blocks[y + 1][x];
      let down = self.blocks[y - 1][x];
      let left = self.blocks[y][x - 1];
      let top_left = self.blocks[y + 1][x - 1];
      let bottom_left = self.blocks[y - 1][x - 1];
      up + down + left + top_left + bottom_left
    } else if y == 0 { // bottom-edge
      let up = self.blocks[y + 1][x];
      let left = self.blocks[y][x - 1];
      let right = self.blocks[y][x + 1];
      let top_left = self.blocks[y + 1][x - 1];
      let top_right = self.blocks[y + 1][x + 1];
      up + left + right + top_left + top_right
    } else if y == (NUM_BOCKS_HEIGHT - 1) { // top-edge
      let down = self.blocks[y - 1][x];
      let left = self.blocks[y][x - 1];
      let right = self.blocks[y][x + 1];
      let bottom_left = self.blocks[y - 1][x - 1];
      let bottom_right = self.blocks[y - 1][x + 1];
      down + left + right + bottom_left + bottom_right
    } else { // non-boundary block
      let up = self.blocks[y + 1][x];
      let down = self.blocks[y - 1][x];
      let left = self.blocks[y][x - 1];
      let right = self.blocks[y][x + 1];
      let top_left = self.blocks[y + 1][x - 1];
      let top_right = self.blocks[y + 1][x + 1];
      let bottom_left = self.blocks[y - 1][x - 1];
      let bottom_right = self.blocks[y - 1][x + 1];
      up + down + left + right + top_left + top_right + bottom_left + bottom_right
    }
  }
}