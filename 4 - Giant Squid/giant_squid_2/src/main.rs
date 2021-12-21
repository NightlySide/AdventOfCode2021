use std::{fs, process::exit};

// A cell on the board
#[derive(Debug, Copy, Clone)]
struct Cell {
    value: usize,
    marked: bool,
}

impl Cell {
    fn new(value: usize) -> Self {
        Cell {
            value,
            marked: false,
        }
    } 
}

// The board containing cells
#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: vec![vec![Cell::new(0); 5]; 5],
        }
    }

    fn get_cells(&mut self) -> impl Iterator<Item = (usize, usize, &mut Cell)> {
        self.cells
            .iter_mut()
            .enumerate()
            .flat_map(|(i, row)| row.iter_mut().enumerate().map(move |(j, cell)| (i, j, cell)))
    }
    
    fn play_number(&mut self, number: usize) {
        for (_, _, cell) in self.get_cells() {
            if cell.value == number {
                cell.marked = true;
                return;
            }
        }
    }

    fn is_winner(&self) -> bool {
        // testing rows
        for row in &self.cells {
            let mut row_win = true;
            for cell in row {
                if !cell.marked {
                    row_win = false;
                    break;
                }
            }

            if row_win { return true; }
        }

        // testing cols
        for index in [0..self.cells.get(0).unwrap().len()] {
            let mut col_win = true;
            for row in &self.cells {
                let cell = row.get(index.clone()).unwrap()[0];
                if !cell.marked {
                    col_win = false;
                    break;
                }
            }

            if col_win { return true; }
        }
        false
    }

    fn get_score(&mut self, number: usize) -> usize {
        let somme: usize = self.get_cells().filter(|(_, _, cell)| !cell.marked).map(|(_, _, cell)| cell.value).sum();
        somme * number
    }
}

// Reads the input file and outputs the board and the order
fn read_input_file(file_path: &str) -> (Vec<Board>, Vec<usize>) {
    let data = fs::read_to_string(file_path)
        .expect(&format!("Cannot open the file '{}'", file_path));

    let mut boards: Vec<Board> = Vec::new();
    let mut order: Vec<usize> = Vec::new();
    let mut cur_board: Board = Board::new();

    for (index, line) in data.split("\n").enumerate() {
        let line = line.trim();

        if index == 0 {
            line.split(",").for_each(|el| order.push(el.parse().unwrap()));
        } else {
            // if this is a new board
            if line.eq("") {
                if index != 1 {
                    boards.push(cur_board);
                }
                cur_board = Board::new();
            } else {
                // getting the row number
                let row_nb = (index - 2) % 6;
                // getting the numbers
                let numbers: Vec<usize> = line.split(" ")
                    .filter(|el| (*el).ne(""))
                    .map(|el| el.parse::<usize>().unwrap())
                    .collect();

                // putting numbers in the board
                numbers.iter().enumerate().for_each(|(idx, nb)| {
                    cur_board.cells[row_nb][idx] = Cell::new(*nb);
                });
            }
        }
    }

    (boards, order)
}

fn play_turn(boards: &mut Vec<Board>, number: usize) {
    for board in boards.iter_mut() {
        board.play_number(number);
    }
}

fn main() {
    let (mut boards, order) = read_input_file("input.txt");

    println!("Boards: {:?} elements", boards.len());
    println!("Order: {:?} elements", order.len());

    for nb in order {
        println!("Playing number : {}", nb);
        play_turn(&mut boards, nb);

        if boards.len() == 1 {
            let last_board = boards.first_mut().unwrap();
            if last_board.is_winner() {
                println!("Last board won with number {} and a score of {}", nb, last_board.get_score(nb));
                exit(0);
            }
        }
        boards = boards.into_iter().filter(|board| !board.is_winner()).collect();
    }
}
