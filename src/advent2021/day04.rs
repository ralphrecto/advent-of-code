use std::iter::Enumerate;

use fileutil;

const BOARD_EDGE_SIZE: usize = 5;

#[derive(Debug)]
struct Board(Vec<Vec<i32>>);

impl Board {
    const UNUSED: i32 = -2;
    pub const MARKER: i32 = -1;

    fn check_consecutive<F>(self: &Self, mark_gen: F) -> bool
        where F: Fn(&Board, usize, usize) -> i32
    {
        for i in 0..self.0.len() {
            let all_i_marked = (0..self.0.len())
            .into_iter()
            .all(|j| mark_gen(self, i, j) == -1);

            if all_i_marked {
                return true;
            }
        }

        false
    }

    pub fn check_bingo(self: &Self) -> bool {
        self.check_consecutive(|b, i, j| b.0[i][j]) ||
        self.check_consecutive(|b, i, j| b.0[j][i])
    }

    pub fn new(size: usize) -> Board {
        Board(vec![vec![Board::UNUSED; size]; size])
    }

    pub fn set(self: &mut Self, i: usize, j: usize, val: i32) {
        self.0[i][j] = val;
    }

    pub fn mark_if_match(self: &mut Self, i: usize, j: usize, val: i32) -> bool {
        if self.0[i][j] == val {
            self.0[i][j] = Board::MARKER;
            return true;
        }
        
        false
    }

    pub fn dimensions(self: &Self) -> (usize, usize) {
        (self.0.len(), self.0.len())
    }

    pub fn get_unmarked(self: &Self) -> Vec<i32> {
        self.0.iter()
        .flat_map(|v| v.iter())
        .map(|v| *v)
        .filter(|v| *v != Board::MARKER)
        .collect()
    }
}

#[derive(Debug)]
struct Input {
    numbers: Vec<i32>,
    boards: Vec<Board>
}

#[derive(Clone,Copy)]
enum ParseState {
    Numbers,
    Blank,
    BoardLine(usize)
}

fn parse_input(filename: &str) -> Input {
    let lines = fileutil::read_lines(filename).unwrap();
    let mut current_state = ParseState::Numbers;

    let mut numbers: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut current_board: Board = Board::new(BOARD_EDGE_SIZE);
    for line in lines {
        match current_state {
            ParseState::Numbers => {
                let parsed_num_iter = line.split(",")
                .into_iter()
                .map(|n| n.parse::<i32>().unwrap());

                numbers.extend(parsed_num_iter);
                current_state = ParseState::Blank;
            },
            ParseState::Blank => {
                current_state = ParseState::BoardLine(BOARD_EDGE_SIZE);
            },
            ParseState::BoardLine(line_num) => {
                let board_row_iter = line.split_whitespace()
                .into_iter()
                .map(|board_elt| board_elt.parse::<i32>().unwrap())
                .enumerate();

                for (i, num) in board_row_iter {
                    current_board.set(line_num - 1, i, num);
                }

                if line_num == 1 {
                    // New board 
                    boards.push(current_board);
                    current_board = Board::new(BOARD_EDGE_SIZE);
                    current_state = ParseState::Blank;
                } else {
                    current_state = ParseState::BoardLine(line_num - 1);
                }
            }
        }
    }

    Input { numbers, boards }
}


fn part1(input: &mut Input) {
    'outermost: for number in &input.numbers {
        for board in &mut input.boards {
            let (num_rows, num_cols ) = board.dimensions();

            for i in 0..num_rows {
                for j in 0..num_cols {
                    if board.mark_if_match(i, j, *number) && board.check_bingo() {
                        let unmarked_sum: i32 = board.get_unmarked().iter().sum();
                        println!("{}", unmarked_sum * (*number));

                        break 'outermost;
                    }
                }
            }
        }
    }
}

pub fn run() {
    let mut input = parse_input("data/2021/04.txt");
    part1(&mut input);
}