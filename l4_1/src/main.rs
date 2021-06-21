extern crate ncurses;

use ncurses::*;

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    initscr();

    getmaxyx(stdscr(), &mut x, &mut y);
    addstr(&format!("Window size is {} rows, {} columns. \n", x, y));

    getch();
    endwin();
}
