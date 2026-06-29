use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::rendering::animation::{MoveAnimator, QueuedMove};
use crate::rendering::cube::{CubeState, RubiksCubeRoot, create_3d_cube, despawn_cube};
use crate::rendering::log::{MoveLog, MoveLogPanel, apply_and_log_move};
use crate::rubiks_core::{Cube, CubeMove};
use crate::solver::solver::Solver;

pub fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cube_state: ResMut<CubeState>,
    mut commands: Commands,
    mut animator: ResMut<MoveAnimator>,
    mut move_log: ResMut<MoveLog>,
    cube_query: Query<Entity, With<RubiksCubeRoot>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
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
                    apply_and_log_move(CubeMove::UPrime, &mut move_log, &mut cube_state);
                } else if ctrl_pressed {
                    apply_and_log_move(CubeMove::U2, &mut move_log, &mut cube_state);
                } else {
                    apply_and_log_move(CubeMove::U, &mut move_log, &mut cube_state);
                }
            }
            KeyCode::KeyD => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    apply_and_log_move(CubeMove::DPrime, &mut move_log, &mut cube_state);
                } else if ctrl_pressed {
                    apply_and_log_move(CubeMove::D2, &mut move_log, &mut cube_state);
                } else {
                    apply_and_log_move(CubeMove::D, &mut move_log, &mut cube_state);
                }
            }
            KeyCode::KeyB => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    apply_and_log_move(CubeMove::BPrime, &mut move_log, &mut cube_state);
                } else if ctrl_pressed {
                    apply_and_log_move(CubeMove::B2, &mut move_log, &mut cube_state);
                } else {
                    apply_and_log_move(CubeMove::B, &mut move_log, &mut cube_state);
                }
            }
            KeyCode::KeyF => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    apply_and_log_move(CubeMove::FPrime, &mut move_log, &mut cube_state);
                } else if ctrl_pressed {
                    apply_and_log_move(CubeMove::F2, &mut move_log, &mut cube_state);
                } else {
                    apply_and_log_move(CubeMove::F, &mut move_log, &mut cube_state);
                }
            }
            KeyCode::KeyR => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    apply_and_log_move(CubeMove::RPrime, &mut move_log, &mut cube_state);
                } else if ctrl_pressed {
                    apply_and_log_move(CubeMove::R2, &mut move_log, &mut cube_state);
                } else {
                    apply_and_log_move(CubeMove::R, &mut move_log, &mut cube_state);
                }
            }
            KeyCode::KeyL => {
                if animation_busy {
                    continue;
                }
                cube_changed = true;
                if shift_pressed {
                    apply_and_log_move(CubeMove::LPrime, &mut move_log, &mut cube_state);
                } else if ctrl_pressed {
                    apply_and_log_move(CubeMove::L2, &mut move_log, &mut cube_state);
                } else {
                    apply_and_log_move(CubeMove::L, &mut move_log, &mut cube_state);
                }
            }
            KeyCode::KeyS => {
                if animation_busy {
                    continue;
                }
                cube_state.cube.make_solved();
                cube_changed = true;
                let mut scramble_moves = Cube::generate_scramble(50);
                if shift_pressed {
                    cube_state.cube.apply_moves(scramble_moves.clone());
                    move_log.moves.append(&mut scramble_moves);
                    move_log.current_idx = Some(move_log.moves.len() - 1);
                    continue;
                }
                for cube_move in scramble_moves {
                    let log_idx = move_log.push_move(cube_move);
                    animator.queue.push_back(QueuedMove { log_idx, cube_move });
                }
            }
            KeyCode::KeyM => {
                if animation_busy {
                    continue;
                }
                let mut solver = Solver::new(&cube_state.cube);
                let solve_trace = solver.solve();
                for cube_move in solve_trace {
                    let log_idx = move_log.push_move(cube_move);
                    animator.queue.push_back(QueuedMove { log_idx, cube_move });
                }
            }
            _ => continue,
        }
    }

    if cube_changed {
        despawn_cube(&mut commands, &cube_query);
        create_3d_cube(&cube_state.cube, &mut commands, meshes, materials);
    }
}

pub fn scroll_move_log_ui(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut query: Query<&mut ScrollPosition, With<MoveLogPanel>>,
) {
    let Ok(mut scroll_position) = query.single_mut() else {
        return;
    };

    for event in mouse_wheel_events.read() {
        let mut delta = event.y;

        if event.unit == MouseScrollUnit::Line {
            delta *= 21.0;
        }

        scroll_position.y -= delta;
    }
}
