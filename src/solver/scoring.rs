use crate::rubiks_core::Cube;
use crate::solver::stages::*;

pub fn score(cube: &Cube) -> usize {
    let mut score: usize = 0;

    score += 100 * pieces_solved(cube);

    if cross_solved(cube) {
        score += 10_000;
    }

    score
}

fn edges_solved(cube: &Cube) -> usize {
    let solved_cube = Cube::solved();
    let solved_edges = solved_cube.edges();

    cube.edges()
        .iter()
        .zip(solved_edges.iter())
        .filter(|(current, solved)| current == solved)
        .count()
}

fn corners_solved(cube: &Cube) -> usize {
    let solved_cube = Cube::solved();
    let solved_corners = solved_cube.corners();

    cube.corners()
        .iter()
        .zip(solved_corners.iter())
        .filter(|(current, solved)| current == solved)
        .count()
}

fn pieces_solved(cube: &Cube) -> usize {
    edges_solved(cube) + corners_solved(cube)
}
