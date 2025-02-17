mod game;
pub mod terrain;
pub mod nim;
pub mod person;
pub mod player;
pub mod town;
pub mod townsfolk;
pub mod ui;
pub mod bob;
pub mod inventory;
pub mod hotbar;

use bevy::prelude::*;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
