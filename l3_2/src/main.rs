extern crate ncurses;

use ncurses::*;

const COUNT: usize = 5;

fn main() {
    let text: [&str; COUNT]  = ["Do", "you", "find", "this", "silly?"];

    initscr();

    for a in 0..COUNT {
        for b in 0..COUNT {
            if b == a {
                attrset(A_BOLD() | A_UNDERLINE());
            }

            addstr(&text[b]);

            if b == a {
                attroff(A_BOLD() | A_UNDERLINE());
            }

            addch(' ' as u32);
        }
        addstr("\n");
    }

    getch();
    endwin();
}
