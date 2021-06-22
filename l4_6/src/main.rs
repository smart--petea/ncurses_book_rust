extern crate ncurses;

use ncurses::*;

fn main() {
    const COL1: i32 = 5;
    const COL2: i32 = 38;

    initscr();

    mvaddstr(5, COL1, &format!("{: <30}:", "Your name"));
    mvaddstr(5, COL2, &format!("{: <30}", "Art Grockmester"));

    mvaddstr(7, COL1, &format!("{: <30}:", "Your company"));
    mvaddstr(7, COL2, &format!("{: <30}", "Sterling/Worbletyme"));

    mvaddstr(9, COL1, &format!("{: <30}:", "Position"));
    mvaddstr(9, COL2, &format!("{: <30}", "Grand Duke of Finance"));

    mvaddstr(11, COL1, &format!("{: <30}:", "Date hired"));
    mvaddstr(11, COL2, &format!("{: <30}", "October 19, 1993"));

    getch();
    endwin();
}
