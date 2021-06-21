extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr("Attention!\n");
    beep();
    getch();

    addstr("I said, ATTENTION!\n");
    flash();

    getch();
    endwin();
}
