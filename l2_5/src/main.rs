extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    let yoda: usize = 874;
    let ss: usize = 65;

    addstr(&format!("Yoda is {} years old\n", yoda));
    addstr(&format!("He has collected {} years\n", yoda - ss));
    addstr("of Social Security.");

    refresh();
    getch();
    endwin();
}
