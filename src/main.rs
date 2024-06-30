use std::{thread, time::Duration};

const BOARD_SIZE: usize = 16;
type Board = [[u8; BOARD_SIZE]; BOARD_SIZE];

const ON: char = 'X';
const OFF: char = ' ';

const FPS: u64 = 4;
const DELAY: Duration = Duration::from_millis(1000 / FPS);



fn transform(grid: [[u8; 3]; 3]) -> u8 {
    let mut num_alive_neighbors = 0;
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue; // Don't check self
            }
            if grid[i][j] != 0 {
                num_alive_neighbors += 1;
            }
        }
    }
    match num_alive_neighbors {
        0 | 1 => 0,
        2 => if grid[1][1] != 0 { 1 } else { 0 },
        3 => 1,
        _ => 0
    }
}

fn step(prev: Board) -> Board{
    let mut next = [[0u8; BOARD_SIZE]; BOARD_SIZE];

    for i in 1..(BOARD_SIZE-1) {
        for j in 1..(BOARD_SIZE-1) {
            let grid = [
                [prev[i-1][j-1], prev[i-1][j], prev[i-1][j+1]],
                [prev[i  ][j-1], prev[i  ][j], prev[i  ][j+1]],
                [prev[i+1][j-1], prev[i+1][j], prev[i+1][j+1]],
            ];
            next[i][j] = transform(grid);
        }
    }

    next
}

fn reset_cursor() {
    // Move cursor to beginning
    print!("\x1b[{}A", BOARD_SIZE);
    print!("\x1b[{}D", 2 * BOARD_SIZE + 3);
}

fn render(b: &Board) {
    for i in 0..BOARD_SIZE {
        print!("| ");
        for j in 0..BOARD_SIZE {
            let c: char = if b[i][j] != 0 { ON } else { OFF };
            print!("{c} ");
        }
        print!("|\n");
    }
}

fn place(board: &mut Board, location: (usize, usize), pattern: &Vec<Vec<u8>>) {
    let (x, y) = location;
    for (i, row) in pattern.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            board[x + i][y + j] = *element;
        }
    }
}


fn main() {
    let mut initial_board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
    let glider: Vec<Vec<u8>> = vec![
        vec![1, 0, 1],
        vec![0, 1, 1],
        vec![0, 1, 0]
    ];
    place(&mut initial_board, (5, 5), &glider);

    render(&initial_board);
    let mut board = step(initial_board);

    loop {
        reset_cursor();
        render(&board);
        board = step(board);
        thread::sleep(DELAY);
    }
}
