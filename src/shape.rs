use macroquad::{color::*, prelude::Color};

use crate::rsg::{RSG, TSR};

pub trait Rotation {
    fn rotate_cell(&self, row: usize, col: usize, rot: isize, width: usize) -> (usize, usize);
}

pub trait ShapeBuilder {
    fn build_shapes(&self) -> Vec<Shape>;
}

#[derive(Clone)]
pub enum RotationType {
    SRS,
    NES,
    STILL,
}

pub enum RotationSystem {
    SRS,
    NES,
}

#[derive(Clone)]
pub struct Shape {
    m: Vec<usize>,
    width: usize,
    color: Color,
    rot_type: RotationType,
    row_offset: usize,
}

pub struct ShapeFactory {
    shapes: Vec<Shape>,
    rotation_system: RotationSystem,
    random_shape_generator: Box<dyn RSG>,
}


impl Rotation for RotationType {
    fn rotate_cell(&self, row: usize, col: usize, rot: isize, width: usize) -> (usize, usize) {
        match self {
            RotationType::SRS => {
                match rot {
                    0 => (row, col),
                    1 => ((width - 1) - col, row),
                    2 => ((width - 1) - row, (width - 1) - col),
                    3 => (col, (width - 1) - row),
                    _ => panic!("Rotation values must go from 0 to 3: Got {}", rot),
                }
            },
            RotationType::NES => {
                match rot {
                    0 | 2 => (row, col),
                    1 | 3 => (col, (width - 1) - row),
                    _ => panic!("Rotation values must go from 0 to 3"),
                }
            },
            RotationType::STILL => (row, col),
        }
    }
}

impl ShapeBuilder for RotationSystem {

    fn build_shapes(&self) -> Vec<Shape> {
        match self {
            RotationSystem::SRS => Self::srs_shapes(),
            RotationSystem::NES => Self::nes_shapes(),
        }
    }
}

impl RotationSystem {
    fn srs_shapes() -> Vec<Shape> {
        let j: Vec<usize> = Vec::from([
            1, 0, 0,
            1, 1, 1,
            0, 0, 0,
        ]);
        let l: Vec<usize> = Vec::from([
            0, 0, 1,
            1, 1, 1,
            0, 0, 0,
        ]);
        let s: Vec<usize> = Vec::from([
            0, 1, 1,
            1, 1, 0,
            0, 0, 0,
        ]);
        let z: Vec<usize> = Vec::from([
            1, 1, 0,
            0, 1, 1,
            0, 0, 0,
        ]);
        let i: Vec<usize> = Vec::from([
            0, 0, 0, 0,
            1, 1, 1, 1,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ]);
        let t: Vec<usize> = Vec::from([
            0, 1, 0,
            1, 1, 1,
            0, 0, 0,
        ]);
        let o: Vec<usize> = Vec::from([
            0, 1, 1, 0,
            0, 1, 1, 0,
            0, 0, 0, 0,
        ]);

        vec![
            Shape::new(j, 3, PINK, RotationType::SRS),
            Shape::new(l, 3, BLUE, RotationType::SRS),
            Shape::new(s, 3, GREEN, RotationType::SRS),
            Shape::new(z, 3, ORANGE, RotationType::SRS),
            Shape::new(i, 4, RED, RotationType::SRS),
            Shape::new(t, 3, PURPLE, RotationType::SRS),
            Shape::new(o, 4, YELLOW, RotationType::STILL),
        ]
    }

    fn nes_shapes() -> Vec<Shape> {
        let j_nes: Vec<usize> = Vec::from([
            0, 0, 0,
            1, 1, 1,
            0, 0, 1,
        ]);

        let l_nes: Vec<usize> = Vec::from([
            0, 0, 0,
            1, 1, 1,
            1, 0, 0,
        ]);

        let s_nes: Vec<usize> = Vec::from([
            0, 0, 0,
            0, 1, 1,
            1, 1, 0,
        ]);

        let z_nes: Vec<usize> = Vec::from([
            0, 0, 0,
            1, 1, 0,
            0, 1, 1,
        ]);

        let i_nes: Vec<usize> = Vec::from([
            0, 0, 0, 0,
            0, 0, 0, 0,
            1, 1, 1, 1,
            0, 0, 0, 0,
        ]);

        let t_nes: Vec<usize> = Vec::from([
            0, 0, 0,
            1, 1, 1,
            0, 1, 0,
        ]);

        let o_nes: Vec<usize> = Vec::from([
            0, 0, 0, 0,
            0, 1, 1, 0,
            0, 1, 1, 0,
            0, 0, 0, 0,
        ]);

        vec![
            Shape::new(j_nes, 3, PINK, RotationType::SRS).set_row_offset(1),
            Shape::new(l_nes, 3, BLUE, RotationType::SRS).set_row_offset(1),
            Shape::new(s_nes, 3, GREEN, RotationType::NES).set_row_offset(1),
            Shape::new(z_nes, 3, ORANGE, RotationType::NES).set_row_offset(1),
            Shape::new(i_nes, 4, RED, RotationType::NES),
            Shape::new(t_nes, 3, PURPLE, RotationType::SRS).set_row_offset(1),
            Shape::new(o_nes, 4, YELLOW, RotationType::STILL).set_row_offset(1),
        ]
    }
}


impl Shape {
    pub fn new(m: Vec<usize>, width: usize, color: Color, shape_type: RotationType) -> Self {
        Self {
            m,
            width,
            color,
            rot_type: shape_type,
            row_offset: 0,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn len(&self) -> usize {
        self.m.len()
    }

    pub fn rot_type(&self) -> &RotationType {
        &self.rot_type
    }

    pub fn row(&self, idx: usize) -> usize {
        idx / self.width()
    }

    pub fn col(&self, idx: usize) -> usize {
        idx % self.width()
    }

    pub fn row_offset(&self) -> usize {
        self.row_offset
    }

    pub fn set_row_offset(mut self, row_offset: usize) -> Self {
        self.row_offset = row_offset;
        self
    }

    pub fn rotate(&self, row: usize, col: usize, rot: isize) -> &usize {
        let (row_r, col_r) = self.rot_type.rotate_cell(row, col, rot, self.width);
        &self.m[((row_r * self.width) + col_r) as usize]
    }
}

impl ShapeFactory {
    pub fn new(rotation_system: RotationSystem) -> Self {
        
        let shapes = rotation_system.build_shapes();
        let n_shapes = shapes.len();

        Self {
            shapes,
            rotation_system,
            random_shape_generator: Box::new(TSR::new(n_shapes)),
        }
    }

    pub fn current_rotation_system(&self) -> &RotationSystem {
        &self.rotation_system
    }

    pub fn update_rotation_system(&mut self, rotation_system: RotationSystem) {
        self.rotation_system = rotation_system;

        // Rebuild the shapes with the new rotation system
        self.shapes = self.rotation_system.build_shapes();
        let n_shapes = self.shapes.len();
        // Update the random shape generator with the new number of shapes
        self.random_shape_generator = Box::new(TSR::new(n_shapes));
    }

    pub fn current_shape(&mut self) -> Shape {
        let shape_number = self.random_shape_generator.get();
        self.shapes[shape_number].clone()
    }

    pub fn next_shape(&mut self) -> Shape {
        let shape_number = self.random_shape_generator.next();
        self.shapes[shape_number].clone()
    }

    /// Return a list of currently active shapes
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }

    
}

#[cfg(test)]
mod tests {
    use macroquad::color::*;

    use super::Shape;
    use super::RotationType;

    #[test]
    fn test_rotate_shape() {
        // 3x3 shape
        let s = Shape::new(
            Vec::from([
                0, 1, 2,
                3, 4, 5,
                6, 7, 8,
            ]),
            3,
            BLACK,
            RotationType::SRS,
        );

        // FIRST ROW ----------------------
        // 0º
        assert_eq!(0, *s.rotate(0, 0, 0));
        assert_eq!(1, *s.rotate(0, 1, 0));
        assert_eq!(2, *s.rotate(0, 2, 0));
        // 90º
        assert_eq!(6, *s.rotate(0, 0, 1));
        assert_eq!(3, *s.rotate(0, 1, 1));
        assert_eq!(0, *s.rotate(0, 2, 1));
        // 180º
        assert_eq!(8, *s.rotate(0, 0, 2));
        assert_eq!(7, *s.rotate(0, 1, 2));
        assert_eq!(6, *s.rotate(0, 2, 2));
        // 270º
        assert_eq!(2, *s.rotate(0, 0, 3));
        assert_eq!(5, *s.rotate(0, 1, 3));
        assert_eq!(8, *s.rotate(0, 2, 3));
        // SECOND ROW ----------------------
        // 0º
        assert_eq!(3, *s.rotate(1, 0, 0));
        assert_eq!(4, *s.rotate(1, 1, 0));
        assert_eq!(5, *s.rotate(1, 2, 0));
        // 90º
        assert_eq!(7, *s.rotate(1, 0, 1));
        assert_eq!(4, *s.rotate(1, 1, 1));
        assert_eq!(1, *s.rotate(1, 2, 1));
        // 180º
        assert_eq!(5, *s.rotate(1, 0, 2));
        assert_eq!(4, *s.rotate(1, 1, 2));
        assert_eq!(3, *s.rotate(1, 2, 2));
        // 270º
        assert_eq!(1, *s.rotate(1, 0, 3));
        assert_eq!(4, *s.rotate(1, 1, 3));
        assert_eq!(7, *s.rotate(1, 2, 3));
        // THIRD ROW ----------------------
        // 0º
        assert_eq!(6, *s.rotate(2, 0, 0));
        assert_eq!(7, *s.rotate(2, 1, 0));
        assert_eq!(8, *s.rotate(2, 2, 0));
        // 90º
        assert_eq!(8, *s.rotate(2, 0, 1));
        assert_eq!(5, *s.rotate(2, 1, 1));
        assert_eq!(2, *s.rotate(2, 2, 1));
        // 180º
        assert_eq!(2, *s.rotate(2, 0, 2));
        assert_eq!(1, *s.rotate(2, 1, 2));
        assert_eq!(0, *s.rotate(2, 2, 2));
        // 270º
        assert_eq!(0, *s.rotate(2, 0, 3));
        assert_eq!(3, *s.rotate(2, 1, 3));
        assert_eq!(6, *s.rotate(2, 2, 3));

        // 4x4 shape
        let s = Shape::new(
            Vec::from([
                0,   1,  2,  3,
                4,   5,  6,  7,
                8,   9, 10, 11,
                12, 13, 14, 15,
            ]),
            4,
            BLACK,
            RotationType::SRS,
        );

        // FIRST ROW ----------------------
        // 0º
        assert_eq!(0, *s.rotate(0, 0, 0));
        assert_eq!(1, *s.rotate(0, 1, 0));
        assert_eq!(2, *s.rotate(0, 2, 0));
        assert_eq!(3, *s.rotate(0, 3, 0));
        // 90º
        assert_eq!(12, *s.rotate(0, 0, 1));
        assert_eq!(8, *s.rotate(0, 1, 1));
        assert_eq!(4, *s.rotate(0, 2, 1));
        assert_eq!(0, *s.rotate(0, 3, 1));
        // 180º
        assert_eq!(15, *s.rotate(0, 0, 2));
        assert_eq!(14, *s.rotate(0, 1, 2));
        assert_eq!(13, *s.rotate(0, 2, 2));
        assert_eq!(12, *s.rotate(0, 3, 2));
        // 270º
        assert_eq!(3, *s.rotate(0, 0, 3));
        assert_eq!(7, *s.rotate(0, 1, 3));
        assert_eq!(11, *s.rotate(0, 2, 3));
        assert_eq!(15, *s.rotate(0, 3, 3));
        // SECOND ROW ----------------------
        // 0º
        assert_eq!(4, *s.rotate(1, 0, 0));
        assert_eq!(5, *s.rotate(1, 1, 0));
        assert_eq!(6, *s.rotate(1, 2, 0));
        assert_eq!(7, *s.rotate(1, 3, 0));
        // 90º
        assert_eq!(13, *s.rotate(1, 0, 1));
        assert_eq!(9, *s.rotate(1, 1, 1));
        assert_eq!(5, *s.rotate(1, 2, 1));
        assert_eq!(1, *s.rotate(1, 3, 1));
        // 180º
        assert_eq!(11, *s.rotate(1, 0, 2));
        assert_eq!(10, *s.rotate(1, 1, 2));
        assert_eq!(9, *s.rotate(1, 2, 2));
        assert_eq!(8, *s.rotate(1, 3, 2));
        // 270º
        assert_eq!(2, *s.rotate(1, 0, 3));
        assert_eq!(6, *s.rotate(1, 1, 3));
        assert_eq!(10, *s.rotate(1, 2, 3));
        assert_eq!(14, *s.rotate(1, 3, 3));
        // THIRD ROW ----------------------
        // 0º
        assert_eq!(8, *s.rotate(2, 0, 0));
        assert_eq!(9, *s.rotate(2, 1, 0));
        assert_eq!(10, *s.rotate(2, 2, 0));
        assert_eq!(11, *s.rotate(2, 3, 0));
        // 90º
        assert_eq!(14, *s.rotate(2, 0, 1));
        assert_eq!(10, *s.rotate(2, 1, 1));
        assert_eq!(6, *s.rotate(2, 2, 1));
        assert_eq!(2, *s.rotate(2, 3, 1));
        // 180º
        assert_eq!(7, *s.rotate(2, 0, 2));
        assert_eq!(6, *s.rotate(2, 1, 2));
        assert_eq!(5, *s.rotate(2, 2, 2));
        assert_eq!(4, *s.rotate(2, 3, 2));
        // 270º
        assert_eq!(1, *s.rotate(2, 0, 3));
        assert_eq!(5, *s.rotate(2, 1, 3));
        assert_eq!(9, *s.rotate(2, 2, 3));
        assert_eq!(13, *s.rotate(2, 3, 3));
        // FOURTH ROW ----------------------
        // 0º
        assert_eq!(12, *s.rotate(3, 0, 0));
        assert_eq!(13, *s.rotate(3, 1, 0));
        assert_eq!(14, *s.rotate(3, 2, 0));
        assert_eq!(15, *s.rotate(3, 3, 0));
        // 90º
        assert_eq!(15, *s.rotate(3, 0, 1));
        assert_eq!(11, *s.rotate(3, 1, 1));
        assert_eq!(7, *s.rotate(3, 2, 1));
        assert_eq!(3, *s.rotate(3, 3, 1));
        // 180º
        assert_eq!(3, *s.rotate(3, 0, 2));
        assert_eq!(2, *s.rotate(3, 1, 2));
        assert_eq!(1, *s.rotate(3, 2, 2));
        assert_eq!(0, *s.rotate(3, 3, 2));
        // 270º
        assert_eq!(0, *s.rotate(3, 0, 3));
        assert_eq!(4, *s.rotate(3, 1, 3));
        assert_eq!(8, *s.rotate(3, 2, 3));
        assert_eq!(12, *s.rotate(3, 3, 3));
    }

    #[test]
    fn test_rotate_still_shape() {
        // 3x3 shape
        let s = Shape::new(
            Vec::from([
                0, 1, 2,
                3, 4, 5,
                6, 7, 8,
            ]),
            3,
            BLACK,
            RotationType::STILL,
        );

        // FIRST ROW ----------------------
        for i in 0..4 {
            assert_eq!(0, *s.rotate(0, 0, i));
            assert_eq!(1, *s.rotate(0, 1, i));
            assert_eq!(2, *s.rotate(0, 2, i));
        }
        // SECOND ROW ----------------------
        for i in 0..4 {
            assert_eq!(3, *s.rotate(1, 0, i));
            assert_eq!(4, *s.rotate(1, 1, i));
            assert_eq!(5, *s.rotate(1, 2, i));
        }
        // THIRD ROW ----------------------
        for i in 0..4 {
            assert_eq!(6, *s.rotate(2, 0, i));
            assert_eq!(7, *s.rotate(2, 1, i));
            assert_eq!(8, *s.rotate(2, 2, i));
        }

        // 3x4 shape
        let s = Shape::new(
            Vec::from([
                0,  1,  2,  3,
                4,  5,  6,  7,
                8,  9, 10, 11,
            ]),
            4,
            BLACK,
            RotationType::STILL,
        );

        // FIRST ROW ----------------------
        for i in 0..4 {
            assert_eq!(0, *s.rotate(0, 0, i));
            assert_eq!(1, *s.rotate(0, 1, i));
            assert_eq!(2, *s.rotate(0, 2, i));
            assert_eq!(3, *s.rotate(0, 3, i));
        }
        // SECOND ROW ----------------------
        for i in 0..4 {
            assert_eq!(4, *s.rotate(1, 0, i));
            assert_eq!(5, *s.rotate(1, 1, i));
            assert_eq!(6, *s.rotate(1, 2, i));
            assert_eq!(7, *s.rotate(1, 3, i));
        }
        // THIRD ROW ----------------------
        for i in 0..4 {
            assert_eq!(8, *s.rotate(2, 0, i));
            assert_eq!(9, *s.rotate(2, 1, i));
            assert_eq!(10, *s.rotate(2, 2, i));
            assert_eq!(11, *s.rotate(2, 3, i));
        }
    }

    #[test]
    fn test_rotate_nes_shape() {
        // 3x3 shape
        let s = Shape::new(
            Vec::from([
                0, 1, 2,
                3, 4, 5,
                6, 7, 8,
            ]),
            3,
            BLACK,
            RotationType::NES,
        );

        // FIRST ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(0, *s.rotate(0, 0, i));
                assert_eq!(1, *s.rotate(0, 1, i));
                assert_eq!(2, *s.rotate(0, 2, i));
            } else {
                assert_eq!(2, *s.rotate(0, 0, i));
                assert_eq!(5, *s.rotate(0, 1, i));
                assert_eq!(8, *s.rotate(0, 2, i));
            }
        }
        // SECOND ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(3, *s.rotate(1, 0, i));
                assert_eq!(4, *s.rotate(1, 1, i));
                assert_eq!(5, *s.rotate(1, 2, i));
            } else {
                assert_eq!(1, *s.rotate(1, 0, i));
                assert_eq!(4, *s.rotate(1, 1, i));
                assert_eq!(7, *s.rotate(1, 2, i));
            }
        }
        // THIRD ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(6, *s.rotate(2, 0, i));
                assert_eq!(7, *s.rotate(2, 1, i));
                assert_eq!(8, *s.rotate(2, 2, i));
            } else {
                assert_eq!(0, *s.rotate(2, 0, i));
                assert_eq!(3, *s.rotate(2, 1, i));
                assert_eq!(6, *s.rotate(2, 2, i));
            }
        }

        // 4x4 shape
        let s = Shape::new(
            Vec::from([
                0,  1,  2,  3,
                4,  5,  6,  7,
                8,  9, 10, 11,
               12, 13, 14, 15,
            ]),
            4,
            BLACK,
            RotationType::NES,
        );

        // FIRST ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(0, *s.rotate(0, 0, i));
                assert_eq!(1, *s.rotate(0, 1, i));
                assert_eq!(2, *s.rotate(0, 2, i));
                assert_eq!(3, *s.rotate(0, 3, i));
            } else {
                assert_eq!(3, *s.rotate(0, 0, i));
                assert_eq!(7, *s.rotate(0, 1, i));
                assert_eq!(11, *s.rotate(0, 2, i));
                assert_eq!(15, *s.rotate(0, 3, i));
            }
        }
        // SECOND ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(4, *s.rotate(1, 0, i));
                assert_eq!(5, *s.rotate(1, 1, i));
                assert_eq!(6, *s.rotate(1, 2, i));
                assert_eq!(7, *s.rotate(1, 3, i));
            } else {
                assert_eq!(2, *s.rotate(1, 0, i));
                assert_eq!(6, *s.rotate(1, 1, i));
                assert_eq!(10, *s.rotate(1, 2, i));
                assert_eq!(14, *s.rotate(1, 3, i));
            }
        }
        // THIRD ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(8, *s.rotate(2, 0, i));
                assert_eq!(9, *s.rotate(2, 1, i));
                assert_eq!(10, *s.rotate(2, 2, i));
                assert_eq!(11, *s.rotate(2, 3, i));
            } else {
                assert_eq!(1, *s.rotate(2, 0, i));
                assert_eq!(5, *s.rotate(2, 1, i));
                assert_eq!(9, *s.rotate(2, 2, i));
                assert_eq!(13, *s.rotate(2, 3, i));
            }
        }
        // FOURTH ROW ----------------------
        for i in 0..4 {
            if i % 2 == 0 {
                assert_eq!(12, *s.rotate(3, 0, i));
                assert_eq!(13, *s.rotate(3, 1, i));
                assert_eq!(14, *s.rotate(3, 2, i));
                assert_eq!(15, *s.rotate(3, 3, i));
            } else {
                assert_eq!(0, *s.rotate(3, 0, i));
                assert_eq!(4, *s.rotate(3, 1, i));
                assert_eq!(8, *s.rotate(3, 2, i));
                assert_eq!(12, *s.rotate(3, 3, i));
            }
        }
    }
}