extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLUE);
    init_pair(2, COLOR_RED, COLOR_YELLOW);

    //Create subwindow
    let sonny: WINDOW = subwin(stdscr(), 5, 20, 10, 30);
    if sonny.is_null() {
        addstr("Unable to create subwindow\n");
        refresh();
        getch();
        endwin();
        return;
    }

    //color windows and spalsh some text
    bkgd(COLOR_PAIR(1));
    addstr("Hello, son.");
    wbkgd(sonny, COLOR_PAIR(2));
    waddstr(sonny, "Hello, Data.");
    refresh();
    getch();

    endwin();
}
