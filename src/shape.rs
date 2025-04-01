use macroquad::prelude::Color;

pub trait Rotation {
    fn rotate(&self, row: usize, col: usize, rot:usize) -> &usize;
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

    fn rotate(&self, row: usize, col: usize, rot:usize) -> &usize {
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

    fn rotate(&self, row: usize, col: usize, _rot:usize) -> &usize {

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

    fn rotate(&self, row: usize, col: usize, rot:usize) -> &usize {
        let row_r: usize;
        let colr_r: usize;
        match rot {
            0 | 2 => {
                row_r = row;
                colr_r = col;
            },
            // 1 => {
            //     row_r = (self.shape_data.width - 1) - col;
            //     colr_r = row;
            // },
            // 2 => {
            //     row_r = (self.shape_data.width - 1) - row;
            //     colr_r = (self.shape_data.width - 1) - col;
            // },
            1 | 3 => {
                row_r = col;
                colr_r = (self.shape_data.width - 1) - row;
            },
            _ => panic!("Rotation values must go from 0 to 3"),
        }
    
        &self.shape_data.m[((row_r * self.shape_data.width) + colr_r) as usize]

    }
}