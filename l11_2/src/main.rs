extern crate ncurses;

use ncurses::*;
use std::fs;

const WIDE: i32 = 19;
const TALL: i32 = 24;
const SPACER: i32 = 5;
const FILENAME: &str = "file.txt";

fn main() {
    initscr();

    let p: WINDOW = newpad(200, WIDE + 1);
    if p.is_null() {
        bomb("Unable to ope file\n".to_string());
    }

    let contents: String = match fs::read_to_string(FILENAME) {
        Ok(s) => s,
        Err(err) => return bomb(format!("{}", err)),
    };

    //display file's contents on the pad
    for ch in contents.chars() {
        waddch(p, ch as u32);
    }


    //display the pad's contents on the screen
    prefresh(p, 0,0, 0, 0, TALL-1, WIDE);
    prefresh(p, TALL, 0, 0, WIDE + SPACER, TALL - 1, WIDE * 2 + SPACER);
    prefresh(p, TALL*2, 0, 0, WIDE*2 + SPACER * 2, TALL - 1, WIDE * 3 + SPACER * 2);
    wgetch(p);

    endwin();
}

fn bomb(message: String) {
    addstr(&message);
    refresh();
    getch();
    endwin();
    panic!("{}", message);
}
