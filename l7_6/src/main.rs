extern crate ncurses;

use ncurses::*;

fn main() {
    let mut name: String = String::with_capacity(46);
    let mut password: String = String::with_capacity(9);

    initscr();

    mvaddstr(3, 10, "Enter your name: ");
    refresh();
    getnstr(&mut name, 45);

    mvaddstr(5, 6, "Enter you password: ");
    refresh();
    //noecho();
    getnstr(&mut password, 8);

    endwin();
}
