extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    addstr("Goodbye, cruel C programing!");

    getch();
    endwin();
}
