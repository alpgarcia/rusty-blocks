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
const QUICK_DROP_DELAY: f64 = 0.05;


fn generate_srs_shapes() -> Vec<Box<dyn Rotation>> {

    // TODO move this to shapes module?
    // TODO: separate SRS and NES shapes

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
        // Box::new(Shape::build(ShapeData::build(j_nes, 3, PINK))),
        // Box::new(Shape::build(ShapeData::build(l_nes, 3, BLUE))),
        // Box::new(NesShape::build(ShapeData::build(s_nes, 3, GREEN))),
        // Box::new(NesShape::build(ShapeData::build(z_nes, 3, ORANGE))),
        // Box::new(NesShape::build(ShapeData::build(i_nes, 4, RED))),
        // Box::new(Shape::build(ShapeData::build(t_nes, 3, PURPLE))),
        // Box::new(StillShape::build(ShapeData::build(o_nes, 4, YELLOW))),
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
    // TODO we might use some kind of animation for these killing zone
    for row in 0..2 {
        for col in 0..p.n_cols() {
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
    for row in 2..p.n_rows() {
        for col in 0..p.n_cols() {
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

fn rotate_clockwise(pf: &Playfield, shape: &dyn Rotation, 
                    cs_row: usize, cs_col: isize, rot: isize) -> isize {
    
    if pf.collides(shape, cs_row, cs_col as isize, (rot + 1) % 4) {
        rot
    } else {
        (rot + 1) % 4
    }
}

fn rotate_counter_cw(pf: &Playfield, shape: &dyn Rotation, 
                     cs_row: usize, cs_col: isize, rot: isize) -> isize{
    
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

fn random_shape(shapes: &Vec<Box<dyn Rotation>>) -> &Box<dyn Rotation> {
    
    // TODO: implement different shape generation algorithms

    let shape_idx = rand::gen_range(0, shapes.len());
    
    &shapes[shape_idx]

}

#[macroquad::main("Rusty Blocks")]
async fn main() {

    // TODO: add a menu

    // Initialize random number generator
    rand::srand(miniquad::date::now() as u64);

    // TODO try to add NES & Gameboy shapes options too
    let shapes = generate_srs_shapes();

    let mut pf = Playfield::build();

    let mut rotation_demo = false;

    // Initialize first shape
    let mut current_shape = random_shape(&shapes);
    let mut cs_col: isize = 
                    ((pf.n_cols() / 2) 
                     - (current_shape.shape_data().width() / 2)) 
                     as isize;
    let mut cs_row: usize = 0;
    let mut rot: isize = 0;
    let mut spawn_shape = false;

    // Initialize Shape Drop System
    let mut drop_start = get_time();
    let drop_delay = 0.8;

    // Initialize Shift Delay System
    let mut shift_start = get_time();
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

        // SPAWN NEW SHAPE --------------------------------------------
            
        if spawn_shape {
            
            // TODO: shape parameters could be stored in a struct
            // TODO: this code is repeated before starting the loop
            // TODO: add a queue of shapes to spawn
            current_shape = random_shape(&shapes);

            cs_col = ((pf.n_cols() / 2) 
                     - (current_shape.shape_data().width() / 2))
                     as isize;
            cs_row = 0;
            rot = 0;
            spawn_shape = false;

            drop_start = get_time();

            if pf.collides(&**current_shape, cs_row, cs_col as isize, rot) {
                // TODO game over
                println!("Game Over");
                break;
            }
        }

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
            is_key_released(KeyCode::Right) ||
            is_key_released(KeyCode::Down) {
            first_press = true;
        }
            
        if is_key_down(KeyCode::Left) 
            && !pf.collides(
                &**current_shape, cs_row, cs_col as isize - 1, rot) {
                
                if first_press || get_time() - shift_start >= SHIFT_DELAY {
                    
                    cs_col -= 1;
                    first_press = false;
                    shift_start = get_time();
                }
        }

        if is_key_down(KeyCode::Right)
            && !pf.collides(
                &**current_shape, cs_row, cs_col  as isize + 1 as isize, rot) {
                
                // TODO: apply a different delay for quick drop
                if first_press || get_time() - shift_start >= SHIFT_DELAY {
                    
                    cs_col += 1;
                    first_press = false;
                    shift_start = get_time();
                }
        }

        if is_key_down(KeyCode::Down) {
            if !pf.collides(
                &**current_shape, cs_row + 1, cs_col as isize, rot) {
                
                if first_press || get_time() - shift_start >= QUICK_DROP_DELAY {
                    
                    cs_row += 1;
                    first_press = false;
                    shift_start = get_time();
                    // Cancel drop delay
                    drop_start = get_time();
                }

            } else {
                spawn_shape = true;
            }
        }

        // TODO: implement hard drop

        // PROCESS FALLING RATE ---------------------------------------

        // We do this after processing input to ensure that the
        // the shape is fixed to the playfield in the correct position

        if !spawn_shape && get_time() - drop_start >= drop_delay {
            if !pf.collides(
                &**current_shape, cs_row + 1, cs_col as isize, rot) {
                
                cs_row += 1;
                drop_start = get_time();
                
            } else {
                spawn_shape = true;
            }
        }

        // FIX SHAPE --------------------------------------------------

        if spawn_shape {
            pf.add(&**current_shape, cs_row, cs_col, rot);

            // TODO remove cleared lines
            //      we could return the row numbers affected by the
            //      add method and check if all their cells are 0
            //      if so, remove them
            //      and move the rest down

            // TODO add a score system
            // TODO add a level system
            // TODO add a line clear animation
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