use std::{process::exit};

const BOARD_CAP: usize = 100;

fn main() {
    let mut board : [u32; BOARD_CAP] = [0; BOARD_CAP];
    board[BOARD_CAP - 2] = 1;

    for _i in 0..(BOARD_CAP - 2) {
        for j in 0..BOARD_CAP {
            print!(
                "{}",
                match board[j] {
                    0 => ' ',
                    1 => '*',
                    _ => exit(1),
                }
            );
        }
        print!("\n");
        let mut pattern = (board[0]<<1)|board[1];
        for j in 1 ..(BOARD_CAP-1){
            pattern = ((pattern<<1)&7)|board[j+1];
            board[j] = (110 >> pattern)&1;
        }
    }
}
