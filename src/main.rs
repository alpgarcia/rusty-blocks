// Project: rusty_blocks


use macroquad::prelude::*;

use rusty_blocks::demo;
use rusty_blocks::graphics::Graphics;
use rusty_blocks::playfield::Playfield;
use rusty_blocks::shape::RotationSystem;
use rusty_blocks::shape::Shape;
use rusty_blocks::shape::ShapeFactory;


const SHIFT_DELAY: f64 = 0.1;
const QUICK_DROP_DELAY: f64 = 0.05;


fn rotate_clockwise(pf: &Playfield, shape: &Shape, 
                    cs_row: usize, cs_col: isize, rot: isize) -> isize {
    
    if pf.collides(shape, cs_row, cs_col as isize, (rot + 1) % 4) {
        rot
    } else {
        (rot + 1) % 4
    }
}

fn rotate_counter_cw(pf: &Playfield, shape:  &Shape, 
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

#[macroquad::main("Rusty Blocks")]
async fn main() {

    // TODO: add a menu

    let mut pf = Playfield::new();

    let mut graphics = Graphics::new(&pf);

    let mut shape_manager = ShapeFactory::new(RotationSystem::NES);

    let mut rotation_demo = false;

    // Initialize first shape
    let mut current_shape = shape_manager.current_shape();
    let mut cs_col: isize = 
                    ((pf.n_cols() / 2) 
                     - (current_shape.width() / 2)) 
                     as isize;
    let mut cs_row: usize = current_shape.row_offset();
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

        graphics.update_scale(&pf);

        // SPAWN NEW SHAPE --------------------------------------------
            
        if spawn_shape {
            
            // TODO: shape parameters could be stored in a struct
            // TODO: this code is repeated before starting the loop
            current_shape = shape_manager.current_shape();

            cs_col = ((pf.n_cols() / 2) 
                     - (current_shape.width() / 2))
                     as isize;
            
            cs_row = current_shape.row_offset();
            rot = 0;
            spawn_shape = false;

            drop_start = get_time();

            if pf.collides(&current_shape, cs_row, cs_col as isize, rot) {
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
                            &pf, &current_shape, cs_row, cs_col, rot);

                    } else {

                        rot = rotate_counter_cw(
                            &pf, &current_shape, cs_row, cs_col, rot);
                    }
                },
                _ => (),
            }
        }

        if is_key_pressed(KeyCode::D) { 
            rot = rotate_clockwise(
                &pf, &current_shape, cs_row, cs_col, rot);
        }

        if is_key_pressed(KeyCode::S) { 
            rot = rotate_counter_cw(
                &pf, &current_shape, cs_row, cs_col, rot);
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
                &current_shape, cs_row, cs_col as isize - 1, rot) {
                
                if first_press || get_time() - shift_start >= SHIFT_DELAY {
                    
                    cs_col -= 1;
                    first_press = false;
                    shift_start = get_time();
                }
        }

        if is_key_down(KeyCode::Right)
            && !pf.collides(
                &current_shape, cs_row, cs_col  as isize + 1 as isize, rot) {
                
                if first_press || get_time() - shift_start >= SHIFT_DELAY {
                    
                    cs_col += 1;
                    first_press = false;
                    shift_start = get_time();
                }
        }

        if is_key_down(KeyCode::Down) {
            if !pf.collides(
                &current_shape, cs_row + 1, cs_col as isize, rot) {
                
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
                &current_shape, cs_row + 1, cs_col as isize, rot) {
                
                cs_row += 1;
                drop_start = get_time();
                
            } else {
                spawn_shape = true;
            }
        }

        // FIX SHAPE --------------------------------------------------

        if spawn_shape {
            
            // Add the shape to the playfield
            let mod_rows = pf.add(&current_shape, cs_row, cs_col, rot);
            // Get the rows that need to be cleared, if any
            let cleared_lines = pf.check_rows(&mod_rows);
            // Clear the rows, if any
            pf.clear_rows(&cleared_lines);

            // TODO add a score system
            // TODO add a level system
            // TODO add a line clear animation
        }

        // DRAW PLAYFIELD ---------------------------------------------

        if !rotation_demo {
            
            graphics.draw_playfield(&pf);

            // DRAW CURRENT SHAPE -------------------------------------

            graphics.draw_shape(
                &current_shape, cs_row as f32, cs_col as f32, rot);
        
        } else {
        
            demo::rotation_demo(&graphics,
                &shape_manager, rot);

        }

        draw_fps();

        next_frame().await
    }

}