extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    start_color(); //remember to check for errors!
    init_pair(1, COLOR_WHITE, COLOR_BLUE);
    init_pair(2, COLOR_RED, COLOR_YELLOW);
    init_pair(3, COLOR_CYAN, COLOR_WHITE);

    //create subwindow and remember to check for errors!
    let sonny: WINDOW = subwin(stdscr(),5, 20, 10, 30);
    let babygirl: WINDOW = derwin(stdscr(), 5, 20, 1, 50);

    //color windows and splash some text
    bkgd(COLOR_PAIR(1));
    addstr("Hello, son, hello baby girl.");
    wbkgd(sonny, COLOR_PAIR(2));
    waddstr(sonny, "Hello, Dad.");
    wbkgd(babygirl, COLOR_PAIR(3));
    waddstr(babygirl, "Hello, Papa.");
    refresh();
    getch();

    endwin();
}
