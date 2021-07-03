extern crate ncurses;

use ncurses::*;

fn main() -> Result<(), &'static str> {
    initscr();

    let help: WINDOW = newwin(0, 0, 0, 0);
    if help.is_null() {
        addstr("Unable to allocate window memory\n");
        endwin();

        return Err("Unable to allocate window memory");
    }

    mvwaddstr(help, 6, 32, "Help menu Screen");
    mvwaddstr(help, 9, 28, "Press the ~ key to quit");
    mvwaddstr(help, 12, 28, "Press ENTER to go back");

    /* now start the program loop */
    addstr("Typer Program\n");
    addstr("Press + for help:\n\n");
    refresh();
    noecho();

    const PLUS: i32 = '+' as i32;
    const TILDA: i32 = '~' as i32;
    loop {
        let ch: i32 = getch();
        match getch() {
            PLUS => {
                showhelp(&help);
            }
            TILDA => {
                break;
            }
            _ => {
                addch(ch as u32);
            }
        }
    }

    endwin();
    Ok(())
}

fn showhelp(help: &WINDOW) {
    touchwin(*help);
    wrefresh(*help);
    getch();

    touchwin(stdscr());
    refresh();
}
