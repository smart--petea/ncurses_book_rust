extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    center(1, "Penguin Soccer Finals");
    center(5, "Cattle Dung Samples from Temecula");
    center(7, "Catatonic Theater");
    center(9, "Why do Ions Hate Each Other?");

    getch();
    endwin();
}

fn center(row: i32, title: &str) 
{
    let mut len: i32;
    let mut indent: i32;
    let mut y: i32 = 0;
    let mut width: i32 = 0;

    getmaxyx(stdscr(), &mut y, &mut width);

    len = title.len() as i32;
    indent = width - len;
    indent = indent / 2;

    mvaddstr(row, indent, title);
}
