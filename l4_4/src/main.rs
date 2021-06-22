extern crate ncurses;

use ncurses::*;

fn main() {
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    initscr();
    getmaxyx(stdscr(), &mut rows, &mut cols);
    rows = rows - 1;
    cols = cols - 1;

    mvaddch(0, 0, '*' as u32);
    napms(500);

    mvaddch(0, cols, '*' as u32);
    napms(500);

    mvaddch(rows, 0, '*' as u32);
    napms(500);

    mvaddch(rows, cols, '*' as u32);

    getch();
    endwin();
}
