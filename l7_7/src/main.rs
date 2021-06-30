extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    keypad(stdscr(), true);

    const NEW_LINE: i32 = '\n' as i32;
    loop {
        match getch() {
            KEY_DOWN  => addstr("Down\n"),
            KEY_UP => addstr("Up\n"),
            KEY_LEFT => addstr("Left\n"),
            KEY_RIGHT => addstr("Right\n"),
            NEW_LINE => break,
            _ => continue,
        };

        refresh();
    }

    endwin();
}
