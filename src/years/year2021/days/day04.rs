use std::io::{BufReader, BufRead, Lines};
use std::fs::File;

fn read_board<B: BufRead>(iter: &mut Lines<B>) -> Option<[i64; 25]> {
    let mut ret = [1000; 25];
    let mut index = 0;

    let blank = iter.next();
    blank.as_ref()?;

    for _ in 0..5 {
        let line: Vec<i64> = iter.next().unwrap().unwrap().split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        for x in line {
            ret[index] = x;
            index += 1;
        }
    }
    Some(ret)
}

fn board_wins(board: [i64; 25]) -> bool {
    if board[0] < 0 && board[1] < 0 && board[2] < 0 && board[3] < 0 && board[4] < 0 {
        return true;
    }
    if board[5] < 0 && board[6] < 0 && board[7] < 0 && board[8] < 0 && board[9] < 0 {
        return true;
    }
    if board[10] < 0 && board[11] < 0 && board[12] < 0 && board[13] < 0 && board[14] < 0 {
        return true;
    }
    if board[15] < 0 && board[16] < 0 && board[17] < 0 && board[18] < 0 && board[19] < 0 {
        return true;
    }
    if board[20] < 0 && board[21] < 0 && board[22] < 0 && board[23] < 0 && board[24] < 0 {
        return true;
    }
    if board[0] < 0 && board[5] < 0 && board[10] < 0 && board[15] < 0 && board[20] < 0 {
        return true;
    }
    if board[1] < 0 && board[6] < 0 && board[11] < 0 && board[16] < 0 && board[21] < 0 {
        return true;
    }
    if board[2] < 0 && board[7] < 0 && board[12] < 0 && board[17] < 0 && board[22] < 0 {
        return true;
    }
    if board[3] < 0 && board[8] < 0 && board[13] < 0 && board[18] < 0 && board[23] < 0 {
        return true;
    }
    if board[4] < 0 && board[9] < 0 && board[14] < 0 && board[19] < 0 && board[24] < 0 {
        return true;
    }

    false
}

fn sum_unmarked(board: [i64; 25]) -> i64 {
    let mut sum = 0;

    for num in board {
        if num > 0 {
            sum += num;
        }
    }

    sum
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day04.txt").unwrap());
    let mut iter = f.lines();
    let moves: Vec<i64> = iter.next().unwrap().unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut min_moves: i64 = 1000000;
    let mut sum = 0;

    loop {
        let board = read_board(&mut iter);
        if board.is_none() {
            break;
        }

        let mut board = board.unwrap();

        for (i, called) in moves.iter().enumerate() {
            for (j, square) in board.iter().enumerate() {
                if *square == *called {
                    board[j] = -*called;
                    break;
                }
            }

            if board_wins(board) {
                if i as i64 + 1 < min_moves {
                    min_moves = i as i64 + 1;
                    sum = sum_unmarked(board) * *called;
                }
                break;
            }
        }
    }

    sum
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day04.txt").unwrap());
    let mut iter = f.lines();
    let moves: Vec<i64> = iter.next().unwrap().unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut max_moves: i64 = 0;
    let mut sum_max = 0;

    loop {
        let board = read_board(&mut iter);
        if board.is_none() {
            break;
        }

        let mut board = board.unwrap();

        for (i, called) in moves.iter().enumerate() {
            for (j, square) in board.iter().enumerate() {
                if *square == *called {
                    board[j] = -*called;
                    break;
                }
            }

            if board_wins(board) {
                if i as i64 + 1 > max_moves {
                    max_moves = i as i64 + 1;
                    sum_max = sum_unmarked(board) * *called;
                }
                break;
            }
        }
    }

    sum_max
}