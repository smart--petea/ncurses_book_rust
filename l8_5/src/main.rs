extern crate ncurses;

use ncurses::*;

fn main() -> Result<(), &'static str> {
    initscr();
    start_color();
    init_pair(1, COLOR_BLACK, COLOR_BLUE);
    init_pair(2, COLOR_BLACK, COLOR_RED);
    init_pair(3, COLOR_BLACK, COLOR_GREEN);
    init_pair(4, COLOR_BLACK, COLOR_CYAN);

    //calculate window sizes and locations
    let mut maxx: i32 = 0;
    let mut maxy: i32 = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);
    let halfx: i32 = maxx >> 1;
    let halfy: i32 = maxy >> 1;

    refresh();

    //create four windowsto fill the screen
    let a: WINDOW = newwin(halfy, halfx, 0, 0);
    if a.is_null() {
        return bomb();
    }

    let b: WINDOW = newwin(halfy, halfx, 0, halfx);
    if b.is_null() {
        return bomb();
    }

    let c: WINDOW = newwin(halfy, halfx, halfy, 0);
    if c.is_null() {
        return bomb();
    }

    let d: WINDOW = newwin(halfy, halfx, halfy, halfx);
    if d.is_null() {
        return bomb();
    }

    //Write to each window
    mvwaddstr(a, 0, 0, "This is window A\n");
    wbkgd(a, COLOR_PAIR(1));
    wrefresh(a);

    mvwaddstr(b, 0, 0, "This is window B\n");
    wbkgd(b, COLOR_PAIR(2));
    wrefresh(b);

    mvwaddstr(c, 0, 0, "This is window C\n");
    wbkgd(c, COLOR_PAIR(3));
    wrefresh(c);

    mvwaddstr(d, 0, 0, "This is window D\n");
    wbkgd(d, COLOR_PAIR(4));
    wrefresh(d);
    getch();

    endwin();

    Ok(())
}

fn bomb() -> Result<(), &'static str> {
    addstr("Unable to allocate memory for new window.\n");
    refresh();
    endwin();

    Err("Unable to allocate memory for new window.")
}
