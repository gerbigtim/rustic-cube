use crate::rubiks_core::{CornerCubie, CornerPiece, Cube, EdgeCubie, EdgePiece};

pub fn cross_solved(cube: &Cube) -> bool {
    cube.edges()[0] == EdgePiece::new(EdgeCubie::WB, 0)
        && cube.edges()[1] == EdgePiece::new(EdgeCubie::WR, 0)
        && cube.edges()[2] == EdgePiece::new(EdgeCubie::WG, 0)
        && cube.edges()[3] == EdgePiece::new(EdgeCubie::WO, 0)
}

pub fn bo_f2l_solved(cube: &Cube) -> bool {
    cube.corners()[0] == CornerPiece::new(CornerCubie::WOB, 0)
        && cube.edges()[4] == EdgePiece::new(EdgeCubie::BO, 0)
}

pub fn br_f2l_solved(cube: &Cube) -> bool {
    cube.corners()[1] == CornerPiece::new(CornerCubie::WBR, 0)
        && cube.edges()[5] == EdgePiece::new(EdgeCubie::BR, 0)
}

pub fn gr_f2l_solved(cube: &Cube) -> bool {
    cube.corners()[2] == CornerPiece::new(CornerCubie::WRG, 0)
        && cube.edges()[6] == EdgePiece::new(EdgeCubie::GR, 0)
}

pub fn go_f2l_solved(cube: &Cube) -> bool {
    cube.corners()[3] == CornerPiece::new(CornerCubie::WGO, 0)
        && cube.edges()[7] == EdgePiece::new(EdgeCubie::GO, 0)
}

pub fn f2l_solved(cube: &Cube) -> bool {
    bo_f2l_solved(cube) && br_f2l_solved(cube) && gr_f2l_solved(cube) && go_f2l_solved(cube)
}

#[cfg(test)]
mod test {
    use crate::rubiks_core::Cube;

    #[test]
    fn solved_cube_has_cross_solved() {
        assert!(super::cross_solved(&Cube::solved()));
    }
}
