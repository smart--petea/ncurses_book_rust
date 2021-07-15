extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    //create a new pad
    let pod: WINDOW = newpad(50, 50);
    if pod.is_null() {
        bomb("Unable to create new pad");
    }

    addstr("New pad created\n");
    refresh();

    //create a subpad
    let pea: WINDOW = subpad(pod, 20, 20, 29, 29);
    if pea.is_null() {
        bomb("Unable to create subpad");
    }

    addstr("Subpad created\n");
    refresh();
    getch();
    endwin();
}

fn bomb(message: &str) {
    addstr(message);
    refresh();
    getch();
    endwin();
    panic!("{}", message)
}
