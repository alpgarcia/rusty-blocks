use macroquad::prelude::*;
use rusty_blocks::shape::Shape;
use rusty_blocks::shape::Rotation;
use rusty_blocks::shape::ShapeData;
use rusty_blocks::shape::StillShape;

const BLOCK_SIZE: f32 = 20.0;

fn generate_srs_shapes() -> Vec<Box<dyn Rotation>> {

    // TODO move this to shapes module?

    let var_name = Vec::from([
        1, 0, 0,
        1, 1, 1,
        0, 0, 0,
    ]);
    let j: Vec<u8> = var_name;

    let l: Vec<u8> = Vec::from([
        0, 0, 1,
        1, 1, 1,
        0, 0, 0,
    ]);

    let s: Vec<u8> = Vec::from([
        0, 1, 1,
        1, 1, 0,
        0, 0, 0,
    ]);

    let z: Vec<u8> = Vec::from([
        1, 1, 0,
        0, 1, 1,
        0, 0, 0,
    ]);

    let i: Vec<u8> = Vec::from([
        0, 0, 0, 0,
        1, 1, 1, 1,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ]);

    let t: Vec<u8> = Vec::from([
        0, 1, 0,
        1, 1, 1,
        0, 0, 0,
    ]);

    let o: Vec<u8> = Vec::from([
        0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
    ]);
    
    vec![
        Box::new(Shape::build(ShapeData::build(j, 3, PINK))),
        Box::new(Shape::build(ShapeData::build(l, 3, BLUE))),
        Box::new(Shape::build(ShapeData::build(s, 3, GREEN))),
        Box::new(Shape::build(ShapeData::build(z, 3, ORANGE))),
        Box::new(Shape::build(ShapeData::build(i, 4, RED))),
        Box::new(Shape::build(ShapeData::build(t, 3, PURPLE))),
        Box::new(StillShape::build(ShapeData::build(o, 4, YELLOW))),
    ]
}

fn draw_shape(shape: &dyn Rotation, pos_x: f32, pos_y: f32, r: u8) {

    // TODO move this to a drawing/graphics module?

    for i in 0..shape.shape_data().len() {
        
        let shape_row = i as u8 / shape.shape_data().width();
        let shape_col = i as u8 % shape.shape_data().width();
        
        if *shape.rotate(shape_row, shape_col, r) != 0 {
            draw_rectangle(
                pos_x + (shape_col as f32 * BLOCK_SIZE) + 1.0,
                pos_y + (shape_row as f32 * BLOCK_SIZE) + 1.0,
                BLOCK_SIZE - 2.0,
                BLOCK_SIZE - 2.0,
                shape.shape_data().color(),
            );
        }
    }

}

#[macroquad::main("Rusty Blocks")]
async fn main() {

    // TODO try to add NES & Gameboy shapes options too
    let shapes = generate_srs_shapes();

    let mut rot = 0;
    let mut time = get_time();
    let mut a = 0.01;
    let mut t = 0.5;
    loop {
        clear_background(DARKGRAY);

        // TODO add keyboard controls instead of automatic rotation
        if (get_time() - time) > t {
            
            rot = (rot + 1) % 4;
            
            if t <= 0.1 || t >= 0.5 {
                a = -a;
            }
            t = t + a;

            time = get_time();

        }

        let mut pos_x = 50.0;
        let pos_y = 50.0;
        for shape in &shapes {
            draw_shape(&**shape, pos_x, pos_y, rot);
            pos_x += 100.0;
        }

        draw_fps();

        next_frame().await
    }

}