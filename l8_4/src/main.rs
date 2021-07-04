extern crate ncurses;

use ncurses::*;

fn main() -> Result<(), &'static str> {
    let a: WINDOW;
    let b: WINDOW;
    let c: WINDOW;
    let d: WINDOW;

    initscr();

    //calcualte window sizes and locations
    let mut maxx: i32 = 0;
    let mut maxy: i32 = 0;
    getmaxyx(stdscr(), & mut maxy, & mut maxx);
    let halfx: i32 = maxx >> 1;
    let halfy: i32 = maxy >> 1;
    
    refresh();

    //create four windows to fill the screen
    a = newwin(halfy, halfx, 0, 0);
    if a.is_null() {
        return bomb();
    }

    b = newwin(halfy, halfx, 0, halfx);
    if b.is_null() {
        return bomb();
    }

    c = newwin(halfy, halfx, halfy, 0);
    if c.is_null() {
        return bomb();
    }

    d = newwin(halfy, halfx, halfy, halfx);
    if d.is_null() {
        return bomb();
    }

    //Write to each window
    mvwaddstr(a, 0, 0, "This is window A\n");
    wrefresh(a);
    mvwaddstr(b, 0, 0, "This is window B\n");
    wrefresh(b);
    mvwaddstr(c, 0, 0, "This is window C\n");
    wrefresh(c);
    mvwaddstr(d, 0, 0, "This is window D\n");
    wrefresh(d);
    refresh();
    getch();

    endwin();
    Ok(())
}

fn bomb() -> Result<(), &'static str> {
    addstr("Unable to allocate memory for new window.\n");
    refresh();
    endwin();

    return Err("Unable to allocate memory for new window.")
}
