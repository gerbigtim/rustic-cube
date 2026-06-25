use bevy::{
    color::{
        Color,
        palettes::css::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::*,
};

use crate::rubiks_core::{Cube, StickerColor};

pub struct DerivedStickers {
    stickers: [[StickerColor; 9]; 6],
}

impl DerivedStickers {
    pub fn from_cube(cube: &Cube) -> Self {
        let mut stickers = [[StickerColor::White; 9]; 6];
        //White Face
        stickers[0] = [
            cube.corners()[0].get_sticker_colors()[0],
            cube.edges()[0].get_sticker_colors()[0],
            cube.corners()[1].get_sticker_colors()[0],
            cube.edges()[3].get_sticker_colors()[0],
            StickerColor::White,
            cube.edges()[1].get_sticker_colors()[0],
            cube.corners()[3].get_sticker_colors()[0],
            cube.edges()[2].get_sticker_colors()[0],
            cube.corners()[2].get_sticker_colors()[0],
        ];
        //Yellow Face
        stickers[1] = [
            cube.corners()[4].get_sticker_colors()[0],
            cube.edges()[8].get_sticker_colors()[0],
            cube.corners()[5].get_sticker_colors()[0],
            cube.edges()[11].get_sticker_colors()[0],
            StickerColor::Yellow,
            cube.edges()[9].get_sticker_colors()[0],
            cube.corners()[7].get_sticker_colors()[0],
            cube.edges()[10].get_sticker_colors()[0],
            cube.corners()[6].get_sticker_colors()[0],
        ];
        //Blue Face
        stickers[2] = [
            cube.corners()[7].get_sticker_colors()[1],
            cube.edges()[10].get_sticker_colors()[1],
            cube.corners()[6].get_sticker_colors()[2],
            cube.edges()[4].get_sticker_colors()[0],
            StickerColor::Blue,
            cube.edges()[5].get_sticker_colors()[0],
            cube.corners()[0].get_sticker_colors()[2],
            cube.edges()[0].get_sticker_colors()[1],
            cube.corners()[1].get_sticker_colors()[1],
        ];
        //Green Face
        stickers[3] = [
            cube.corners()[3].get_sticker_colors()[1],
            cube.edges()[2].get_sticker_colors()[1],
            cube.corners()[2].get_sticker_colors()[2],
            cube.edges()[7].get_sticker_colors()[0],
            StickerColor::Green,
            cube.edges()[6].get_sticker_colors()[0],
            cube.corners()[4].get_sticker_colors()[2],
            cube.edges()[8].get_sticker_colors()[1],
            cube.corners()[5].get_sticker_colors()[1],
        ];
        //Red Face
        stickers[4] = [
            cube.corners()[2].get_sticker_colors()[1],
            cube.edges()[1].get_sticker_colors()[1],
            cube.corners()[1].get_sticker_colors()[2],
            cube.edges()[6].get_sticker_colors()[1],
            StickerColor::Red,
            cube.edges()[5].get_sticker_colors()[1],
            cube.corners()[5].get_sticker_colors()[2],
            cube.edges()[9].get_sticker_colors()[1],
            cube.corners()[6].get_sticker_colors()[1],
        ];
        //Orange Face
        stickers[5] = [
            cube.corners()[0].get_sticker_colors()[1],
            cube.edges()[3].get_sticker_colors()[1],
            cube.corners()[3].get_sticker_colors()[2],
            cube.edges()[4].get_sticker_colors()[1],
            StickerColor::Orange,
            cube.edges()[7].get_sticker_colors()[1],
            cube.corners()[7].get_sticker_colors()[2],
            cube.edges()[11].get_sticker_colors()[1],
            cube.corners()[4].get_sticker_colors()[1],
        ];
        Self { stickers }
    }

    pub fn print_cube_net(&self) {
        let mut sticker_chars = [['-'; 9]; 6];
        for (face_idx, face) in self.stickers.iter().enumerate() {
            for (sticker_idx, sticker_color) in face.iter().enumerate() {
                match *sticker_color {
                    StickerColor::White => sticker_chars[face_idx][sticker_idx] = 'W',
                    StickerColor::Yellow => sticker_chars[face_idx][sticker_idx] = 'Y',
                    StickerColor::Blue => sticker_chars[face_idx][sticker_idx] = 'B',
                    StickerColor::Green => sticker_chars[face_idx][sticker_idx] = 'G',
                    StickerColor::Red => sticker_chars[face_idx][sticker_idx] = 'R',
                    StickerColor::Orange => sticker_chars[face_idx][sticker_idx] = 'O',
                }
            }
        }
        println!(
            "      {} {} {}      ",
            sticker_chars[0][0], sticker_chars[0][1], sticker_chars[0][2]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[0][3], sticker_chars[0][4], sticker_chars[0][5]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[0][6], sticker_chars[0][7], sticker_chars[0][8]
        );
        print!(
            "{} {} {} ",
            sticker_chars[5][0], sticker_chars[5][1], sticker_chars[5][2]
        );
        print!(
            "{} {} {} ",
            sticker_chars[3][0], sticker_chars[3][1], sticker_chars[3][2]
        );
        print!(
            "{} {} {} ",
            sticker_chars[4][0], sticker_chars[4][1], sticker_chars[4][2]
        );
        println!();
        print!(
            "{} {} {} ",
            sticker_chars[5][3], sticker_chars[5][4], sticker_chars[5][5]
        );
        print!(
            "{} {} {} ",
            sticker_chars[3][3], sticker_chars[3][4], sticker_chars[3][5]
        );
        print!(
            "{} {} {} ",
            sticker_chars[4][3], sticker_chars[4][4], sticker_chars[4][5]
        );
        println!();
        print!(
            "{} {} {} ",
            sticker_chars[5][6], sticker_chars[5][7], sticker_chars[5][8]
        );
        print!(
            "{} {} {} ",
            sticker_chars[3][6], sticker_chars[3][7], sticker_chars[3][8]
        );
        print!(
            "{} {} {} ",
            sticker_chars[4][6], sticker_chars[4][7], sticker_chars[4][8]
        );
        println!();
        println!(
            "      {} {} {}      ",
            sticker_chars[1][0], sticker_chars[1][1], sticker_chars[1][2]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[1][3], sticker_chars[1][4], sticker_chars[1][5]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[1][6], sticker_chars[1][7], sticker_chars[1][8]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[2][0], sticker_chars[2][1], sticker_chars[2][2]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[2][3], sticker_chars[2][4], sticker_chars[2][5]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[2][6], sticker_chars[2][7], sticker_chars[2][8]
        );
    }
}

fn sticker_to_material(
    sticker_color: &StickerColor,
    materials: &mut Assets<StandardMaterial>,
) -> MeshMaterial3d<StandardMaterial> {
    match *sticker_color {
        StickerColor::White => MeshMaterial3d(materials.add(Color::from(WHITE))),
        StickerColor::Yellow => MeshMaterial3d(materials.add(Color::from(YELLOW))),
        StickerColor::Blue => MeshMaterial3d(materials.add(Color::from(BLUE))),
        StickerColor::Green => MeshMaterial3d(materials.add(Color::from(GREEN))),
        StickerColor::Red => MeshMaterial3d(materials.add(Color::from(RED))),
        StickerColor::Orange => MeshMaterial3d(materials.add(Color::from(ORANGE))),
    }
}

pub fn create_and_print_solved_cube() {
    let cube = Cube::solved();
    let stickers = DerivedStickers::from_cube(&cube);
    stickers.print_cube_net();
}

#[derive(Resource)]
pub struct CubeState {
    pub cube: Cube,
}

pub fn create_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cube_state: Res<CubeState>,
) {
    create_3d_cube(&cube_state.cube, &mut commands, meshes, materials);

    commands.spawn((PointLight::default(), Transform::from_xyz(10.0, 10.0, 10.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

#[derive(Component)]
struct RubiksCubeRoot;

#[derive(Component)]
struct CubeFace {
    idx: usize,
}
#[derive(Component)]
struct Facelet {
    idx: usize,
}

fn create_3d_cube(
    cube: &Cube,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Cuboid::new(1.0, 0.1, 1.0));
    let stickers = DerivedStickers::from_cube(cube).stickers;

    commands
        .spawn((RubiksCubeRoot, Transform::default(), Visibility::default()))
        .with_children(|cube| {
            for face_idx in 0..6 {
                cube.spawn((
                    CubeFace { idx: face_idx },
                    Transform::default(),
                    Visibility::default(),
                ))
                .with_children(|face| {
                    for sticker_idx in 0..9 {
                        let color = stickers[face_idx][sticker_idx];
                        let material = sticker_to_material(&color, &mut materials);
                        let transform = facelet_transform(face_idx, sticker_idx);

                        face.spawn((
                            Facelet { idx: sticker_idx },
                            Mesh3d(mesh_handle.clone()),
                            material,
                            transform,
                        ));
                    }
                });
            }
        });
}

fn facelet_transform(face_idx: usize, sticker_idx: usize) -> Transform {
    let sticker_gap = 0.2;
    let sticker_thickness = 0.1;
    let sticker_size = 1.0;
    let sticker_spacing = sticker_size + sticker_gap;
    let face_offset = 1.5 * sticker_size + 2.0 * sticker_gap + 0.5 * sticker_thickness;

    match face_idx {
        // White
        0 => {
            let x: f32 = (sticker_idx as isize % 3 - 1) as f32 * sticker_spacing;
            let y: f32 = face_offset;
            let z: f32 = (sticker_idx as isize / 3 - 1) as f32 * sticker_spacing;
            Transform::from_xyz(x, y, z)
        }
        // Yellow
        1 => {
            let x: f32 = (sticker_idx as isize % 3 - 1) as f32 * sticker_spacing;
            let y: f32 = -face_offset;
            let z: f32 = (1 - sticker_idx as isize / 3) as f32 * sticker_spacing;
            Transform::from_xyz(x, y, z)
        }
        // Blue
        2 => {
            let x: f32 = (sticker_idx as isize % 3 - 1) as f32 * sticker_spacing;
            let y: f32 = (sticker_idx as isize / 3 - 1) as f32 * sticker_spacing;
            let z: f32 = -face_offset;
            Transform::from_xyz(x, y, z)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
        }
        // Green
        3 => {
            let x: f32 = (sticker_idx as isize % 3 - 1) as f32 * sticker_spacing;
            let y: f32 = (1 - sticker_idx as isize / 3) as f32 * sticker_spacing;
            let z: f32 = face_offset;
            Transform::from_xyz(x, y, z)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
        }
        // Red
        4 => {
            let x: f32 = face_offset;
            let y: f32 = (1 - sticker_idx as isize / 3) as f32 * sticker_spacing;
            let z: f32 = (1 - sticker_idx as isize % 3) as f32 * sticker_spacing;
            Transform::from_xyz(x, y, z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
        }
        // Orange
        5 => {
            let x: f32 = -face_offset;
            let y: f32 = (1 - sticker_idx as isize / 3) as f32 * sticker_spacing;
            let z: f32 = (sticker_idx as isize % 3 - 1) as f32 * sticker_spacing;
            Transform::from_xyz(x, y, z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
        }
        _ => Transform::default(),
    }
}
