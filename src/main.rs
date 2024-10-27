mod game;
mod hello;

use bevy::prelude::*;

use hello::HelloPlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
      //  .add_plugins(HelloPlugin)
        .run();
}
