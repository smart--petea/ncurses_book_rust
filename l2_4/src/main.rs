extern crate ncurses;

use ncurses::*;

fn main() {
    let text1: &str = "Oh give me a clone!\n";
    let text2: &str = "Yes a clone of my own!";
    initscr();

    refresh();
    addstr(text1);
    addstr(text2);
    mv(2, 0);
    addstr("With the Y chromosome changed to the X.");
    getch();
    endwin();
}
