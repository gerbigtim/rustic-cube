use std::collections::VecDeque;

use bevy::prelude::*;

use crate::rendering::animation::MoveAnimator;
use crate::rendering::cube::{CubeState, RubiksCubeRoot, create_3d_cube, despawn_cube};
use crate::rubiks_core::{Cube, CubeMove};

pub fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cube_state: ResMut<CubeState>,
    mut commands: Commands,
    mut animator: ResMut<MoveAnimator>,
    cube_query: Query<Entity, With<RubiksCubeRoot>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = &mut cube_state.cube;
    let mut cube_changed = false;
    let shift_pressed =
        keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let ctrl_pressed =
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let animation_busy = animator.active_move.is_some() || !animator.queue.is_empty();

    for key in keyboard.get_just_pressed() {
        match key {
            KeyCode::KeyU => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::UPrime);
                } else if ctrl_pressed {
                    cube.apply_move(CubeMove::U2);
                } else {
                    cube.apply_move(CubeMove::U);
                }
            }
            KeyCode::KeyD => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::DPrime);
                } else if ctrl_pressed {
                    cube.apply_move(CubeMove::D2);
                } else {
                    cube.apply_move(CubeMove::D);
                }
            }
            KeyCode::KeyB => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::BPrime);
                } else if ctrl_pressed {
                    cube.apply_move(CubeMove::B2);
                } else {
                    cube.apply_move(CubeMove::B);
                }
            }
            KeyCode::KeyF => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::FPrime);
                } else if ctrl_pressed {
                    cube.apply_move(CubeMove::F2);
                } else {
                    cube.apply_move(CubeMove::F);
                }
            }
            KeyCode::KeyR => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::RPrime);
                } else if ctrl_pressed {
                    cube.apply_move(CubeMove::R2);
                } else {
                    cube.apply_move(CubeMove::R);
                }
            }
            KeyCode::KeyL => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    cube.apply_move(CubeMove::LPrime);
                } else if ctrl_pressed {
                    cube.apply_move(CubeMove::L2);
                } else {
                    cube.apply_move(CubeMove::L);
                }
            }
            KeyCode::KeyS => {
                if animation_busy {
                    continue;
                }
                cube.make_solved();
                cube_changed = true;
                let scramble_moves = Cube::generate_scramble(50);
                animator.queue = VecDeque::from(scramble_moves);
            }
            _ => continue,
        }
    }

    if cube_changed {
        despawn_cube(&mut commands, &cube_query);
        create_3d_cube(cube, &mut commands, meshes, materials);
    }
}
