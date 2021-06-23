extern crate ncurses;

use ncurses::*;

fn main() {
    let ham1: &str = "To be, or not to be: that is the question:\n";
    let ham2: &str = "Whether 'tis nobler in the mind to suffer\n";
    let ham3: &str = "The slings and arrows of outrageous fortune,\n";
    let ham4: &str = "Or to take arms against a sea of troublers,\n";
    let ham5: &str = "And by opposing end them?\n";

    initscr();

    addstr(ham1);
    addstr(ham3);
    addstr(ham5);

    getch();
    endwin();
}
