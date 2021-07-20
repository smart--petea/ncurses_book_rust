extern crate ncurses;

use ncurses::*;
use std::fs;

const FILENAME: &str = "file.txt";
const TALL: i32 = 24;
const WIDE: i32 = 19;
const SPACER: i32 = 5;

fn main() {
    initscr();

    //create a new pad
    let p: WINDOW = newpad(200, WIDE + 1);
    if p.is_null() {
        bomb("Unable to create new pad\n");
    }

    //create three subpads
    let s1: WINDOW = subpad(p, TALL, WIDE + 1, 0, 0);
    if s1.is_null() {
        bomb("Unable to create subpad 1\n");
    }

    let s2: WINDOW = subpad(p, TALL, WIDE + 1, TALL, 0);
    if s2.is_null() {
        bomb("Unable to create subpad 2\n");
    }

    let s3: WINDOW = subpad(p, TALL, WIDE + 1, 2 * TALL, 0);
    if s3.is_null() {
        bomb("Unable to create subpad3\n");
    }

    let contents: String = match fs::read_to_string(FILENAME) {
        Ok(s) => s,
        Err(err) => {
            bomb(&format!("{}", err));
            String::new()
        },
    };

    for ch in contents.chars() {
        waddch(p, ch as u32);
    }

    prefresh(s1, 0, 0, 0, 0, TALL - 1, WIDE);
    prefresh(s2, 0, 0, 0, WIDE + SPACER, TALL - 1, WIDE * 2 + SPACER);
    prefresh(s3, 0, 0, 0, WIDE * 2 + SPACER * 2, TALL - 1, WIDE * 3 + SPACER * 2);
    wgetch(p);

    endwin();
}

fn bomb(msg: &str) {
    addstr(msg);
    refresh();
    getch();
    endwin();

    std::process::exit(1);
}
