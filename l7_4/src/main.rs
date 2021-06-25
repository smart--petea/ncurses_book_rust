extern crate ncurses;

use ncurses::*;

fn main() {
    const CAPACITY: usize = 81;
    let mut buffer: String = String::with_capacity(CAPACITY);
    initscr();

    addstr("Type on the keyboard whilst I wait...\n");
    refresh();
    napms(5000);

    addstr("Here is what you typed: \n");
    getnstr(&mut buffer, CAPACITY as i32);

    refresh();
    getch();
    endwin();
}
