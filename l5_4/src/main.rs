extern crate ncurses;

use ncurses::*;
use std::io;

fn main() -> io::Result<()> {
    let text: &str = "Stock Market Swells! DOW tops 15.000!";

    initscr();

    for ch in text.chars().rev() {
        mv(5,5);
        insch(ch as u32);
        refresh();
        napms(100);
    }

    getch();
    endwin();

    Ok(())
}
