use prelude::*;

pub mod config;
pub mod game_logic;
pub mod life;
pub mod prelude;
pub mod savestates;
#[cfg(test)]
mod tests;
pub mod timing;
pub mod ui;
pub mod utils;

fn main() {
    nannou::app(model).update(update).view(view).run();
}
