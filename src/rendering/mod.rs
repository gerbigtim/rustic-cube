pub mod animation;
mod camera;
pub mod cube;
pub mod input;
pub mod scene;

use bevy::prelude::*;

use crate::rubiks_core::Cube;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(cube::CubeState {
            cube: Cube::solved(),
        });
        app.insert_resource(animation::MoveAnimator::default());
        app.add_systems(Startup, cube::create_scene);
        app.add_systems(Update, input::handle_keyboard);
        app.add_systems(Update, animation::animate_moves);
    }
}
