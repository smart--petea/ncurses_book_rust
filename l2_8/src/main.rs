extern crate ncurses;

use ncurses::*;

fn main() {
    let mut first: String = String::new();
    let mut last: String = String::new();

    initscr();
    addstr("Enter the first 3 letters of your first name? ");
    getnstr(& mut first, 3);

    addstr("Enter the first 3 letters of your last name? ");
    getnstr(& mut last, 3);

    addstr("Your secret agent name is ");
    addstr(&format!("{}{}", first, last));
    addch('!' as u32);

    getch();
    endwin();
}
