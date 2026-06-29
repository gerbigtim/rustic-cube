use std::io::{self, Write};

use bevy::platform::collections::HashSet;

use crate::{
    rubiks_core::{Cube, CubeError, CubeMove},
    solver::scoring::score,
};

const RECURSION_DEPTH: usize = 6;

pub struct Solver {
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
            cube: initial_cube_ref.clone(),
            solve_trace: Vec::new(),
            visited: HashSet::from([initial_cube_ref.clone()]),
        }
    }

    pub fn solve(&mut self) -> Vec<CubeMove> {
        let mut node = SolverNode::new(
            self.cube.clone(),
            [true; 18],
            Vec::new(),
            RECURSION_DEPTH,
            score(&self.cube),
        );

        let mut best_solution = CubeSolution {
            solve_trace: node.local_trace.clone(),
            score: node.score,
        };

        println!("solver:");
        println!("node:");

        let mut found_improvement = true;

        while found_improvement {
            found_improvement = false;
            node.reset_trace();
            let found_solution = self.solve_recursive(&node);
            if found_solution.score > best_solution.score {
                found_improvement = true;
                best_solution = found_solution;
                self.solve_trace
                    .append(&mut best_solution.solve_trace.clone());
                node.apply_moves(&best_solution.solve_trace);
            }
        }
        self.solve_trace.clone()
    }

    fn solve_recursive(&mut self, node: &SolverNode) -> CubeSolution {
        let mut best_solution = CubeSolution {
            solve_trace: node.local_trace.clone(),
            score: node.score,
        };
        print_progress(&self.solve_trace, &node.local_trace);
        if node.depth == 0 {
            return best_solution;
        }
        for cube_move in node.get_available_moves() {
            let child = node.child_from_move(cube_move);

            if self.try_claim_cube(&child.cube) {
                let child_solution = self.solve_recursive(&child);
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
    local_trace: Vec<CubeMove>,
    depth: usize,
    score: isize,
}

impl SolverNode {
    fn new(
        cube: Cube,
        move_mask: [bool; 18],
        local_trace: Vec<CubeMove>,
        depth: usize,
        score: isize,
    ) -> Self {
        Self {
            cube,
            move_mask,
            local_trace,
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

    fn child_from_move(&self, cube_move: CubeMove) -> Self {
        let mut cube = self.cube.clone();
        cube.apply_move(cube_move);
        let mut local_trace = self.local_trace.clone();
        local_trace.push(cube_move);
        let score = score(&cube);
        let mut child = Self {
            cube,
            move_mask: self.move_mask,
            local_trace,
            depth: self.depth - 1,
            score,
        };
        child.block_and_unblock_moves(cube_move);
        child
    }

    fn apply_moves(&mut self, cube_moves: &[CubeMove]) {
        self.cube.apply_moves(cube_moves.to_vec());
        self.score = score(&self.cube);
        self.move_mask = [true; 18];
        self.local_trace.append(&mut cube_moves.to_vec());
        let last_moves = last_up_to_two(&self.local_trace);
        if let Some(last_moves) = last_moves {
            for cube_move in last_moves {
                self.block_and_unblock_moves(cube_move);
            }
        }
    }

    fn reset_trace(&mut self) {
        self.local_trace = Vec::new();
    }

    fn reset_depth(&mut self, depth: usize) {
        self.depth = depth;
    }
}

fn last_up_to_two(items: &[CubeMove]) -> Option<Vec<CubeMove>> {
    match items.len() {
        0 => None,
        1 => Some(vec![items[0]]),
        len => Some(items[len - 2..].to_vec()),
    }
}

fn print_progress(solver_trace: &[CubeMove], node_trace: &[CubeMove]) {
    let mut stdout = io::stdout();

    write!(stdout, "\x1B[2A").unwrap();

    write!(stdout, "\x1B[2K").unwrap();
    writeln!(stdout, "solver: {:?}", solver_trace);

    write!(stdout, "\x1B[2K").unwrap();
    writeln!(stdout, "node: {:?}", node_trace);

    stdout.flush().unwrap();
}
