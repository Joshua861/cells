//! The entry point of the program.
//!
//! Adds all the events and initializes the game state.

use crate::{prelude::*, ui::notify_info};
use clap::Parser;
use fps_ticker::Fps;
use grid::Grid;
use nannou::text::Font;

/// Struct which stores all the game state.
#[derive(Clone)]
pub struct Model {
    pub board: Board,
    pub paused: bool,
    pub pressed: Option<MouseButton>,
    pub last_mouse_pos: (f32, f32),
    pub last_mouse_pressed: Option<MouseButton>,
    pub cache: Cache,
    pub mouse_pos: (f32, f32),
    pub grid_lines: bool,
    pub symmetry: bool,
    pub show_info: bool,
    pub fps: Fps,
    pub font: Font,
    pub rulestring: String,
    pub selection: Option<Selection>,
    pub keybinds: String,
    pub show_keybinds: bool,
    pub clipboard: Option<Grid<bool>>,
}

impl Model {
    /// Estimates the time since the last frame.
    pub fn delta_time(&self) -> f32 {
        1. / self.fps.avg() as f32
    }
}

/// Struct which holds the arguments that can be passed to the program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Used to load a savestate with a given name when launching the program.
    #[arg(short, long)]
    load: Option<String>,

    /// Print out a savestate with a given name instead of launching the program, used for
    /// debugging.
    #[arg(short, long)]
    print: Option<String>,
}

/// Entry point function.
///
/// Sets everything up.
pub fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .resized(window_resized)
        .key_pressed(key_pressed)
        .mouse_moved(mouse_moved)
        .mouse_wheel(mouse_wheel)
        .build()
        .unwrap();

    let initial_tile_size = CONFIG.tile_size;

    let (mut board, width, height) = if CONFIG.autosize_board {
        let rect = app.window_rect();
        let width = (rect.w() / CONFIG.tile_size).ceil() as usize;
        let height = (rect.h() / CONFIG.tile_size).ceil() as usize;

        let board = Board::new(width, height);

        (board, width, height)
    } else {
        (
            Board::new(CONFIG.board_size.x, CONFIG.board_size.y),
            CONFIG.board_size.x,
            CONFIG.board_size.y,
        )
    };

    let args = Args::parse();
    let mut paused = false;

    if args.load.is_some() {
        board = load_savestate(args.load.unwrap());
        board.set_wh(width, height);
        paused = true;
        notify_info("Savestate loaded.");
    }

    if args.print.is_some() {
        let board = load_savestate(args.print.unwrap());
        board.print();
        app.quit();
    }

    let mut model = Model {
        board,
        paused,
        pressed: None,
        last_mouse_pos: (0., 0.),
        cache: Cache::new((width, height), initial_tile_size),
        mouse_pos: (0.0, 0.0),
        grid_lines: false,
        symmetry: false,
        last_mouse_pressed: None,
        show_info: false,
        fps: Fps::default(),
        font: load_font(),
        rulestring: CONFIG.rule.serialize(),
        selection: None,
        // This include_str! is a macro meaning that it runs at compile time, so once you've
        // compiled the program, this macro is replaced with whatever text is inside keybinds.txt
        // when you compiled, so it doesn't need the file to be run.
        keybinds: include_str!("../../assets/keybinds.txt").to_string(),
        show_keybinds: false,
        clipboard: None,
    };

    model.cache.update((width, height), CONFIG.tile_size);

    notify_info("Press K to show keybinds");

    model
}
