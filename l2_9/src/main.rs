extern crate ncurses;

#[macro_use]
extern crate scan_fmt;

use ncurses::*;

const UNI: f32 = 4.5;

fn main() {
    let mut input: String = String::new();

    initscr();

    mv(4, 0);
    addstr(&format!("We have Uni today for {:.2}.\n", UNI));
    addstr("How many pieces would you like? ");
    refresh();


    getstr(& mut input);
    let pieces: i32 = scan_fmt!(&input, "{d}", i32).unwrap();

    addstr(&format!("\nYou want {} pieces?\n", pieces));
    addstr(&format!("That will be {:.2}!", UNI * (pieces as f32)));
    refresh();

    getch();
    endwin();
}
