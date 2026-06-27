use bevy::prelude::*;

use crate::{
    rendering::{animation::MoveAnimator, cube::CubeState},
    rubiks_core::CubeMove,
};

#[derive(Resource, Debug, Default)]
pub struct MoveLog {
    pub moves: Vec<CubeMove>,
    pub current_idx: Option<usize>,
}

impl MoveLog {
    pub fn push_move(&mut self, cube_move: CubeMove) -> usize {
        let idx = self.moves.len();
        self.moves.push(cube_move);
        idx
    }
}

#[derive(Component)]
pub struct MoveLogText;

fn format_move_log(move_log: &MoveLog, animator: &MoveAnimator) -> String {
    let mut text = String::new();

    text.push_str("Moves\n");

    for (idx, cube_move) in move_log.moves.iter().enumerate() {
        if Some(idx) == move_log.current_idx {
            text.push_str("> ");
        } else {
            text.push_str("  ");
        }
        text.push_str(&format!("{cube_move:?}\n"));
    }
    text.push_str("\nQueue\n");

    for queued_move in animator.queue.iter() {
        text.push_str(&format!("  {:?}\n", queued_move.cube_move));
    }

    text
}

#[derive(Component)]
pub struct MoveLogPanel;

pub fn setup_move_log_ui(mut commands: Commands) {
    commands
        .spawn((
            MoveLogPanel,
            Node {
                position_type: PositionType::Absolute,
                right: px(12),
                top: px(12),
                width: px(260),
                height: percent(80),
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(px(8)),
                ..default()
            },
            ScrollPosition::default(),
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.85)),
        ))
        .with_child((
            Text::new("Moves\n"),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(Color::WHITE),
            MoveLogText,
        ));
}

pub fn update_move_log_ui(
    move_log: Res<MoveLog>,
    animator: Res<MoveAnimator>,
    mut query: Query<&mut Text, With<MoveLogText>>,
) {
    if !move_log.is_changed() && !animator.is_changed() {
        return;
    }

    let mut text = query.single_mut().unwrap();
    *text = Text::new(format_move_log(&move_log, &animator));
}

pub fn apply_and_log_move(cube_move: CubeMove, move_log: &mut MoveLog, cube_state: &mut CubeState) {
    let log_idx = move_log.push_move(cube_move);
    move_log.current_idx = Some(log_idx);
    cube_state.cube.apply_move(cube_move);
}
