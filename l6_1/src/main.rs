extern crate ncurses;

use ncurses::*;

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    initscr();

    getmaxyx(stdscr(), &mut y, &mut x);
    let cmax: i32 = (x * y) / 5;
    for c in 0..cmax {
        addstr("blah ");
    }

    refresh();
    getch();
    endwin();
}
