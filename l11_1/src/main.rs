extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    let p: WINDOW = newpad(50, 100);
    if p.is_null() {
        addstr("Unable to create new pad");
        refresh();
        endwin();
        return;
    }

    addstr("New pad created");
    refresh();
    getch();

    endwin();
}
