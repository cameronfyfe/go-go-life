use crate::cell;


pub struct Universe {
    pub cells: Vec<Vec<cell::State>>,
    cells_new: Vec<Vec<cell::State>>
}


impl Universe {
    pub fn new(w: u16, h: u16) -> Universe {
        Universe {
            cells: (0..w).map(|_| (0..h).map(|_| cell::State::Dead).collect()).collect(),
            cells_new: (0..w).map(|_| (0..h).map(|_| cell::State::Dead).collect()).collect()
        }
    }

    pub fn add_box(&mut self, x: usize, y: usize) {
        self.cells[x  ][y  ] = cell::State::Alive;
        self.cells[x  ][y+1] = cell::State::Alive;
        self.cells[x+1][y  ] = cell::State::Alive;
        self.cells[x+1][y+1] = cell::State::Alive; 
    }

    pub fn add_glider(&mut self, x: usize, y: usize) {
        // TODO: add direction and permutation args
        self.cells[x  ][y  ] = cell::State::Alive;
        self.cells[x+1][y+1] = cell::State::Alive;
        self.cells[x+2][y+1] = cell::State::Alive;
        self.cells[x  ][y+2] = cell::State::Alive;
        self.cells[x+1][y+2] = cell::State::Alive;
    }

    pub fn cycle(&mut self) {
        // TODO: handle edges, both normal and loop-around modes
        for i in 1..self.cells.len()-1 {
            for j in 1..self.cells[i].len()-1 {
                // TODO: count neighbors better
                let mut num_neighbors = 0;
                num_neighbors += if self.cells[i-1][j-1] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i-1][j  ] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i-1][j+1] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i  ][j-1] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i  ][j+1] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i+1][j-1] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i+1][j  ] == cell::State::Alive {1} else {0};
                num_neighbors += if self.cells[i+1][j+1] == cell::State::Alive {1} else {0};
                // Apply Game Rules
                match (&self.cells[i][j], num_neighbors) {
                    (cell::State::Alive, 0...1) => {self.cells_new[i][j] = cell::State::Dead;},
                    (cell::State::Alive, 4...8) => {self.cells_new[i][j] = cell::State::Dead;},
                    (cell::State::Dead,  3    ) => {self.cells_new[i][j] = cell::State::Alive;},
                    (_, _) => {self.cells_new[i][j] = self.cells[i][j];}
                }
            }
        }
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                self.cells[i][j] = self.cells_new[i][j];
            }
        }
    }
}