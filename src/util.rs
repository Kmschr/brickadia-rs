use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::save::{Brick, Direction, Rotation, Size};

pub const ROTATION_TABLE: [u8; 576] = [
    16, 15, 22, 9, 18, 11, 20, 13, 17, 3, 21, 5, 19, 7, 23, 1, 0, 8, 4, 12, 6, 10, 2, 14, 17, 12,
    23, 10, 19, 8, 21, 14, 18, 0, 22, 6, 16, 4, 20, 2, 1, 9, 5, 13, 7, 11, 3, 15, 18, 13, 20, 11,
    16, 9, 22, 15, 19, 1, 23, 7, 17, 5, 21, 3, 2, 10, 6, 14, 4, 8, 0, 12, 19, 14, 21, 8, 17, 10,
    23, 12, 16, 2, 20, 4, 18, 6, 22, 0, 3, 11, 7, 15, 5, 9, 1, 13, 22, 9, 16, 15, 20, 13, 18, 11,
    21, 5, 17, 3, 23, 1, 19, 7, 4, 12, 0, 8, 2, 14, 6, 10, 23, 10, 17, 12, 21, 14, 19, 8, 22, 6,
    18, 0, 20, 2, 16, 4, 5, 13, 1, 9, 3, 15, 7, 11, 20, 11, 18, 13, 22, 15, 16, 9, 23, 7, 19, 1,
    21, 3, 17, 5, 6, 14, 2, 10, 0, 12, 4, 8, 21, 8, 19, 14, 23, 12, 17, 10, 20, 4, 16, 2, 22, 0,
    18, 6, 7, 15, 3, 11, 1, 13, 5, 9, 15, 22, 9, 16, 11, 20, 13, 18, 3, 21, 5, 17, 7, 23, 1, 19, 8,
    4, 12, 0, 10, 2, 14, 6, 12, 23, 10, 17, 8, 21, 14, 19, 0, 22, 6, 18, 4, 20, 2, 16, 9, 5, 13, 1,
    11, 3, 15, 7, 13, 20, 11, 18, 9, 22, 15, 16, 1, 23, 7, 19, 5, 21, 3, 17, 10, 6, 14, 2, 8, 0,
    12, 4, 14, 21, 8, 19, 10, 23, 12, 17, 2, 20, 4, 16, 6, 22, 0, 18, 11, 7, 15, 3, 9, 1, 13, 5, 9,
    16, 15, 22, 13, 18, 11, 20, 5, 17, 3, 21, 1, 19, 7, 23, 12, 0, 8, 4, 14, 6, 10, 2, 10, 17, 12,
    23, 14, 19, 8, 21, 6, 18, 0, 22, 2, 16, 4, 20, 13, 1, 9, 5, 15, 7, 11, 3, 11, 18, 13, 20, 15,
    16, 9, 22, 7, 19, 1, 23, 3, 17, 5, 21, 14, 2, 10, 6, 12, 4, 8, 0, 8, 19, 14, 21, 12, 17, 10,
    23, 4, 16, 2, 20, 0, 18, 6, 22, 15, 3, 11, 7, 13, 5, 9, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13,
    14, 15, 12, 17, 18, 19, 16, 21, 22, 23, 20, 2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12,
    13, 18, 19, 16, 17, 22, 23, 20, 21, 3, 0, 1, 2, 7, 4, 5, 6, 11, 8, 9, 10, 15, 12, 13, 14, 19,
    16, 17, 18, 23, 20, 21, 22, 6, 5, 4, 7, 2, 1, 0, 3, 14, 13, 12, 15, 10, 9, 8, 11, 20, 23, 22,
    21, 16, 19, 18, 17, 7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 21, 20, 23, 22, 17,
    16, 19, 18, 4, 7, 6, 5, 0, 3, 2, 1, 12, 15, 14, 13, 8, 11, 10, 9, 22, 21, 20, 23, 18, 17, 16,
    19, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8, 11, 10, 23, 22, 21, 20, 19, 18, 17, 16,
];

#[rustfmt::skip]
pub const TRANSLATION_TABLE: [(i8, i8, i8); 24] = [
    (3, -2, 1),
    (3, -1, -2),
    (3, 2, -1),
    (3, 1, 2),

    (-3, 2, 1),
    (-3, 1, -2),
    (-3, -2, -1),
    (-3, -1, 2),

    (2, 3, 1),
    (1, 3, -2),
    (-2, 3, -1),
    (-1, 3, 2),

    (-2, -3, 1),
    (-1, -3, -2),
    (2, -3, -1),
    (1, -3, 2),

    (1, 2, 3),
    (-2, 1, 3),
    (-1, -2, 3),
    (2, -1, 3),

    (-1, 2, -3),
    (2, 1, -3),
    (1, -2, -3),
    (-2, -1, -3),
];

lazy_static! {
    pub static ref DEFAULT_MATERIALS: Vec<&'static str> = vec![
        "BMC_Hidden",
        "BMC_Ghost",
        "BMC_Ghost_Fail",
        "BMC_Plastic",
        "BMC_Glass",
        "BMC_Glow",
        "BMC_Metallic",
        "BMC_Hologram",
    ];
    pub static ref BRICK_SIZE_MAP: HashMap<&'static str, (u32, u32, u32)> = vec![
        ("B_1x1_Brick_Side", (5, 5, 6)),
        ("B_1x1_Brick_Side_Lip", (5, 5, 6)),
        ("B_1x1_Cone", (5, 5, 6)),
        ("B_1x1_Round", (5, 5, 6)),
        ("B_1x1F_Octo", (5, 5, 2)),
        ("B_1x1F_Round", (5, 5, 2)),
        ("B_1x2_Overhang", (6, 10, 5)),
        ("B_1x2f_Plate_Center", (10, 5, 2)),
        ("B_1x2f_Plate_Center_Inv", (10, 5, 2)),
        ("B_1x4_Brick_Side", (20, 5, 6)),
        ("B_1x_Octo", (5, 5, 5)),
        ("B_1x_Octo_90Deg", (5, 5, 5)),
        ("B_1x_Octo_90Deg_Inv", (5, 5, 5)),
        ("B_1x_Octo_T", (5, 5, 5)),
        ("B_1x_Octo_T_Inv", (5, 5, 5)),
        ("B_2x1_Slipper", (10, 5, 6)),
        ("B_2x2_Cone", (10, 10, 12)),
        ("B_2x2_Corner", (10, 10, 6)),
        ("B_2x2_Overhang", (6, 10, 10)),
        ("B_2x2_Round", (10, 10, 6)),
        ("B_2x2_Slipper", (10, 10, 6)),
        ("B_2x2F_Octo", (10, 10, 2)),
        ("B_2x2F_Octo_Converter", (10, 10, 2)),
        ("B_2x2F_Octo_Converter_Inv", (10, 10, 2)),
        ("B_2x2f_Plate_Center", (10, 10, 2)),
        ("B_2x2f_Plate_Center_Inv", (10, 10, 2)),
        ("B_2x2F_Round", (10, 10, 2)),
        ("B_2x4_Door_Frame", (10, 20, 36)),
        ("B_2x_Octo", (10, 10, 10)),
        ("B_2x_Octo_90Deg", (10, 10, 10)),
        ("B_2x_Octo_90Deg_Inv", (10, 10, 10)),
        ("B_2x_Octo_Cone", (10, 10, 10)),
        ("B_2x_Octo_T", (10, 10, 10)),
        ("B_2x_Octo_T_Inv", (10, 10, 10)),
        ("B_4x4_Round", (20, 20, 6)),
        ("B_8x8_Lattice_Plate", (40, 40, 2)),
        ("B_Bishop", (5, 5, 10)),
        ("B_Bone", (10, 10, 2)),
        ("B_BoneStraight", (10, 5, 2)),
        ("B_Branch", (20, 11, 2)),
        ("B_Bush", (15, 15, 16)),
        ("B_Cauldron", (10, 10, 10)),
        ("B_Chalice", (5, 5, 10)),
        ("B_CheckPoint", (20, 20, 2)),
        ("B_Coffin", (5, 20, 30)),
        ("B_Coffin_Lid", (10, 20, 30)),
        ("B_Fern", (10, 10, 2)),
        ("B_Flame", (5, 5, 10)),
        ("B_Flower", (10, 10, 2)),
        ("B_Gravestone", (10, 20, 20)),
        ("B_Handle", (5, 10, 6)),
        ("B_Hedge_1x1", (5, 5, 6)),
        ("B_Hedge_1x1_Corner", (5, 5, 6)),
        ("B_Hedge_1x2", (5, 10, 6)),
        ("B_Hedge_1x4", (5, 20, 6)),
        ("B_Inverted_Cone", (5, 5, 8)),
        ("B_Jar", (9, 9, 12)),
        ("B_King", (5, 5, 12)),
        ("B_Knight", (5, 5, 8)),
        ("B_Ladder", (12, 15, 12)),
        ("B_Pawn", (5, 5, 6)),
        ("B_Picket_Fence", (20, 5, 12)),
        ("B_Pine_Tree", (20, 20, 38)),
        ("B_Pumpkin", (10, 10, 7)),
        ("B_Pumpkin_Carved", (10, 10, 7)),
        ("B_Queen", (5, 5, 12)),
        ("B_Rook", (5, 5, 6)),
        ("B_Sausage", (10, 5, 2)),
        ("B_Small_Flower", (5, 5, 2)),
        ("B_SpawnPoint", (20, 20, 2)),
        ("B_Swirl_Plate", (5, 5, 4)),
        ("B_Turkey_Body", (15, 10, 7)),
        ("B_Turkey_Leg", (10, 5, 3)),
        ("B_Leaf_Bush", (26, 26, 20)),
        ("B_GoalPoint", (20, 20, 2)),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
}

#[inline]
fn translation_coord(coords: (i32, i32, i32), translation: i8) -> i32 {
    let sign = translation.signum() as i32;
    match translation.abs() {
        1 => sign * coords.0,
        2 => sign * coords.1,
        3 => sign * coords.2,
        _ => unreachable!(),
    }
}

pub fn use_translation_table(coords: (i32, i32, i32), orientation: u8) -> (i32, i32, i32) {
    let translation = &TRANSLATION_TABLE[orientation as usize];
    (
        translation_coord(coords, translation.0),
        translation_coord(coords, translation.1),
        translation_coord(coords, translation.2),
    )
}

/// Get a brick's size for special, non-procedural bricks.
/// If this brick is procedural or the asset couldn't be found,
/// returns (0, 0, 0).
pub fn get_brick_size(brick: &Brick, assets: &[String]) -> (u32, u32, u32) {
    assets
        .get(brick.asset_name_index as usize)
        .and_then(|a| BRICK_SIZE_MAP.get(a.as_str()))
        .map(|s| *s)
        .unwrap_or((0, 0, 0))
}

/// Gets a scale axis for scale when using rotation and direction.
pub fn get_scale_axis(brick: &Brick, mut axis: u8) -> u8 {
    match brick.direction {
        Direction::XPositive | Direction::XNegative => match axis {
            0 => axis = 2,
            2 => axis = 0,
            _ => (),
        },
        Direction::YPositive | Direction::YNegative => match axis {
            0 => axis = 1,
            1 => axis = 2,
            2 => axis = 0,
            _ => (),
        },
        _ => (),
    }

    if brick.rotation == Rotation::Deg90 || brick.rotation == Rotation::Deg270 {
        match axis {
            0 => axis = 1,
            1 => axis = 0,
            _ => (),
        }
    }

    axis
}

/// Gets a brick's size along an axis.
pub fn get_axis_size(brick: &Brick, assets: &[String], axis: u8) -> u32 {
    let size = match brick.size {
        Size::Procedural(x, y, z) => (x, y, z),
        Size::Empty => get_brick_size(brick, assets),
    };

    match get_scale_axis(brick, axis) {
        0 => size.0,
        1 => size.1,
        2 => size.2,
        _ => panic!(),
    }
}

pub mod rotation {
    pub fn d2o(direction: u8, rotation: u8) -> u8 {
        (direction << 2) | rotation
    }

    pub fn o2d(orientation: u8) -> (u8, u8) {
        ((orientation >> 2) % 6, orientation % 3)
    }

    pub fn rotate_direction((ad, ar): (u8, u8), (bd, br): (u8, u8)) -> (u8, u8) {
        o2d(super::ROTATION_TABLE[(d2o(ad, ar) * 24 + d2o(bd, br)) as usize])
    }
}
