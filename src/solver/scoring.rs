use crate::rubiks_core::Cube;
use crate::solver::stages::*;

pub fn score(cube: &Cube) -> isize {
    let mut score: isize = 0;

    score += 10 * pieces_solved(cube);

    if cross_solved(cube) {
        score += 100_000;
    }
    if bo_f2l_solved(cube) {
        score += 1000
    }
    if br_f2l_solved(cube) {
        score += 1000
    }
    if gr_f2l_solved(cube) {
        score += 1000
    }
    if go_f2l_solved(cube) {
        score += 1000
    }
    if f2l_solved(cube) {
        score += 10_000
    }
    if oll_edges_solved(cube) {
        score += 5000
    }
    if oll_corners_solved(cube) {
        score += 1000
    }
    if pll_edges_solved(cube) {
        score += 500
    }
    if pll_corners_solved(cube) {
        score += 100
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

fn pieces_solved(cube: &Cube) -> isize {
    edges_solved(cube) + corners_solved(cube)
}
