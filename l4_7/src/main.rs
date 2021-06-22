extern crate ncurses;

use ncurses::*;

fn main() {
    let mut row: i32 = 0;
    let mut col: i32 = 0;

    initscr();

    addstr("Type some text; '~' to end:\n");
    loop {
       let ch = getch();
       if ch == '~' as i32 {
           break;
       }

       if ch == '\n' as i32 {
           addstr("\n");
       }
    }

    getyx(stdscr(), &mut row, &mut col);
    addstr(&format!("\n\nThe cursor was at row {}, column {}\n", row, col));
    addstr("when you stopped typing.");

    getch();
    endwin();
}
