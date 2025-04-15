/// Rotation demo for rusty blocks
 

use macroquad::prelude::*;

use crate::shape::{ShapeBuilder, ShapeFactory};
use crate::graphics::Graphics;

pub fn rotation_demo(graphics: &Graphics, shape_manager: &ShapeFactory,
                     rot: isize) {
    
    let mut pos_x = graphics.block_size();
    let mut pos_y = 50.0;
    
    draw_text(shape_manager.current_rotation_system().name(), 
              pos_x, pos_y, 50.0, BLUE);

    for shape in shape_manager.shapes() {

        if pos_x + graphics.block_size() * 5.0 >= screen_width() {
                     
            pos_x += graphics.block_size() * 5.0;
            pos_y += 100.0;

        }

        graphics.draw_shape_abs(shape, pos_x, pos_y, rot);
            
        pos_x += graphics.block_size() * 5.0;
    }

}