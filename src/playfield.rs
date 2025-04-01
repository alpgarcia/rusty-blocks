use crate::shape::Rotation;



pub struct Playfield {
    cells: [usize; 220]
}

impl Playfield {

    const N_COLS: usize = 10;

    pub fn build() -> Self{
        Playfield { 
            cells: [0; 220],
        }
    }

    pub fn n_rows(&self) -> usize  {
        self.cells.len() / Self::N_COLS
    }

    pub fn n_cols(&self) -> usize {
        Self::N_COLS
    }

    pub fn get_cell(&self, row: usize, col: usize) -> usize {
        self.cells[self.cell_idx(row, col)]
    }

    pub fn add<T: Rotation + ?Sized>(&mut self, shape: &T, 
                                     row: usize, col: usize, r: usize) {

        for i in 0..shape.shape_data().len() {
        
            let shape_row = shape.shape_data().row(i);
            let shape_col = shape.shape_data().col(i);
            
            let cell_idx = self.cell_idx(row + shape_row, col + shape_col);

            let v = *shape.rotate(shape_row, shape_col, r);
            if v != 0 {
                self.cells[cell_idx] = v;
            }
        }
    }

    fn cell_idx(&self, row: usize, col: usize) -> usize {
        (row * Self::N_COLS) + col
    }

}