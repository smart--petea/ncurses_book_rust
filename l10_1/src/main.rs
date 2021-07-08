extern crate ncurses;

use ncurses::*;

fn main() {
    let text1: String = "Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh eusmod tincidunt ut laoreet dolore magna aliquam erat volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo conequat.;".to_string();
    let text2: String = "Four score and serven years ago our fathers brought forth on this continent, a new nation, conceived in Liberty, and dedicated to the proposition that all men are created equal.".to_string();
    initscr();

    refresh();

    //build windows
    let alpha: WINDOW = newwin(0, 0, 0, 0); //Remember to check for errors!

    addstr(&text1); //add text to sdscr and wait
    refresh();
    getch();

    waddstr(alpha, &text2); //Show win alpha and wait
    wrefresh(alpha);
    getch();

    //Copy text from one window to the other, non-destructively
    overlay(stdscr(), alpha);
    wrefresh(alpha);
    getch();

    endwin();
}
