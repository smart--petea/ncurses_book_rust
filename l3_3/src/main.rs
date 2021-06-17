extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    if !has_colors() {
        bomb("Terminal cannot do colors");
    }

    if start_color() != OK {
        bomb("Iunable to start colors");
    }

    addstr("Colors have been properly initialized.\n");
    addstr("Congratulations!\n");
    addstr(&format!("NCurses reports that you can use {} colors,\n", COLORS()));
    addstr(&format!("and {} color pairs.", COLOR_PAIRS()));

    getch();
    endwin();
}

fn bomb(msg: &str) {
    endwin();
    println!("{}", msg);
    std::process::exit(1);
}
