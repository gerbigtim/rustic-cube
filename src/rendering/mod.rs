mod camera;
pub mod cube;
pub mod scene;

use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cube::create_and_print_solved_cube);
    }
}
