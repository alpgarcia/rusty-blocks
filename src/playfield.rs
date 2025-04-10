use crate::shape::Rotation;



pub struct Playfield {
    cells: [usize; 276]
}

impl Playfield {

    const N_COLS: usize = 12;

    pub fn build() -> Self{

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
    pub fn add(&mut self, shape: &dyn Rotation, 
               row: usize, col: isize, r: isize) {

        for i in 0..shape.shape_data().len() {
        
            let shape_row = shape.shape_data().row(i);
            let shape_col = shape.shape_data().col(i);

            let v = *shape.rotate(shape_row, shape_col, r);
            if v > 0 {
                // col can be negative, when there are two or more blank
                // columns on the left side of the shape (that should
                // always be blank columns because collides must be called
                // beforehand and will control this too)
                let cell_idx = self.cell_idx(row + shape_row, 
                    (col + shape_col as isize) as usize);

                self.cells[cell_idx] = v;
            }
        }
    }

    pub fn collides(&self, shape: &dyn Rotation, 
                row: usize, col: isize, r: isize) -> bool{

        let mut collision = false;

        for i in 0..shape.shape_data().len() {

            let shape_row = shape.shape_data().row(i);
            let shape_col = shape.shape_data().col(i);

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

    use crate::shape::Shape;
    use crate::shape::ShapeData;

    use super::Playfield;

    #[test]
    fn test_playfield() {
        let pf = Playfield::build();
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
        let mut pf = Playfield::build();

        let shape = Shape::build(ShapeData::build(
            vec![0, 1, 1, 0,
                 0, 1, 1, 0],
            4,
            BLACK,
        ));

        // Add shape to playfield at (0, 0) with rotation 0
        // (don't overwrite the borders)
        pf.add(&shape, 0, 0, 0);
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
        let mut pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![1, 1, 1, 0,
                 1, 1, 1, 0],
            4,
            BLACK,
        ));
        pf.add(&shape, 0, 0, 0);
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
        let mut pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
        ));
        pf.add(&shape, 0, -1, 0);
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
        let mut pf = Playfield::build();

        let shape = Shape::build(ShapeData::build(
            vec![0, 1, 1, 0,
                 0, 1, 1, 0],
            4,
            BLACK,
        ));

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
        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![1, 1, 1, 0,
                 1, 1, 1, 0,
                 0, 0, 0, 0,
                 0, 0, 0, 0],
            4,
            BLACK,
        ));
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
        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 1, 1, 0,
                 0, 1, 1, 0,
                 0, 0, 0, 0,
                 0, 0, 0, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, -1, 0));
        }

        // Test collision when a shape rotation leaves some blocks
        // out of bounds (col < 0)
        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, -1, 1));
        }

        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 0, 1));
        }

        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0,
                 0, 1, 0, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, -1, 1));
        }

        // Test collision when a shape rotation leaves some blocks
        // out of the playfield on the right edge (col > 11)
        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 10, 1));
        }

        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 11, 1));
        }

        let pf = Playfield::build();
        let shape = Shape::build(ShapeData::build(
            vec![0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0,
                 0, 0, 1, 0],
            4,
            BLACK,
        ));
        // Test left border collisions
        for i in 0..pf.n_rows() {
            assert_eq!(true, pf.collides(&shape, i, 12, 1));
        }

    }

}