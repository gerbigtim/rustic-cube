use crate::rubiks_core::Cube;
use crate::solver::stages::*;

pub fn score(cube: &Cube) -> isize {
    let mut score: isize = 0;

    score += 100 * pieces_solved(cube);

    if cross_solved(cube) {
        score += 10_000;
    }

    score
}

fn edges_solved(cube: &Cube) -> isize {
    let solved_cube = Cube::solved();
    let solved_edges = solved_cube.edges();

    cube.edges()
        .iter()
        .zip(solved_edges.iter())
        .filter(|(current, solved)| current == solved)
        .count() as isize
}

fn corners_solved(cube: &Cube) -> isize {
    let solved_cube = Cube::solved();
    let solved_corners = solved_cube.corners();

    cube.corners()
        .iter()
        .zip(solved_corners.iter())
        .filter(|(current, solved)| current == solved)
        .count() as isize
}

fn pieces_solved(cube: &Cube) -> usisize {
    edges_solved(cube) + corners_solved(cube)
}
