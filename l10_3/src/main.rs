extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    refresh();

    //Build window & wait
    let fred: WINDOW = newwin(0, 0, 0, 0);
    waddstr(fred, "This is the original window, Fred.\n");
    wrefresh(fred);
    getch();

    //Create and show barney
    let barney: WINDOW = dupwin(fred);
    waddstr(barney, "This is the Barney copy of window Fred.\n");
    wrefresh(barney);
    getch();

    //Go back to fred
    waddstr(fred, "Nice to see you again\n");
    wrefresh(fred);
    getch();

    //One more time to barney
    waddstr(barney, "And Barney says 'Hi' as well.\n");
    touchwin(barney);
    wrefresh(barney);
    getch();

    endwin();
}
