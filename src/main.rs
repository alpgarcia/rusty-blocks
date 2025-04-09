// Project: rusty_blocks


use macroquad::prelude::*;

use rusty_blocks::playfield::Playfield;
use rusty_blocks::shape::NesShape;
use rusty_blocks::shape::Shape;
use rusty_blocks::shape::Rotation;
use rusty_blocks::shape::ShapeData;
use rusty_blocks::shape::StillShape;


const BLOCK_SIZE: f32 = 20.0;
const SHIFT_DELAY: f64 = 0.1;


fn generate_srs_shapes() -> Vec<Box<dyn Rotation>> {

    // TODO move this to shapes module?
    let j: Vec<usize> =Vec::from([
        1, 0, 0,
        1, 1, 1,
        0, 0, 0,
    ]);
    let j_nes: Vec<usize> =Vec::from([
        0, 0, 0,
        1, 1, 1,
        0, 0, 1,
    ]);

    let l: Vec<usize> = Vec::from([
        0, 0, 1,
        1, 1, 1,
        0, 0, 0,
    ]);
    let l_nes: Vec<usize> = Vec::from([
        0, 0, 0,
        1, 1, 1,
        1, 0, 0,
    ]);

    let s: Vec<usize> = Vec::from([
        0, 1, 1,
        1, 1, 0,
        0, 0, 0,
    ]);
    let s_nes: Vec<usize> = Vec::from([
        0, 0, 0,
        0, 1, 1,
        1, 1, 0,
    ]);

    let z: Vec<usize> = Vec::from([
        1, 1, 0,
        0, 1, 1,
        0, 0, 0,
    ]);
    let z_nes: Vec<usize> = Vec::from([
        0, 0, 0,
        1, 1, 0,
        0, 1, 1,
    ]);

    let i: Vec<usize> = Vec::from([
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ]);
    let i_nes: Vec<usize> = Vec::from([
        0, 0, 0, 0,
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
    ]);

    let t: Vec<usize> = Vec::from([
        0, 1, 0,
        1, 1, 1,
        0, 0, 0,
    ]);
    let t_nes: Vec<usize> = Vec::from([
        0, 0, 0,
        1, 1, 1,
        0, 1, 0,
    ]);

    let o: Vec<usize> = Vec::from([
        0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
    ]);

    let o_nes: Vec<usize> = Vec::from([
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

fn color_for(i: usize) -> Color {
    if i == 0 {
        BLACK
    } else if i == 99 {
        DARKPURPLE 
    } else {
        WHITE
    }
}

fn draw_playfield(p: &Playfield, pos_x: f32, pos_y: f32, block_size: f32) {
    // Draw hidden rows
    for row in (0..2).rev() {
        for col in (0..p.n_cols()).rev() {
            draw_rectangle_lines(
                pos_x + (col as f32 * block_size) + 1.0,
                pos_y + (row as f32 * block_size) + 1.0,
                block_size - 2.0,
                block_size - 2.0,
                4.0,
                color_for(p.get_cell(row, col)),
            );
        }
    }

    // Draw visible rows
    for row in (2..p.n_rows()).rev() {
        for col in (0..p.n_cols()).rev() {
            draw_rectangle(
                pos_x + (col as f32 * block_size) + 1.0,
                pos_y + (row as f32 * block_size) + 1.0,
                block_size - 2.0,
                block_size - 2.0,
                color_for(p.get_cell(row, col)),
            );
        }
    }
}

fn draw_shape(shape: &dyn Rotation, pos_x: f32, pos_y: f32, 
                                    r: isize, block_size: f32) {

    // TODO move this to a drawing/graphics module?

    for i in 0..shape.shape_data().len() {
        
        let shape_row = shape.shape_data().row(i);
        let shape_col = shape.shape_data().col(i);
        
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

fn rotate_clockwise(pf: &Playfield, shape: &dyn Rotation, 
                    cs_row: usize, cs_col: usize, rot: isize) -> isize {
    
    if pf.collides(shape, cs_row, cs_col as isize, (rot + 1) % 4) {
        rot
    } else {
        (rot + 1) % 4
    }
}

fn rotate_counter_cw(pf: &Playfield, shape: &dyn Rotation, 
                     cs_row: usize, cs_col: usize, rot: isize) -> isize{
    
    let new_rot: isize;
    if rot == 0 {
        new_rot = 3;
    } else {
        new_rot = (rot - 1) % 4;
    }

    if pf.collides(shape, cs_row, cs_col as isize, new_rot) {
        rot
    } else {
        new_rot
    }
}

#[macroquad::main("Rusty Blocks")]
async fn main() {

    // TODO try to add NES & Gameboy shapes options too
    let shapes = generate_srs_shapes();

    let pf = Playfield::build();

    let current_shape = &shapes[0];

    let mut rotation_demo = false;
    let mut cs_col = (pf.n_cols() / 2) 
                     - (current_shape.shape_data().width() / 2);
    let cs_row = 0;
    // let mut col_f = 0.0; 
    let mut rot: isize = 0;

    let mut start = get_time();
    let mut first_press = true;
    loop {

        // let delta = get_frame_time();

        // CLEAR SCREEN -----------------------------------------------

        clear_background(DARKGRAY);

        // SCALE BLOCK SIZE AND COMPUTE UI COMPONENTS POSITIONS -------

        // We'll use this to scale the game
        let block_size = BLOCK_SIZE.min(screen_width() / 30.0)
            .min(screen_height() / 30.0);

        let pf_x = (screen_width() / 2.0) 
                    - ((pf.n_cols() / 2) as f32 * block_size);
        let pf_y = (screen_height() / 2.0) 
                    - ((pf.n_rows() / 2) as f32 * block_size);

        // PROCESS INPUT ----------------------------------------------

        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    if touch.position.x > screen_width() / 2.0 {
                        
                        rot = rotate_clockwise(
                            &pf, &**current_shape, cs_row, cs_col, rot);

                    } else {

                        rot = rotate_counter_cw(
                            &pf, &**current_shape, cs_row, cs_col, rot);
                    }
                },
                _ => (),
            }
        }

        if is_key_pressed(KeyCode::D) { 
            rot = rotate_clockwise(
                &pf, &**current_shape, cs_row, cs_col, rot);
        }

        if is_key_pressed(KeyCode::S) { 
            rot = rotate_counter_cw(
                &pf, &**current_shape, cs_row, cs_col, rot);
        }

        if is_key_pressed(KeyCode::R) {
            rotation_demo = !rotation_demo;
        }

        if is_key_released(KeyCode::Left) || 
            is_key_released(KeyCode::Right) {
            first_press = true;
        }
            
        if is_key_down(KeyCode::Left) 
            && !pf.collides(
                &**current_shape, cs_row, cs_col as isize - 1, rot) {
                
                if first_press || get_time() - start >= SHIFT_DELAY {
                    cs_col -= 1;
                    first_press = false;
                    start = get_time();
                }
        }

        if is_key_down(KeyCode::Right)
            && !pf.collides(
                &**current_shape, cs_row, cs_col  as isize + 1 as isize, rot) {
                
                if first_press || get_time() - start >= SHIFT_DELAY {
                    cs_col += 1;
                    first_press = false;
                    start = get_time();
                }
        }
        

        // DRAW PLAYFIELD ---------------------------------------------

        if !rotation_demo {
            
            draw_playfield(&pf, pf_x, pf_y, block_size);

            // DRAW CURRENT SHAPE -------------------------------------

            let cs_x = pf_x + (cs_col as f32 * block_size);
            let cs_y = pf_y + (cs_row as f32 * block_size);
            draw_shape(&**current_shape, cs_x, cs_y, rot, block_size);
        
        } else {
        
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
        }

        draw_fps();

        next_frame().await
    }

}