// Mich!
//

extern crate ncurses;

use ncurses::*;
use std::cmp;

const FULL_LINE: &str = "-------------------";
const DIRECTIONS: [i32; 4] = [KEY_LEFT, KEY_RIGHT, KEY_DOWN, KEY_UP];

fn print_full_line() {
    printw(FULL_LINE);
}

fn print_data_line() {
    printw("|x|x|x|x|x|x|x|x|x|");
}

#[derive(Default)]
pub struct Grid {
    cur_x: i32,
    cur_y: i32,
}

impl Grid {
    pub fn move_cur(&mut self, direction: i32) {
        if direction == KEY_LEFT {
            self.cur_x -= 1;
        } else if direction == KEY_RIGHT {
            self.cur_x += 1;
        } else if direction == KEY_UP {
            self.cur_y -= 1;
        } else if direction == KEY_DOWN {
            self.cur_y += 1
        } else {
            // TODO err
        }
        self.cur_x = cmp::min(8, cmp::max(0, self.cur_x));
        self.cur_y = cmp::min(8, cmp::max(0, self.cur_y));
        //mvaddch(self.cur_y, self.cur_x, ACS_CKBOARD());
        mv(self.cur_y * 2 + 2, self.cur_x * 2 + 1);
    }
}

fn print_grid() {
    for number in (0..19).rev() {
        mv(1 + number, 0);
        if number % 2 == 0 {
            print_full_line();
        } else {
            print_data_line();
        }
    }
}

fn main() {
    /* Start ncurses. */
    initscr();
    noecho();
    keypad(stdscr(), true);

    /* Print to the back buffer. */
    printw("Welcome to Sudoku in ncurses + rust");
    print_grid();
        mv(2, 1);

    /* Update the screen. */
    refresh();

    let mut grid: Grid = Default::default();

    /* Wait for a key press. */
    loop {
        //mv(22, 0);
        let c = getch();
        //printw(&c.to_string());
        //printw(" - ");
        let res = DIRECTIONS.iter().position(|&s| s == c);
        if res.is_some() {
            grid.move_cur(c);
        }
        //if [KEY_LEFT, KEY_RIGHT, KEY_DOWN, KEY_UP] {
        //    move_cursor(c);
        //}
        refresh();
    }

    /* Terminate ncurses. */
    //endwin();
}
