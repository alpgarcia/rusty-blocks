use macroquad::prelude::*;

use rusty_blocks::shape::NesShape;
use rusty_blocks::shape::Shape;
use rusty_blocks::shape::Rotation;
use rusty_blocks::shape::ShapeData;
use rusty_blocks::shape::StillShape;


const BLOCK_SIZE: f32 = 20.0;


fn generate_srs_shapes() -> Vec<Box<dyn Rotation>> {

    // TODO move this to shapes module?
    let j: Vec<u8> =Vec::from([
        1, 0, 0,
        1, 1, 1,
        0, 0, 0,
    ]);
    let j_nes: Vec<u8> =Vec::from([
        0, 0, 0,
        1, 1, 1,
        0, 0, 1,
    ]);

    let l: Vec<u8> = Vec::from([
        0, 0, 1,
        1, 1, 1,
        0, 0, 0,
    ]);
    let l_nes: Vec<u8> = Vec::from([
        0, 0, 0,
        1, 1, 1,
        1, 0, 0,
    ]);

    let s: Vec<u8> = Vec::from([
        0, 1, 1,
        1, 1, 0,
        0, 0, 0,
    ]);
    let s_nes: Vec<u8> = Vec::from([
        0, 0, 0,
        0, 1, 1,
        1, 1, 0,
    ]);

    let z: Vec<u8> = Vec::from([
        1, 1, 0,
        0, 1, 1,
        0, 0, 0,
    ]);
    let z_nes: Vec<u8> = Vec::from([
        0, 0, 0,
        1, 1, 0,
        0, 1, 1,
    ]);

    let i: Vec<u8> = Vec::from([
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ]);
    let i_nes: Vec<u8> = Vec::from([
        0, 0, 0, 0,
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
    ]);

    let t: Vec<u8> = Vec::from([
        0, 1, 0,
        1, 1, 1,
        0, 0, 0,
    ]);
    let t_nes: Vec<u8> = Vec::from([
        0, 0, 0,
        1, 1, 1,
        0, 1, 0,
    ]);

    let o: Vec<u8> = Vec::from([
        0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
    ]);

    let o_nes: Vec<u8> = Vec::from([
        0, 0, 0, 0,
        0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
    ]);
    
    vec![
        // SRS
        Box::new(Shape::build(ShapeData::build(j, 3, PINK))),
        Box::new(Shape::build(ShapeData::build(l, 3, BLUE))),
        Box::new(Shape::build(ShapeData::build(s, 3, GREEN))),
        Box::new(Shape::build(ShapeData::build(z, 3, ORANGE))),
        Box::new(Shape::build(ShapeData::build(i, 4, RED))),
        Box::new(Shape::build(ShapeData::build(t, 3, PURPLE))),
        Box::new(StillShape::build(ShapeData::build(o, 4, YELLOW))),

        // NES
        Box::new(Shape::build(ShapeData::build(j_nes, 3, PINK))),
        Box::new(Shape::build(ShapeData::build(l_nes, 3, BLUE))),
        Box::new(NesShape::build(ShapeData::build(s_nes, 3, GREEN))),
        Box::new(NesShape::build(ShapeData::build(z_nes, 3, ORANGE))),
        Box::new(NesShape::build(ShapeData::build(i_nes, 4, RED))),
        Box::new(Shape::build(ShapeData::build(t_nes, 3, PURPLE))),
        Box::new(StillShape::build(ShapeData::build(o_nes, 4, YELLOW))),
    ]
}

fn draw_shape(shape: &dyn Rotation, pos_x: f32, pos_y: f32, r: u8, block_size: f32) {

    // TODO move this to a drawing/graphics module?

    for i in 0..shape.shape_data().len() {
        
        let shape_row = i as u8 / shape.shape_data().width();
        let shape_col = i as u8 % shape.shape_data().width();
        
        if *shape.rotate(shape_row, shape_col, r) != 0 {
            draw_rectangle(
                pos_x + (shape_col as f32 * block_size) + 1.0,
                pos_y + (shape_row as f32 * block_size) + 1.0,
                block_size - 2.0,
                block_size - 2.0,
                shape.shape_data().color(),
            );
        } else {
            draw_rectangle(
                pos_x + (shape_col as f32 * block_size) + 1.0,
                pos_y + (shape_row as f32 * block_size) + 1.0,
                block_size - 2.0,
                block_size - 2.0,
                BLACK,
            );
        }
    }

}

fn rotate_clockwise(rot: &mut u8) {
    *rot = (*rot + 1) % 4; 
}

fn rotate_counter_cw(rot: &mut u8) {
    if *rot == 0 {
        *rot = 3;
    } else {
        *rot = (*rot - 1) % 4; 
    }
}

#[macroquad::main("Rusty Blocks")]
async fn main() {

    // TODO try to add NES & Gameboy shapes options too
    let shapes = generate_srs_shapes();

    let mut rot = 0;
    loop {

        // CLEAR SCREEN & COMPUTE BLOCK SIZE --------------------------

        clear_background(DARKGRAY);

        // We'll use this to scale the game
        let block_size = BLOCK_SIZE.min(screen_width() / 30.0)
            .min(screen_height() / 30.0);

        // PROCESS INPUT ----------------------------------------------

        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    if touch.position.x > screen_width() / 2.0 {
                        rotate_clockwise(&mut rot);

                    } else {
                        rotate_counter_cw(&mut rot);
                    }
                },
                _ => (),
            }
        }

        if is_key_pressed(KeyCode::D) {
            rotate_clockwise(&mut rot);
        }

        if is_key_pressed(KeyCode::S) {
            rotate_counter_cw(&mut rot);
        }

        // DRAW SHAPES

        let mut pos_x = block_size;
        let mut pos_y = -50.0;
        let mut i = 0;
        for shape in &shapes {

            if pos_x + block_size * 5.0 >= screen_width() ||
                i % 7 == 0 {
                
                pos_x = block_size;
                pos_y += 100.0;

                if i == 0 {
                    draw_text("SRS", pos_x, pos_y, 50.0, BLUE);
                } else if i == 7 {
                    draw_text("NES", pos_x, pos_y, 50.0, BLUE);
                }
                
                pos_x += block_size * 5.0;
            }

            i += 1;

            draw_shape(&**shape, pos_x, pos_y, rot, block_size);
            pos_x += block_size * 5.0;
        }

        draw_fps();

        next_frame().await
    }

}