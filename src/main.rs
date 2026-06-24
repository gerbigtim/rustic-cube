mod rendering;
mod rubiks_core;

fn main() {
    let cube = rubiks_core::Cube::solved();
    let stickers = rendering::cube::DerivedStickers::from_cube(&cube);
    stickers.print_cube_net();
}
