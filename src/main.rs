use bevy::prelude::*;

pub mod game;

use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin)
        .run();
}