extern crate ncurses;

use ncurses::*;

fn main() {
    let text: &str = "Stock Market Swells! DOW tops 15.000!";
    initscr();
    fill();
    refresh();

    for ch in text.chars().rev() {
        mv(5,5);
        insch(ch as u32);
        refresh();
        napms(100);
    }

    getch();
    endwin();
}

fn fill() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    getmaxyx(stdscr(), &mut y, &mut x);
    for _ in 0..y {
        addstr("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z\n");
    }
}
