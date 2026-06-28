use bevy::platform::collections::HashSet;

use crate::{
    rubiks_core::{Cube, CubeError, CubeMove},
    solver::scoring::score,
};

pub struct Solver {
    initial_cube: Cube,
    cube: Cube,
    solve_trace: Vec<CubeMove>,
    visited: HashSet<Cube>,
}

struct CubeSolution {
    solve_trace: Vec<CubeMove>,
    score: isize,
}

impl Solver {
    pub fn new(initial_cube_ref: &Cube) -> Self {
        Self {
            initial_cube: initial_cube_ref.clone(),
            cube: Cube::solved(),
            solve_trace: Vec::new(),
            visited: HashSet::from([initial_cube_ref.clone()]),
        }
    }

    fn solve(&mut self) {
        let initial_node = SolverNode::new(
            self.cube.clone(),
            [true; 18],
            self.solve_trace.clone(),
            8,
            score(&self.cube),
        );

        let mut best_solution = self.solve_recursive(initial_node);
    }

    fn solve_recursive(&mut self, node: SolverNode) -> CubeSolution {
        let mut best_solution = CubeSolution {
            solve_trace: node.solve_trace.clone(),
            score: node.score,
        };
        if node.depth <= 0 {
            return best_solution;
        }
        for cube_move in node.get_available_moves() {
            let child = node.apply_move(cube_move);

            if self.try_claim_cube(&child.cube) {
                let child_solution = self.solve_recursive(child);
                if child_solution.score > best_solution.score {
                    best_solution = child_solution;
                }
            }
        }
        best_solution
    }

    fn try_claim_cube(&mut self, cube: &Cube) -> bool {
        self.visited.insert(cube.clone())
    }
}

struct SolverNode {
    cube: Cube,
    move_mask: [bool; 18],
    solve_trace: Vec<CubeMove>,
    depth: usize,
    score: isize,
}

impl SolverNode {
    fn new(
        cube: Cube,
        move_mask: [bool; 18],
        solve_trace: Vec<CubeMove>,
        depth: usize,
        score: isize,
    ) -> Self {
        Self {
            cube,
            move_mask,
            solve_trace,
            depth,
            score,
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

    fn apply_move(&self, cube_move: CubeMove) -> Self {
        let mut cube = self.cube.clone();
        cube.apply_move(cube_move);
        let mut solve_trace = self.solve_trace.clone();
        solve_trace.push(cube_move);
        let score = score(&cube);
        Self {
            cube,
            move_mask: self.move_mask,
            solve_trace,
            depth: self.depth - 1,
            score,
        }
    }
}
