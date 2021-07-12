extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    let mut maxy: i32 = 0;
    let mut maxx: i32 = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);
    scrollok(stdscr(), true);

    for y in 0..maxy {
        mvprintw(y, 0, &format!("This is boring text written to line {}.", y));
    }
    refresh();
    getch();

    scrl(3);
    refresh();
    getch();

    endwin();
}
