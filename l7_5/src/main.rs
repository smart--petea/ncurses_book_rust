extern crate ncurses;

use ncurses::*;

fn main() {
    let mut buffer: String = String::with_capacity(80);
    initscr();

    addstr("Ype on the keyboard whilst I wait...\n");
    refresh();
    napms(5000);

    addstr("Here is what you typed:\n");
    flushinp();
    getnstr(&mut buffer, 80);
    refresh();

    endwin();
}
