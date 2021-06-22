extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr(&format!("Window size is {} rows, {} columns\n", LINES(), COLS()));

    getch();
    endwin();
}
