//! Handles the CONFIG global variable, loading from a TOML file, and providing default values.

use crate::prelude::*;
pub use color::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{env, fs};

pub mod color;

lazy_static! {
    /// Lazy_static const which holds an instance of the Config struct.
    pub static ref CONFIG: Config = Config::load();
}

/// Struct that holds all the configuration.
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub window_title: String,
    pub tile_size: f32,
    pub background_color: Color,
    pub void_color: Color,
    pub cell_color: Color,
    pub cell_color_highlighted: Color,
    pub background_color_highlighted: Color,
    pub zoom_speed: f32,
    pub grid_color: Color,
    pub grid_thickness: f32,
    pub scale_grid_with_zoom: bool,
    pub pan_speed: f32,
    pub text_color: Color,
    pub smoothing_factor: f32,
    pub rule: Rule,
    pub autosize_board: bool,
    pub board_size: VecU2,
    pub selection_color: Color,
    pub selection_thickness: f32,
    pub font_size: u32,
    pub window_color: Color,
    pub info_color: Color,
    pub error_color: Color,
}

impl Config {
    /// Load the configuration from config.toml.
    pub fn load() -> Config {
        let config_path = config_path();

        let text = fs::read_to_string(&config_path);

        if let Ok(text) = text {
            return toml::from_str(text.as_str()).unwrap_or(Config::default());
        } else {
            notify_info("Failed to read config file; using default values.");

            #[allow(deprecated)]
            let path = config_path
                .split('/')
                .filter(|s| !s.contains(".toml"))
                .collect::<Vec<&str>>()
                .join("/")
                .replace('~', env::home_dir().unwrap().to_str().unwrap());

            let _ = fs::create_dir_all(path);

            let text = Config::default().to_toml();

            fs::write(config_path, text).expect("Failed to write default values to config file");

            Config::default()
        }
    }

    /// Utility function to convert the Config to a TOML string.
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}

fn config_path() -> String {
    let path = BASE_DIR.to_string() + "/config.toml";
    dbg!(&path);

    path
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: String::from("Game of Life"),
            tile_size: 10.0,
            background_color: Color::new(0.1, 0.1, 0.1),
            cell_color: Color::new(0.9, 0.9, 0.9),
            cell_color_highlighted: Color::new(0.8, 0.8, 0.8),
            background_color_highlighted: Color::new(0.2, 0.2, 0.2),
            zoom_speed: 1.0,
            grid_color: Color::new(0.2, 0.2, 0.2),
            grid_thickness: 1.0,
            scale_grid_with_zoom: false,
            pan_speed: 5.,
            void_color: Color::new(0.08, 0.08, 0.08),
            text_color: Color::new(0.95, 0.95, 0.95),
            smoothing_factor: 3.0,
            rule: "23/3".into(),
            autosize_board: false,
            board_size: VecU2::new(100, 100),
            selection_color: Color::hex(0x4ba4f2),
            selection_thickness: 4.0,
            font_size: 24,
            window_color: Color::new(0.2, 0.2, 0.2),
            info_color: Color::hex(0x51aee9),
            error_color: Color::hex(0xcc6b70),
        }
    }
}
