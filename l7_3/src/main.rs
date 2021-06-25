extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr("Press any key to end this program: ");
    while !khbit() {}

    getch();
    endwin();
}

fn khbit() -> bool {
    nodelay(stdscr(), true);
    noecho();

    let mut r: bool;

    let ch: i32 = getch();
    if ch == ERR {
        r = false;
    } else {
        r = true;
        ungetch(ch);
    }

    echo();
    nodelay(stdscr(), false);

    return r;
}
