extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    start_color(); //remember to check for errors
    init_pair(1, COLOR_WHITE, COLOR_BLUE);
    init_pair(2, COLOR_RED, COLOR_YELLOW);
    init_pair(3, COLOR_CYAN, COLOR_GREEN);

    let mut maxy: i32 = 0;
    let mut maxx: i32 = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);

    //create windows - remember to check for errors!
    let grandpa: WINDOW = newwin(maxy-4, maxx-10, 2, 5);
    let father: WINDOW = subwin(grandpa, maxy-8, maxx-20, 4, 10);
    let boy: WINDOW = subwin(father, maxy-16, maxx-40, 8, 20);

    refresh();

    //color windows and splash some text
    wbkgd(grandpa, COLOR_PAIR(1));
    waddstr(grandpa, "Grandpa");
    wbkgd(father, COLOR_PAIR(2));
    waddstr(father, "Father");
    wbkgd(boy, COLOR_PAIR(3));
    waddstr(boy, "Boy");
    wrefresh(grandpa);
    getch();

    endwin();
}
