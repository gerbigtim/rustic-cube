use bevy::prelude::*;

use crate::rendering::cube::{CubeState, RubiksCubeRoot, create_3d_cube, despawn_cube};
use crate::rubiks_core::CubeMove;

pub fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cube_state: ResMut<CubeState>,
    mut commands: Commands,
    mut cube_query: Query<Entity, With<RubiksCubeRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = &mut cube_state.cube;
    let mut cube_changed = false;
    let shift_pressed =
        keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    for key in keyboard.get_just_pressed() {
        match key {
            KeyCode::KeyU => {
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::UPrime);
                } else {
                    cube.apply_move(CubeMove::U);
                }
            }
            KeyCode::KeyD => {
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::DPrime);
                } else {
                    cube.apply_move(CubeMove::D);
                }
            }
            _ => continue,
        }
    }

    if cube_changed {
        despawn_cube(&mut commands, &cube_query);
        create_3d_cube(cube, &mut commands, meshes, materials);
    }
}
