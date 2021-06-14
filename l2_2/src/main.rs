extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    let text: String = "Greetings from NCurses!".to_string();

    for ch in text.chars() {
        addch(ch as u32);
        refresh();
        napms(100);
    }
    getch();

    endwin();
}
