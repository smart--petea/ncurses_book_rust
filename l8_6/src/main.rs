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
    let mut maxy: i32 = 0;
    let mut maxx: i32 = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);
    let halfx: i32 = maxx >> 2;
    let halfy: i32 = maxy >> 2;

    //create four windows to fill the screen
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

    refresh();
    //Write to each window
    wbkgd(a, COLOR_PAIR(1));
    mvwaddstr(a, 0, 0, "This is window A\n");
    wrefresh(a);

    wbkgd(b, COLOR_PAIR(2));
    mvwaddstr(b, 0, 0, "This is window B\n");
    wrefresh(b);

    wbkgd(c, COLOR_PAIR(3));
    mvwaddstr(c, 0, 0, "This is window C\n");
    wrefresh(c);

    wbkgd(d, COLOR_PAIR(4));
    mvwaddstr(d, 0, 0, "This is window D\n");
    wrefresh(d);

    //Update each window
    loop {
        let ch: i32 = wgetch(a);
        if ch == '~' as i32 {
            break;
        }

        waddch(b, ch as u32);
        waddch(c, ch as u32);
        waddch(d, ch as u32);
        wrefresh(b);
        wrefresh(c);
        wrefresh(d);
    }

    endwin();

    Ok(())
}

fn bomb() -> Result<(), &'static str> {
    addstr("Unable to allocate memory for new window.\n");
    refresh();
    endwin();

    Err("Unable to allocate memory for new window.")
}
