use std::collections::VecDeque;

use bevy::prelude::*;

use crate::rendering::cube::{CubeState, RubiksCubeRoot, create_3d_cube, despawn_cube};
use crate::rubiks_core::CubeMove;

#[derive(Resource, Debug)]
pub struct MoveAnimator {
    pub queue: VecDeque<CubeMove>,
    pub active_move: Option<CubeMove>,
    timer: Timer,
}

impl Default for MoveAnimator {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            active_move: None,
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

pub fn animate_moves(
    time: Res<Time>,
    mut animator: ResMut<MoveAnimator>,
    mut cube_state: ResMut<CubeState>,
    mut commands: Commands,
    cube_query: Query<Entity, With<RubiksCubeRoot>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if animator.active_move.is_none() {
        animator.active_move = animator.queue.pop_front();
        if animator.active_move.is_none() {
            return;
        }
        animator.timer.reset();
    }
    if animator.timer.tick(time.delta()).just_finished() {
        let Some(cube_move) = animator.active_move else {
            return;
        };
        cube_state.cube.apply_move(cube_move);
        animator.active_move = None;
        despawn_cube(&mut commands, &cube_query);
        create_3d_cube(&cube_state.cube, &mut commands, meshes, materials);
    }
}
