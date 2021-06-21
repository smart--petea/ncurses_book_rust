extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLUE);
    bkgd(COLOR_PAIR(1) | ('.' as chtype));

    addstr("So this is what a color screen looks like?\n");

    getch();
    endwin();
}
