use crate::cell;


pub struct Universe {
    pub cells: Vec<Vec<cell::State>>
}


impl Universe {
    pub fn new(w: u16, h: u16) -> Universe {
        Universe {
            cells: (0..w).map(|_| (0..h).map(|_| cell::State::Dead).collect()).collect()
        }
    }

    pub fn cycle(&mut self) {
        for i in 1..self.cells.len() {
            for j in 1..self.cells[i].len() {
                let mut num_neighbors = 0;
                for ii in (i-1)..(i+1) {
                    for jj in (j-1)..(j+1) {
                        match self.cells[ii][jj] {
                            cell::State::Alive => {num_neighbors += 1;},
                            _ => {}
                        }
                    }
                }
                match (&self.cells[i][j], num_neighbors) {
                    (cell::State::Alive, 0...1) => {self.cells[i][j] = cell::State::Dead;},
                    (cell::State::Alive, 3...8) => {self.cells[i][j] = cell::State::Dead;},
                    (cell::State::Dead,  3    ) => {self.cells[i][j] = cell::State::Alive;},
                    (_, _) => {}
                }
            }
        }
    }
}