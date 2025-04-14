
use crate::shape::Shape;



pub struct Playfield {
    cells: [usize; 276]
}

impl Playfield {

    const N_COLS: usize = 12;

    pub fn new() -> Self{

        // Set borders at cols 0 and 11
        let mut _cells = [0;276];
        for row in 0..23 {
            _cells[row * Self::N_COLS] = 99;
            _cells[(row * Self::N_COLS) + (Self::N_COLS - 1)] = 99;
        }

        // Set border at row 22
        for col in 0..Self::N_COLS {
            _cells[(22 * Self::N_COLS) + col] = 99;
        }

        Playfield { 
            cells: _cells,
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

    /// Add a shape to the playfield at the given row and column
    /// position. The shape is rotated by the given rotation
    /// value. The shape is added to the playfield by setting
    /// the cells to the value of the shape at the given
    /// position.
    /// 
    /// Asumes that the position is valid and it will
    /// not check for collisions but overwrite the cells
    /// at the given position, ignoring column numbers
    /// < 0 (should be blank columns, controlled by
    /// collides function that must be called beforehand).
    ///
    /// # Arguments
    /// * `shape` - The shape to add to the playfield
    /// * `row` - The row position to add the shape
    /// * `col` - The column position to add the shape
    /// * `r` - The rotation value of the shape
    /// 
    /// # Returns
    /// A Vector with the rows that were modified in
    /// the playfield. The rows are sorted in ascending
    /// order.
    /// 
    pub fn add(&mut self, shape: &Shape, 
               row: usize, col: isize, r: isize) -> Vec<usize> {
        
        let mut rows: Vec<usize> = Vec::new();

        for i in 0..shape.len() {
        
            let shape_row = shape.row(i);
            let shape_col = shape.col(i);

            let v = *shape.rotate(shape_row, shape_col, r);
            if v > 0 {
                // col can be negative, when there are two or more blank
                // columns on the left side of the shape (that should
                // always be blank columns because collides must be called
                // beforehand and will control this too)
                let cell_idx = self.cell_idx(row + shape_row, 
                    (col + shape_col as isize) as usize);

                self.cells[cell_idx] = v;

                if !rows.contains(&(row + shape_row)) {
                    rows.push(row + shape_row);
                }
            }
        }

        // Sort the rows in ascending order
        rows.sort();

        rows

    }

    /// Check if the given rows are full. A row is full
    /// if all the cells in the row are not empty (0).
    /// The first and last columns are not checked because
    /// they are borders and should always be full.
    ///
    /// # Arguments
    /// * `rows` - The rows to check in ascending order
    /// 
    /// # Returns
    /// A vector with the rows that are full in ascending order.
    pub fn check_rows(&self, rows: &Vec<usize>) -> Vec<usize> {
        
        let mut cleared_rows: Vec<usize> = Vec::new();

        for row in rows {
            
            let mut full_row = true;
            
            for col in 1..Self::N_COLS - 1 {
                if self.cells[self.cell_idx(*row, col)] == 0 {
                    full_row = false;
                    break;
                }
            }

            if full_row {
                cleared_rows.push(*row);
            }
        }

        cleared_rows

    }

    pub fn is_empty(&self, row: usize) -> bool {
        for col in 1..Self::N_COLS - 1 {
            if self.cells[self.cell_idx(row, col)] > 0 {
                return false;
            }
        }
        true
    }

    /// Clear the cells of the given rows. The cells are set to 0.
    /// All rows above the cleared rows are moved down by one row.
    ///
    /// # Arguments
    /// * `rows` - The rows to clear in ascending order
    ///
    pub fn clear_rows(&mut self, rows: &Vec<usize>) {

        // Move the rows down
        // The rows are sorted in ascending order, so we start
        // from the top cleared row (closer to the top of the
        // playfield). This way we ensure that we don't move
        // down rows that are also cleared.
        for cleared_row in rows.iter() {

            // for each clared row, start from the row above
            // (closer to the top of the playfield) and move
            // to the top of the playfield 
            for r in (0..*cleared_row).rev() {

                // move the row down
                for col in 1..Self::N_COLS - 1 {
                    self.cells[self.cell_idx(r + 1, col)] = 
                        self.cells[self.cell_idx(r, col)];
                }

                // if the row we just moved down is empty, we 
                // can stop because all the rows above (which
                // row index is less than the cleared row) are
                // empty too (can't be empty rows in between
                // not empty rows as any empty row below a 
                // non-empty one should have been overwritten
                // by the non-empty row in a previous iteration)
                if self.is_empty(r) {
                    break;
                }
            }
        }
    } 

    pub fn collides(&self, shape: &Shape, 
                row: usize, col: isize, r: isize) -> bool{

        let mut collision = false;

        for i in 0..shape.len() {

            let shape_row = shape.row(i);
            let shape_col = shape.col(i);

            let v = *shape.rotate(shape_row, shape_col, r);

            let pf_row = row + shape_row;
            let pf_col = col + shape_col as isize;

            // Empty blocks must not be checked because they
            // never cause a collision, however they can reach
            // out of bounds column positions when the shapes
            // are placed against the left wall and have empty 
            // columns at their left. Thus, not checking them
            // is not only more efficient but also prevents
            // out of bounds errors.
            if v == 0 {
                continue;

            // if there are blocks outside the playfield (can happen
            // when trying to rotate next to the border) => collision
            }  else if pf_col < 0 || pf_col > 11 || pf_row > 22 {
                collision = true;
                break;
            }
            
            let cell_idx = self.cell_idx(pf_row, pf_col as usize);
            if self.cells[cell_idx] > 0 {
                collision = true;
                break;
            }
        }

        collision

    }

    fn cell_idx(&self, row: usize, col: usize) -> usize {
        (row * Self::N_COLS) + col
    }

}

#[cfg(test)]
mod tests {
    use macroquad::color::BLACK;

    use super::Playfield;
    use crate::shape::{Shape, RotationType};
    

    #[test]
    fn test_playfield() {
        let pf = Playfield::new();
        assert_eq!(23, pf.n_rows());
        assert_eq!(12, pf.n_cols());

        for i in 0..pf.n_rows() {
            for j in 0..pf.n_cols() {
                if i == pf.n_rows() - 1 || j == 0 || j == pf.n_cols() - 1 {
                    assert_eq!(99, pf.get_cell(i, j));
                } else {
                    assert_eq!(0, pf.get_cell(i, j));
                }
            }
        }
    }

    #[test]
    fn test_playfield_add() {
        let mut pf = Playfield::new();

        let shape = Shape::new(
            vec![0, 1, 1, 0,
                 0, 1, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );

        // Add shape to playfield at (0, 0) with rotation 0
        // (don't overwrite the borders)
        assert_eq!(vec![0, 1], pf.add(&shape, 0, 0, 0));
        assert_eq!(1, pf.get_cell(0, 1));
        assert_eq!(1, pf.get_cell(0, 2));
        assert_eq!(1, pf.get_cell(1, 1));
        assert_eq!(1, pf.get_cell(1, 2));
        assert_eq!(99, pf.get_cell(0, 0));
        assert_eq!(99, pf.get_cell(1, 0));
        assert_eq!(0, pf.get_cell(0, 3));
        assert_eq!(0, pf.get_cell(1, 3));
        assert_eq!(99, pf.get_cell(2, 0));
        assert_eq!(0, pf.get_cell(2, 1));
        assert_eq!(0, pf.get_cell(2, 2));
        assert_eq!(0, pf.get_cell(2, 3));

        // Add shape to playfield overlapping the borders
        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![1, 1, 1, 0,
                 1, 1, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        assert_eq!(vec![0, 1], pf.add(&shape, 0, 0, 0));
        assert_eq!(1, pf.get_cell(0, 0));
        assert_eq!(1, pf.get_cell(0, 1));
        assert_eq!(1, pf.get_cell(0, 2));
        assert_eq!(1, pf.get_cell(1, 0));
        assert_eq!(1, pf.get_cell(1, 1));
        assert_eq!(1, pf.get_cell(1, 2));
        assert_eq!(0, pf.get_cell(0, 3));
        assert_eq!(0, pf.get_cell(1, 3));
        assert_eq!(99, pf.get_cell(2, 0));
        assert_eq!(0, pf.get_cell(2, 1));
        assert_eq!(0, pf.get_cell(2, 2));
        assert_eq!(0, pf.get_cell(2, 3));

        // Add shape to the playfield with blank columns
        // on the left side
        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        assert_eq!(vec![0, 1], pf.add(&shape, 0, -1, 0));
        assert_eq!(99, pf.get_cell(0, 0));
        assert_eq!(1, pf.get_cell(0, 1));
        assert_eq!(0, pf.get_cell(0, 2));
        assert_eq!(99, pf.get_cell(1, 0));
        assert_eq!(1, pf.get_cell(1, 1));
        assert_eq!(0, pf.get_cell(1, 2));
        assert_eq!(0, pf.get_cell(0, 3));
        assert_eq!(0, pf.get_cell(1, 3));
        assert_eq!(99, pf.get_cell(2, 0));
        assert_eq!(0, pf.get_cell(2, 1));
        assert_eq!(0, pf.get_cell(2, 2));
        assert_eq!(0, pf.get_cell(2, 3));

    }

    #[test]
    fn test_playfield_collides() {
        let mut pf = Playfield::new();

        let shape = Shape::new(
            vec![0, 1, 1, 0,
                 0, 1, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );

        // Add shape to playfield at (0, 0) with rotation 0
        // (don't overwrite the borders)
        pf.add(&shape, 0, 0, 0);
        assert_eq!(true, pf.collides(&shape, 0, 0, 0));
        assert_eq!(true, pf.collides(&shape, 0, 1, 0));
        assert_eq!(true, pf.collides(&shape, 1, 0, 0));
        assert_eq!(true, pf.collides(&shape, 1, 1, 0));

        assert_eq!(false, pf.collides(&shape, 0, 2, 0));
        assert_eq!(false, pf.collides(&shape, 1, 2, 0));
        assert_eq!(false, pf.collides(&shape, 2, 0, 0));
        assert_eq!(false, pf.collides(&shape, 2, 1, 0));

        // Add shape to playfield overlapping the borders
        let pf = Playfield::new();
        let shape = Shape::new(
            vec![1, 1, 1, 0,
                 1, 1, 1, 0,
                 0, 0, 0, 0,
                 0, 0, 0, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test side borders collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 0, 0));
            assert_eq!(true, pf.collides(&shape, i, 9, 0));
            assert_eq!(true, pf.collides(&shape, i, 10, 0));
            assert_eq!(true, pf.collides(&shape, i, 11, 0));
            // Avoid going out of bounds (last row, column 10, when
            // rotated 90º blocks would be displaced 2 columns to 
            // the left). That position is not valid as the shape
            // shouldn't be placed over the last row in any case.
            if i < pf.n_rows() - 1 {
                // Last valid column, then rotate 90º
                assert_eq!(true, pf.collides(&shape, i, 10, 1));
            }
        }
        // Test bottom border collisions
        for i in 0..pf.n_cols() {
            assert_eq!(true, pf.collides(&shape, 21, i as isize, 0));
            assert_eq!(true, pf.collides(&shape, 22, i as isize, 0));
        }
        // Test valid positions
        for i in 0..pf.n_rows() - 2 {
            for j in 1..pf.n_cols() - 3 {
                assert_eq!(false, pf.collides(&shape, i, j as isize, 0));
            }
        }

        // Test left border collision when the first column of the shape
        // needs to be places out of bounds
        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 1, 1, 0,
                 0, 1, 1, 0,
                 0, 0, 0, 0,
                 0, 0, 0, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, -1, 0));
        }

        // Test collision when a shape rotation leaves some blocks
        // out of bounds (col < 0)
        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, -1, 1));
        }

        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 0, 1));
        }

        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, -1, 1));
        }

        // Test collision when a shape rotation leaves some blocks
        // out of the playfield on the right edge (col > 11)
        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 10, 1));
        }

        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 11, 1));
        }

        let pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
            RotationType::SRS
        );
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 12, 1));
        }

    }

    #[test]
    fn test_playfield_check_rows() {
        let mut pf = Playfield::new();

        let shape = Shape::new(
            vec![1, 1, 1, 0,
                 1, 1, 1, 0,
                 0, 0, 0, 0,
                 0, 0, 0, 0],
            4,
            BLACK,
            RotationType::SRS
        );

        // Add shape to playfield at (0, 0) with rotation 0
        // (don't overwrite the borders)
        pf.add(&shape, 0, 1, 0);
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![0]));
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![1]));
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![0, 1]));
        pf.add(&shape, 0, 4, 0);
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![0]));
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![1]));
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![0, 1]));
        pf.add(&shape, 0, 5, 1);
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![0]));
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![1]));
        assert_eq!(Vec::<usize>::new(), pf.check_rows(&vec![0, 1]));
        pf.add(&shape, 0, 7, 1);

        assert_eq!(vec![0], pf.check_rows(&vec![0]));
        assert_eq!(vec![1], pf.check_rows(&vec![1]));
        assert_eq!(vec![0, 1], pf.check_rows(&vec![0, 1]));

    }

    #[test]
    fn test_playfield_clear_rows() {

        // Test clearing 4 rows

        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            10,
            BLACK,
            RotationType::SRS
        );
        pf.add(&shape, 18, 1, 0);
        
        for i in 1..pf.n_cols() - 1 {
            assert_eq!(1, pf.get_cell(18, i));
            assert_eq!(1, pf.get_cell(19, i));
            assert_eq!(1, pf.get_cell(20, i));
            assert_eq!(1, pf.get_cell(21, i));
        }

        pf.clear_rows(&vec![18, 19, 20, 21]);

        for i in 1..pf.n_cols() - 1 {
            assert_eq!(0, pf.get_cell(18, i));
            assert_eq!(0, pf.get_cell(19, i));
            assert_eq!(0, pf.get_cell(20, i));
            assert_eq!(0, pf.get_cell(21, i));
        }

        // Test clearing 2 rows with some garbage above
        // that needs to be moved down

        let mut pf = Playfield::new();

        let shape = Shape::new(
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            10,
            BLACK,
            RotationType::SRS
        );

        pf.add(&shape, 19, 1, 0);

        for i in 1..6 {
            assert_eq!(0, pf.get_cell(19, i));
        }
        for i in 7..pf.n_cols() - 1 {
            assert_eq!(1, pf.get_cell(19, i));
        }
        for i in 1..pf.n_cols() - 1 {
            assert_eq!(1, pf.get_cell(20, i));
            assert_eq!(1, pf.get_cell(21, i));
        }

        pf.clear_rows(&vec![20, 21]);
        
        for i in 1..pf.n_cols() - 1 {
            assert_eq!(0, pf.get_cell(19, i));
            assert_eq!(0, pf.get_cell(20, i));
        }
        for i in 1..6 {
            assert_eq!(0, pf.get_cell(21, i));
        }
        for i in 7..pf.n_cols() - 1{
            assert_eq!(1, pf.get_cell(21, i));
        }

        // Test clearing 3 rows with some garbage
        // in between

        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            10,
            BLACK,
            RotationType::SRS
        );
        pf.add(&shape, 18, 1, 0);
        
        for i in 1..pf.n_cols() - 1 {
            assert_eq!(1, pf.get_cell(18, i));
            assert_eq!(1, pf.get_cell(19, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(20, i));
            } else {
                assert_eq!(1, pf.get_cell(20, i));
            }
            assert_eq!(1, pf.get_cell(21, i));
        }

        pf.clear_rows(&vec![18, 19, 21]);

        for i in 1..pf.n_cols() - 1 {
            assert_eq!(0, pf.get_cell(18, i));
            assert_eq!(0, pf.get_cell(19, i));
            assert_eq!(0, pf.get_cell(20, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(21, i));
            } else {
                assert_eq!(1, pf.get_cell(21, i));
            }
        }

        // Test clearing 2 rows with some garbage
        // in between (alterbate rows)

        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            10,
            BLACK,
            RotationType::SRS
        );
        pf.add(&shape, 18, 1, 0);
        
        for i in 1..pf.n_cols() - 1 {
            assert_eq!(1, pf.get_cell(19, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(18, i));
                assert_eq!(0, pf.get_cell(20, i));
            } else {
                assert_eq!(1, pf.get_cell(18, i));
                assert_eq!(1, pf.get_cell(20, i));
            }
            assert_eq!(1, pf.get_cell(21, i));
        }

        pf.clear_rows(&vec![19, 21]);

        for i in 1..pf.n_cols() - 1 {
            assert_eq!(0, pf.get_cell(18, i));
            assert_eq!(0, pf.get_cell(19, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(20, i));
                assert_eq!(0, pf.get_cell(21, i));
            } else {
                assert_eq!(1, pf.get_cell(20, i));
                assert_eq!(1, pf.get_cell(21, i));
            }
        }

        // Test clearing 2 rows with some garbage
        // in between (consecutive rows)

        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            10,
            BLACK,
            RotationType::SRS
        );
        pf.add(&shape, 18, 1, 0);
        
        for i in 1..pf.n_cols() - 1 {
            assert_eq!(1, pf.get_cell(18, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(19, i));
                assert_eq!(0, pf.get_cell(20, i));
            } else {
                assert_eq!(1, pf.get_cell(19, i));
                assert_eq!(1, pf.get_cell(20, i));
            }
            assert_eq!(1, pf.get_cell(21, i));
        }

        pf.clear_rows(&vec![18, 21]);

        for i in 1..pf.n_cols() - 1 {
            assert_eq!(0, pf.get_cell(18, i));
            assert_eq!(0, pf.get_cell(19, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(20, i));
                assert_eq!(0, pf.get_cell(21, i));
            } else {
                assert_eq!(1, pf.get_cell(20, i));
                assert_eq!(1, pf.get_cell(21, i));
            }
        }
        
        // Test clearing 1 row with some garbage

        let mut pf = Playfield::new();
        let shape = Shape::new(
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            10,
            BLACK,
            RotationType::SRS
        );
        pf.add(&shape, 18, 1, 0);
        
        for i in 1..pf.n_cols() - 1 {
            if i == 1 {
                assert_eq!(0, pf.get_cell(18, i));
                assert_eq!(0, pf.get_cell(19, i));
                assert_eq!(0, pf.get_cell(20, i));
            } else {
                assert_eq!(1, pf.get_cell(18, i));
                assert_eq!(1, pf.get_cell(19, i));
                assert_eq!(1, pf.get_cell(20, i));
            }
            assert_eq!(1, pf.get_cell(21, i));
        }

        pf.clear_rows(&vec![21]);

        for i in 1..pf.n_cols() - 1 {
            assert_eq!(0, pf.get_cell(18, i));
            if i == 1 {
                assert_eq!(0, pf.get_cell(19, i));
                assert_eq!(0, pf.get_cell(20, i));
                assert_eq!(0, pf.get_cell(21, i));
            } else {
                assert_eq!(1, pf.get_cell(19, i));
                assert_eq!(1, pf.get_cell(20, i));
                assert_eq!(1, pf.get_cell(21, i));
            }
        }

    }

    #[test]
    fn test_playfield_is_empty() {
        let mut pf = Playfield::new();
        assert_eq!(true, pf.is_empty(0));
        assert_eq!(true, pf.is_empty(1));
        assert_eq!(true, pf.is_empty(2));
        assert_eq!(true, pf.is_empty(3));

        let shape = Shape::new(
            vec![1, 1, 1, 1,
                 1, 1, 1, 1],
            4,
            BLACK,
            RotationType::SRS
        );

        pf.add(&shape, 0, 1, 0);
        assert_eq!(false, pf.is_empty(0));
        assert_eq!(false, pf.is_empty(1));
        assert_eq!(true, pf.is_empty(2));
        assert_eq!(true, pf.is_empty(3));

    }

}