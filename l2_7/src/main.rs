extern crate ncurses;

use ncurses::*;

fn main() {
    let mut first: String = String::new();
    let mut last: String = String::new();

    initscr();

    addstr("What is your first name? ");
    refresh();
    getstr(& mut first);

    addstr("\nWhat is your last name? ");
    refresh();
    getstr(& mut last);

    addstr(&format!("\nPleased to meet you, {} {}!", first, last));
    refresh();

    getch();
    endwin();
}
