/// Graphics module for drawing on the screen

use macroquad::prelude::*;

use crate::{playfield::Playfield, shape::Shape};

const BLOCK_SIZE: f32 = 20.0;

pub struct Graphics {
    pub block_size: f32,
    pub pf_x: f32,
    pub pf_y: f32,
}


pub fn color_for(i: usize) -> Color {
    if i == 0 {
        BLACK
    } else if i == 99 {
        DARKPURPLE 
    } else {
        WHITE
    }
}


impl Graphics {

    pub fn new(pf: &Playfield) -> Self {
        
        let (block_size, pf_x, pf_y) = Graphics::get_scaled_vars(pf);

        Self { 
            block_size, 
            pf_x, 
            pf_y 
        }
    }

    pub fn block_size(&self) -> f32 {
        self.block_size
    }

    pub fn update_scale(&mut self, pf: &Playfield) {
        
        let (block_size, pf_x, pf_y) = Graphics::get_scaled_vars(pf);
        
        self.block_size = block_size;
        self.pf_x = pf_x;
        self.pf_y = pf_y;
    }

    pub fn draw_playfield(&self, p: &Playfield) {
        // Draw hidden rows
        // TODO we might use some kind of animation for these killing zone
        for row in 0..2 {
            for col in 0..p.n_cols() {
                draw_rectangle_lines(
                    self.pf_x + (col as f32 * self.block_size) + 1.0,
                    self.pf_y + (row as f32 * self.block_size) + 1.0,
                    self.block_size - 2.0,
                    self.block_size - 2.0,
                    4.0,
                    color_for(p.get_cell(row, col)),
                );
            }
        }
    
        // Draw visible rows
        for row in 2..p.n_rows() {
            for col in 0..p.n_cols() {
                draw_rectangle(
                    self.pf_x + (col as f32 * self.block_size) + 1.0,
                    self.pf_y + (row as f32 * self.block_size) + 1.0,
                    self.block_size - 2.0,
                    self.block_size - 2.0,
                    color_for(p.get_cell(row, col)),
                );
            }
        }
    }
    
    pub fn draw_shape(&self, shape: &Shape, row: f32, col: f32, 
                      r: isize) {

        let cs_x = self.pf_x + (col * self.block_size);
        let cs_y = self.pf_y + (row * self.block_size);
    
        self.draw_shape_abs(shape, cs_x, cs_y, r);
    }

    pub fn draw_shape_abs(&self, shape: &Shape, cs_x: f32, cs_y: f32, 
                     r: isize) {

        for i in 0..shape.len() {
    
            let shape_row = shape.row(i);
            let shape_col = shape.col(i);
    
            if *shape.rotate(shape_row, shape_col, r) != 0 {
                draw_rectangle(
                    cs_x + (shape_col as f32 * self.block_size) + 1.0,
                    cs_y + (shape_row as f32 * self.block_size) + 1.0,
                    self.block_size - 2.0,
                    self.block_size - 2.0,
                    shape.color(),
                );
            } 
            // else {
            //     draw_rectangle(
            //         pos_x + (shape_col as f32 * block_size) + 1.0,
            //         pos_y + (shape_row as f32 * block_size) + 1.0,
            //         block_size - 2.0,
            //         block_size - 2.0,
            //         BLACK,
            //     );
            // }
        }

    }

    fn get_scaled_vars(pf: &Playfield) -> (f32, f32, f32){
        // We'll use this to scale the game
        let block_size = BLOCK_SIZE.min(screen_width() / 30.0)
            .min(screen_height() / 30.0);
    
        let pf_x = (screen_width() / 2.0) 
                    - ((pf.n_cols() / 2) as f32 * block_size);
        let pf_y = (screen_height() / 2.0) 
                    - ((pf.n_rows() / 2) as f32 * block_size);

        (block_size, pf_x, pf_y)
    
    }
}