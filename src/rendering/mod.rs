mod camera;
pub mod cube;
pub mod scene;

use bevy::prelude::*;

use crate::rubiks_core::Cube;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(cube::CubeState {
            cube: Cube::solved(),
        })
        .add_systems(Startup, cube::create_scene);
    }
}
