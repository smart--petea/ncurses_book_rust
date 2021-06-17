extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    start_color();

    init_pair(1, COLOR_BLACK, COLOR_RED);
    init_pair(2, COLOR_BLUE, COLOR_BLACK);
    attrset(COLOR_PAIR(1));
    addstr("My name is Mr. Black!\n");

    attrset(COLOR_PAIR(2));
    addstr("My name is Mr. Blue!\n");

    attrset(COLOR_PAIR(1));
    addstr("How do you do?\n");

    attrset(COLOR_PAIR(2));
    addstr("How d I do ");

    attron(A_BOLD());
    addstr("what");
    attroff(A_BOLD());
    addch('?' as u32);

    getch();
    endwin();
}
