extern crate ncurses;

use ncurses::*;

fn main() {
    const NEW_COLOR: i16 = 1;
    const RED: i16 = 1000;
    const GREEN: i16 = 750;
    const BLUE: i16 = 750;

    initscr();
    start_color();
    if !can_change_color() {
        addstr("This probably won't work, but anyway:\n");
    }

    init_color(NEW_COLOR, RED, GREEN, BLUE);

    init_pair(1, NEW_COLOR, COLOR_BLACK);
    attrset(COLOR_PAIR(1));
    addstr(&format!("This is the new color {}.\n", NEW_COLOR));

    getch();
    endwin();
}
