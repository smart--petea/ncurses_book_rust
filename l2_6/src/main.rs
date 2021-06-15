extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr("Type a few lines of text\n");
    addstr("Press ~ to quit\n");
    refresh();

    let tilda: i32 = '~' as i32;
    while getch() != tilda {}

    getch();
    endwin();
}
