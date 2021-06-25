extern crate ncurses;

use ncurses::*;

fn main() {
    let mut value: i32 = 0;

    initscr();

    addstr("Press any key to begin:\n");
    refresh();
    getch();

    nodelay(stdscr(), true);
    addstr("Press the Spacebar to stop the insane loop!\n");
    loop {
        addstr(&format!("{}\r", value));
        refresh();

        value = value + 1;
        if getch() == ' ' as i32 {
            break;
        }
    }

    endwin();
}
