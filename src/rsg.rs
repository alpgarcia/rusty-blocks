/// Random Shape Generators
///

use macroquad::{miniquad, prelude::rand};


pub trait RSG {
    /// This method will generate a new shape number
    /// and return the one that was generated in the
    /// previous call.
    /// 
    /// Use next() to get the newly generated shape 
    /// number
    fn get(&mut self) -> usize;

    /// Return the next shape number.
    /// This is also the number that was generated
    /// (but not returned)
    /// in the previous call to get(): it will
    /// also be returned by the next call to get()
    fn next(&self) -> usize;
}


/// Truly Pseudo Random Shape Generator
/// Generates a random shape number
pub struct TSR {
    next: usize,
    n_shapes: usize,
}

impl TSR {

    /// Returns a Truly Pseudo Random Shape Generator
    /// 
    /// # Arguments
    /// * `n_shapes` - The total number of shapes
    pub fn new(n_shapes: usize) -> Self {

        // Initialize random number generator
        rand::srand(miniquad::date::now() as u64);

        Self { 
            next: rand::gen_range(0, n_shapes),
            n_shapes,
        }
    }
}

impl RSG for TSR {
    fn get(&mut self) -> usize {
        let current = self.next;
        self.next = rand::gen_range(0, self.n_shapes);

        current
    }

    fn next(&self) -> usize {
        self.next
    }
}