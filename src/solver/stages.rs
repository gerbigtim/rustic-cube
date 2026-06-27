use crate::rubiks_core::{CornerCubie, CornerPiece, Cube, EdgeCubie, EdgePiece};

fn cross_solved(cube: &Cube) -> bool {
    cube.edges()[0] == EdgePiece::new(EdgeCubie::WB, 0)
        && cube.edges()[1] == EdgePiece::new(EdgeCubie::WR, 0)
        && cube.edges()[2] == EdgePiece::new(EdgeCubie::WG, 0)
        && cube.edges()[3] == EdgePiece::new(EdgeCubie::WO, 0)
}

#[cfg(test)]
mod test {
    use crate::rubiks_core::Cube;

    #[test]
    fn solved_cube_has_cross_solved() {
        assert!(super::cross_solved(&Cube::solved()));
    }
}
