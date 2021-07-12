extern crate ncurses;

use ncurses::*;

const TSIZE: i32 = 18;

fn main() {
    initscr();

    let mut maxy: i32 = 0;
    let mut maxx: i32 = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);
    let x: i32 = (maxx - TSIZE) >> 1;

    let b: WINDOW = newwin(1, TSIZE, 0, x);
    /*
    bkgd('.' as u32);
    refresh();
    */
    waddstr(b, "I'm getting sick!");

    for y in 1..maxy {
        mvwin(b, y, x);
        wrefresh(b);
        /*
        touchline(stdscr(), y-1, 1);
        refresh();
        */
        getch();
    }

    endwin();
}
