extern crate ncurses;

use ncurses::*;
use std::fs;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "gettysburg.txt";

fn main() {
    initscr();

    let mut maxy: i32 = 0;
    let mut maxx: i32 = 0;
    getmaxyx(stdscr(), &mut maxy, &mut maxx);

    //create window lister
    let lister: WINDOW = newwin(maxy, maxx >> 1, 0, maxx >> 2);
    if lister.is_null() {
        addstr("Unable to create window\n");
        refresh();
        getch();
        endwin();
        return;
    }

    //open the file
    let file = match fs::File::open(FILENAME) {
        Ok(f) => f,
        Err(e) => {
            addstr("unable to open file\n");
            refresh();
            getch();
            endwin();

            panic!("{}", e)
        }
    };

    refresh();

    //display the file's contents
    let mut file = BufReader::new(file);
    for line in file.lines() {
        match line {
            Ok(l) => {
                for ch in l.chars() {
                    waddch(lister, ch as u32);
                    wrefresh(lister);
                }
            }
            Err(e) => {
                addstr("lines failed\n");
                refresh();
                getch();
                endwin();
                panic!("{}", e);
            }
        }
    }

    getch();
    endwin();
}
