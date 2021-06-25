extern crate ncurses;

use ncurses::*;

fn main() {
    let mut value: i32 = 0;

    initscr();

    addstr("Press any key to begin:\n");
    refresh();
    getch();

    nodelay(stdscr(), true);
    addstr("Press any key to stop the insane loop!\n");
    
    while getch() == ERR {
        addstr(&format!("{}\r", value));
        value = value + 1;
        refresh();
    }

    endwin();
}
