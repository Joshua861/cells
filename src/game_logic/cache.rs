//! The Cache holds a lot of information about the state of the game.
//!
//! Mostly cached variables, stored here so we don't have to recompute them all the time. Though it
//! also has information about the camera_offset and zoom.

use crate::config::CONFIG;

/// Struct which stores the cached data.
#[derive(Clone)]
pub struct Cache {
    pub half_board_width: f32,
    pub half_board_height: f32,
    pub tile_size: f32,
    pub board_width: usize,
    pub board_height: usize,
    pub scale_factor: f32,
    pub camera_offset: (f32, f32),
    pub window_size: (f32, f32),
    pub target_tile_size: f32,
    pub target_camera_offset: (f32, f32),
}

impl Cache {
    /// Creates a new instance of the board, computing the variables based off some values.
    pub fn new(board_size: (usize, usize), tile_size: f32) -> Self {
        Self {
            half_board_width: board_size.0 as f32 / 2.,
            half_board_height: board_size.1 as f32 / 2.,
            tile_size,
            target_tile_size: tile_size,
            board_width: board_size.0,
            board_height: board_size.1,
            scale_factor: 1.,
            camera_offset: (0., 0.),
            window_size: (0., 0.),
            target_camera_offset: (0., 0.),
        }
    }
    /// Now only used when CONFIG.autosize_board is false.
    pub fn update(&mut self, board_size: (usize, usize), tile_size: f32) {
        self.half_board_width = board_size.0 as f32 / 2.;
        self.half_board_height = board_size.1 as f32 / 2.;
        self.tile_size = tile_size * 0.9;
        self.board_width = board_size.0;
        self.board_height = board_size.1;
        self.scale_factor = tile_size / CONFIG.tile_size;
    }
}
