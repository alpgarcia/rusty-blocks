use macroquad::prelude::Color;

pub trait Rotation {
    fn rotate(&self, row: u8, col: u8, rot:u8) -> &u8;
    fn shape_data(&self) -> &ShapeData;
}

pub struct ShapeData {
    m: Vec<u8>,
    width: u8,
    color: Color,
}

pub struct Shape {
    shape_data: ShapeData,
}

pub struct StillShape {
    shape_data: ShapeData,
}


impl ShapeData {

    pub fn build(m: Vec<u8>, width: u8, color: Color) -> Self {
        Self {
            m,
            width,
            color,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn len(&self) -> usize {
        self.m.len()
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

    fn rotate(&self, row: u8, col: u8, rot:u8) -> &u8 {
        let row_r: u8;
        let colr_r: u8;
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
            _ => panic!("Rotation values must go from 0 to 3"),
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

    fn rotate(&self, row: u8, col: u8, _rot:u8) -> &u8 {

        &self.shape_data.m[((row * self.shape_data.width) + col) as usize]

    }
}