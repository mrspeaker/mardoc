mod game;
pub mod terrain;
pub mod nim;

use bevy::prelude::*;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
