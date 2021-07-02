extern crate ncurses;

use ncurses::*;

const MENUMAX: usize = 6;
const NEW_LINE: i32 = '\n' as i32;

fn main() {
    let mut menuitem: usize = 0;

    initscr();

    drawmenu(&menuitem);
    keypad(stdscr(), true);
    noecho();
    loop {
        match getch() {
            KEY_DOWN => {
                menuitem = menuitem + 1;
                if menuitem > MENUMAX - 1 {
                    menuitem = 0;
                }
            }
            KEY_UP => {
                if menuitem == 0 {
                    menuitem = MENUMAX;
                }
                menuitem = menuitem - 1;
            }
            NEW_LINE => {
                break;
            }
            _ => (),
        }
        drawmenu(&menuitem);
    }
    echo();

    endwin();
}

fn drawmenu(item: &usize) {
    let mainmenu: &str = "Main Menu";
    let menu: [&str; MENUMAX] = [
        "Answer E-mail",
        "Off to the Web",
        "Word processing",
        "Financial management",
        "Maintanance",
        "Shutdown"
    ];

    clear();
    addstr(mainmenu);
    for i in 0..MENUMAX {
        if i == *item {
            attron(A_REVERSE());
        }

        mvaddstr((3 + (i * 2)) as i32, 20, menu[i]);
        attroff(A_REVERSE());
    }

    mvaddstr(17, 25, "Use arrow keys to move; Enter to select:");
    refresh();
}
