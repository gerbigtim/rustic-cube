mod rendering;
mod rubiks_core;
mod solver;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rendering::RenderingPlugin)
        .run();
}
