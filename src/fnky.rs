//sourced from https://gist.github.com/fnky/286f0cfbaaf203be706d7d3e1ea1ff88


use std::error::Error;
use std::io::{self, Write};

const BOARD_CAP: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let mut board = [0; BOARD_CAP];
    board[BOARD_CAP - 2] = 1;

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = io::BufWriter::new(lock);
    for _i in 0..BOARD_CAP - 2 {
        for j in 0..BOARD_CAP {
            write!(buf, "{}", " *".chars().nth(board[j]).unwrap())?;
        }
        write!(buf, "\n")?;
        let mut pattern = (board[0] << 1) | board[1];
        for j in 1..BOARD_CAP - 1 {
            pattern = ((pattern << 1) & 7) | board[j + 1];
            board[j] = (110 >> pattern) & 1;
        }
    }
    buf.flush()?;
    Ok(())
}
