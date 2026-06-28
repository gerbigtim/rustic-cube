use crate::rubiks_core::{Cube, CubeError, CubeMove};

pub struct Solver {
    initial_cube: Cube,
    cube: Cube,
    moves: [CubeMove; 18],
    solve_trace: Vec<CubeMove>,
}

impl Solver {
    pub fn new(initial_cube_ref: &Cube) -> Self {
        Self {
            initial_cube: initial_cube_ref.clone(),
            cube: Cube::solved(),
            moves: CubeMove::ALL,
            solve_trace: Vec::new(),
        }
    }

    fn solve(&self) {}
}

struct SolverNode {
    cube: Cube,
    move_mask: [bool; 18],
    solve_trace: Vec<CubeMove>,
}

impl SolverNode {
    fn new(cube: Cube, move_mask: [bool; 18], solve_trace: Vec<CubeMove>) -> Self {
        Self {
            cube,
            move_mask,
            solve_trace,
        }
    }

    fn block_and_unblock_moves(&mut self, cube_move: CubeMove) {
        let block_idxs: [usize; 3];
        let ignore_idxs: [usize; 3];
        match cube_move {
            CubeMove::U | CubeMove::UPrime | CubeMove::U2 => {
                block_idxs = [0, 1, 2];
                ignore_idxs = [3, 4, 5];
            }
            CubeMove::D | CubeMove::DPrime | CubeMove::D2 => {
                block_idxs = [3, 4, 5];
                ignore_idxs = [0, 1, 2];
            }
            CubeMove::B | CubeMove::BPrime | CubeMove::B2 => {
                block_idxs = [6, 7, 8];
                ignore_idxs = [9, 10, 11];
            }
            CubeMove::F | CubeMove::FPrime | CubeMove::F2 => {
                block_idxs = [9, 10, 11];
                ignore_idxs = [6, 7, 8];
            }
            CubeMove::R | CubeMove::RPrime | CubeMove::R2 => {
                block_idxs = [12, 13, 14];
                ignore_idxs = [15, 16, 17];
            }
            CubeMove::L | CubeMove::LPrime | CubeMove::L2 => {
                block_idxs = [15, 16, 17];
                ignore_idxs = [12, 13, 14];
            }
        }

        for idx in 0..self.move_mask.len() {
            if block_idxs.contains(&idx) {
                self.move_mask[idx] = false;
            } else if ignore_idxs.contains(&idx) {
                continue;
            } else {
                self.move_mask[idx] = true;
            }
        }
    }

    fn get_available_moves(&self) -> Vec<CubeMove> {
        CubeMove::ALL
            .into_iter()
            .zip(self.move_mask)
            .filter_map(
                |(cube_move, is_available)| {
                    if is_available { Some(cube_move) } else { None }
                },
            )
            .collect()
    }
}
