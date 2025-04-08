use macroquad::prelude::Color;

pub trait Rotation {
    fn rotate(&self, row: usize, col: usize, rot:isize) -> &usize;
    fn shape_data(&self) -> &ShapeData;
}

pub struct ShapeData {
    m: Vec<usize>,
    width: usize,
    color: Color,
}

pub struct Shape {
    shape_data: ShapeData,
}

pub struct StillShape {
    shape_data: ShapeData,
}

pub struct NesShape {
    shape_data: ShapeData,
}


impl ShapeData {

    pub fn build(m: Vec<usize>, width: usize, color: Color) -> Self {
        Self {
            m,
            width,
            color,
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

    pub fn row(&self, idx: usize) -> usize {
        idx / self.width()
    }

    pub fn col(&self, idx: usize) -> usize {
        idx % self.width()
    }
    
}

impl Shape {
    pub fn build(shape_data: ShapeData) -> Self {
        Self {
            shape_data,
        }
    }

}

impl Rotation for Shape {

    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn rotate(&self, row: usize, col: usize, rot:isize) -> &usize {
        let row_r: usize;
        let colr_r: usize;
        match rot {
            0 => {
                row_r = row;
                colr_r = col;
            },
            1 => {
                row_r = (self.shape_data.width - 1) - col;
                colr_r = row;
            },
            2 => {
                row_r = (self.shape_data.width - 1) - row;
                colr_r = (self.shape_data.width - 1) - col;
            },
            3 => {
                row_r = col;
                colr_r = (self.shape_data.width - 1) - row;
            },
            _ => panic!("Rotation values must go from 0 to 3: Got {}", rot),
        }
    
        &self.shape_data.m[((row_r * self.shape_data.width) + colr_r) as usize]

    }
}

impl StillShape {
    pub fn build(shape_data: ShapeData) -> Self {
        Self {
            shape_data,
        }
    }
}

impl Rotation for StillShape {

    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn rotate(&self, row: usize, col: usize, _rot:isize) -> &usize {

        &self.shape_data.m[((row * self.shape_data.width) + col) as usize]

    }
}

impl NesShape {
    pub fn build(shape_data: ShapeData) -> Self {
        Self {
            shape_data,
        }
    }

}

impl Rotation for NesShape {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn rotate(&self, row: usize, col: usize, rot:isize) -> &usize {
        let row_r: usize;
        let colr_r: usize;
        match rot {
            0 | 2 => {
                // 0 and 180 degrees rotates as SRS 0º
                row_r = row;
                colr_r = col;
            },
            1 | 3 => {
                // 90 and 270 degrees rotates as SRS 270º
                row_r = col;
                colr_r = (self.shape_data.width - 1) - row;
            },
            _ => panic!("Rotation values must go from 0 to 3"),
        }
    
        &self.shape_data.m[((row_r * self.shape_data.width) + colr_r) as usize]

    }
}

#[cfg(test)]
mod tests {

    use macroquad::color::BLACK;

    use super::Shape;
    use super::StillShape;
    use super::NesShape;
    use super::ShapeData;
    use super::Rotation;

    #[test]
    fn test_rotate_shape() {

        // 3x3 shape
        let s = Shape::build(ShapeData::build(
            Vec::from([
                0,   1,  2,
                3,   4,  5,
                6,   7,  8
            ]), 
            3,
            BLACK));
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
        let s = Shape::build(ShapeData::build(
            Vec::from([
                0,   1,  2,  3,
                4,   5,  6,  7,
                8,   9, 10, 11,
                12, 13, 14, 15
            ]), 
            4,
            BLACK));
            
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
        let s = StillShape::build(ShapeData::build(
            Vec::from([
                0,   1,  2,
                3,   4,  5,
                6,   7,  8
            ]), 
            3,
            BLACK));
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
        let s = StillShape::build(ShapeData::build(
            Vec::from([
                0,   1,  2,  3,
                4,   5,  6,  7,
                8,   9, 10, 11
            ]), 
            4,
            BLACK));
        
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
        let s = NesShape::build(ShapeData::build(
            Vec::from([
                0,   1,  2,
                3,   4,  5,
                6,   7,  8
            ]), 
            3,
            BLACK));
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
        let s = NesShape::build(ShapeData::build(
            Vec::from([
                0,   1,  2,  3,
                4,   5,  6,  7,
                8,   9, 10, 11,
                12, 13, 14, 15
            ]), 
            4,
            BLACK));
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