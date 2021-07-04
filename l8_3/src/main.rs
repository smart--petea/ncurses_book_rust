extern crate ncurses;

use ncurses::*;

fn main() -> Result<(), &'static str> {
    let two: WINDOW;

    initscr(); 

    addstr("This is the original window, stdscr.\n");
    refresh();
    getch();

    two = newwin(0, 0, 0, 0);
    if two.is_null() {
        addstr("Unable to allocate memory for new window.");
        endwin();

        return Err("Unable to allocate memory for new window.");
    }
    waddstr(two, "This is the new window created!\n");
    wrefresh(two);
    getch();

    delwin(two);
    addstr("The second window was removed.\n");
    touchwin(stdscr());
    refresh();
    getch();

    endwin();

    Ok(())
}
