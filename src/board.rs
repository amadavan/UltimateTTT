use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
    Empty,
}

impl From<Player> for char {
    fn from(state: Player) -> Self {
        match state {
            Player::X => 'X',
            Player::O => 'O',
            Player::Empty => '-',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardStatus {
    InProgress,
    Won(Player),
    Draw,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Move {
    microboard_row: usize,
    microboard_col: usize,
    cell_row: usize,
    cell_col: usize,
}

impl Move {
    pub fn new(microboard_row: usize, microboard_col: usize, cell_row: usize, cell_col: usize) -> Self {
        Move {
            microboard_row,
            microboard_col,
            cell_row,
            cell_col,
        }
    }

    pub fn get_microboard_position(&self) -> (usize, usize) {
        (self.microboard_row, self.microboard_col)
    }

    pub fn get_cell_position(&self) -> (usize, usize) {
        (self.cell_row, self.cell_col)
    }
}

impl From<Move> for (usize, usize, usize, usize) {
    fn from(mv: Move) -> Self {
        (mv.microboard_row, mv.microboard_col, mv.cell_row, mv.cell_col)
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    previous_move: Option<Move>,
    status: BoardStatus,
    cells: Vec<Vec<MicroBoard>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            previous_move: None,
            status: BoardStatus::InProgress,
            cells: vec![vec![MicroBoard::new(); 3]; 3],
        }
    }

    pub fn set_status(&mut self, status: BoardStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> &BoardStatus {
        &self.status
    }

    pub fn get_cells(&self) -> &Vec<Vec<MicroBoard>> {
        &self.cells
    }

    pub fn update_status(&mut self) -> &BoardStatus {
        if self.is_won() {
            // Determine the winner
            for i in 0..3 {
                for j in 0..3 {
                    if self.cells[i][j].status != BoardStatus::InProgress {
                        self.status = self.cells[i][j].status.clone();
                    }
                }
            }
        } else if self.get_available_moves().is_empty() {
            self.status = BoardStatus::Draw;
        }
        &self.status
    }

    pub fn is_won(&self) -> bool {
        // Check rows and columns
        for i in 0..3 {
            if self.cells[i][0].status != BoardStatus::InProgress
                && self.cells[i][0].status == self.cells[i][1].status
                && self.cells[i][1].status == self.cells[i][2].status
            {
                return true;
            }
            if self.cells[0][i].status != BoardStatus::InProgress
                && self.cells[0][i].status == self.cells[1][i].status
                && self.cells[1][i].status == self.cells[2][i].status
            {
                return true;
            }
        }
        // Check diagonals
        if self.cells[0][0].status != BoardStatus::InProgress
            && self.cells[0][0].status == self.cells[1][1].status
            && self.cells[1][1].status == self.cells[2][2].status
        {
            return true;
        }
        if self.cells[0][2].status != BoardStatus::InProgress
            && self.cells[0][2].status == self.cells[1][1].status
            && self.cells[1][1].status == self.cells[2][0].status
        {
            return true;
        }
        false
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        // Check if the game is already won
        if self.is_won() {
            return Vec::new();
        }

        // Check if restricted by the previous board
        if let Some(prev_move) = self.previous_move {
            let (cell_row, cell_col) = prev_move.get_cell_position();
            let microboard = &self.cells[cell_row][cell_col];
            if microboard.status == BoardStatus::InProgress {
                return vec![Move::new(cell_row, cell_col, 0, 0)]; // Cell positions are irrelevant here
            }
        }

        let mut moves = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i][j].status == BoardStatus::InProgress {
                    moves.push(Move::new(i, j, 0, 0));
                }
            }
        }
        moves
    }

    pub fn is_valid_move(
        &self,
        microboard_row: usize,
        microboard_col: usize,
    ) -> bool {
        // Check if the game is already won
        if self.is_won() {
            return false;
        }

        // Check if restricted by the previous board
        if let Some(prev_move) = self.previous_move {
            if (microboard_row, microboard_col) != prev_move.get_cell_position() {
                return false;
            }
        }

        // Check if the specified microboard is available
        let microboard = &self.cells[microboard_row][microboard_col];
        if microboard.status != BoardStatus::InProgress {
            return false;
        }

        true
    }

    pub fn play(
        &mut self,
        microboard_row: usize,
        microboard_col: usize,
        cell_row: usize,
        cell_col: usize,
        player: Player,
    ) -> Result<(), String> {
        // Check if move is valid
        if self.is_won() {
            return Err("Board already won".to_string());
        }
        if microboard_row >= 3 || microboard_col >= 3 {
            return Err("Invalid microboard position".to_string());
        }
        if !self.is_valid_move(microboard_row, microboard_col) {
            println!("Invalid move: previous_move = {:?}, attempted move = ({}, {})", self.previous_move, microboard_row, microboard_col);
            println!("Valid microboards: {:?}", self.get_available_moves());
            return Err("Invalid move based on previous move".to_string());
        }

        // Play the move on the specified microboard
        let microboard = &mut self.cells[microboard_row][microboard_col];
        microboard.play(cell_row, cell_col, player)?;

        // Set the previous move
        self.previous_move = Some(Move::new(microboard_row, microboard_col, cell_row, cell_col));

        // Update the overall board status
        self.update_status();

        Ok(())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--------------------")?;
        for row in &self.cells {
            for microboard_row in 0..3 {
                    write!(f, "|  ")?;
                for microboard in row {
                    for cell in 0..3 {
                        let cell_state: char = microboard.cells[microboard_row][cell].into();
                        write!(f, "{}", cell_state)?;
                    }
                    write!(f, " | ")?;
                }
                writeln!(f)?;
            }
            writeln!(f, "--------------------")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MicroBoard {
    status: BoardStatus,
    cells: Vec<Vec<Player>>,
}

impl MicroBoard {
    pub fn new() -> Self {
        MicroBoard {
            status: BoardStatus::InProgress,
            cells: vec![vec![Player::Empty; 3]; 3],
        }
    }

    pub fn get_status(&self) -> &BoardStatus {
        &self.status
    }

    pub fn get_cells(&self) -> &Vec<Vec<Player>> {
        &self.cells
    }

    pub fn is_won(&self) -> bool {
        // Check rows and columns
        for i in 0..3 {
            if self.cells[i][0] != Player::Empty
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][1] == self.cells[i][2]
            {
                return true;
            }
            if self.cells[0][i] != Player::Empty
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[1][i] == self.cells[2][i]
            {
                return true;
            }
        }
        // Check diagonals
        if self.cells[0][0] != Player::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return true;
        }
        if self.cells[0][2] != Player::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return true;
        }
        false
    }

    pub fn get_available_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i][j] == Player::Empty {
                    moves.push((i, j));
                }
            }
        }
        moves
    }

    fn play(&mut self, row: usize, col: usize, player: Player) -> Result<(), String> {
        // Check if move is valid
        if self.is_won() {
            return Err("MicroBoard already won".to_string());
        }
        if self.cells[row][col] != Player::Empty {
            return Err("Cell already occupied".to_string());
        }

        // Play the move
        self.cells[row][col] = player;

        // Update status
        if self.is_won() {
            self.status = BoardStatus::Won(player);
        } else if self.get_available_moves().is_empty() {
            self.status = BoardStatus::Draw;
        }

        Ok(())
    }
}
