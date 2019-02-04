// Mich!
//

extern crate ncurses;

use ncurses::*;
use std::cmp;

const FULL_LINE: &str = "-------------------------------------------------------\n";
const CELL_LINE: &str = "|     |     |     |     |     |     |     |     |     |\n";
const DIRECTIONS: [i32; 4] = [KEY_LEFT, KEY_RIGHT, KEY_DOWN, KEY_UP];
//const NUMBERS: [i32, 10] = [0, 1, ]

#[derive(Default)]
pub struct Grid {
    cur_x: i32,
    cur_y: i32,

    base_content: [[i32; 9]; 9],
    content: [[i32; 9]; 9],
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
        self.cur();
    }

    pub fn cur(&mut self) {
        mv(self.cur_y * 4 + 3, self.cur_x * 6 + 3);
    }

    pub fn put_cur(&mut self, x: i32, y: i32) {
        self.cur_x = x;
        self.cur_y = y;
        self.cur();
    }

    pub fn input_num(&mut self, num: i32) {
        attron(A_UNDERLINE());
        printw(&char::from(num as u8).to_string());
        attroff(A_UNDERLINE());
        self.cur();
        self.base_content[self.cur_x as usize][self.cur_y as usize] = num - 48;
    }

    pub fn erase(&mut self) {
        printw(" ");
        self.cur();
        self.base_content[self.cur_x as usize][self.cur_y as usize] = 0;
    }

    pub fn validate_pos(&mut self, x: usize, y: usize) -> bool {
        let curr = self.content[x][y];
        // check row
        for ix in 0..9 {
            if ix == x {
                continue;
            }
            if curr == self.content[ix][y] {
                return false;
            }
        }

        // check line
        for iy in 0..9 {
            if iy == y {
                continue;
            }
            if curr == self.content[x][iy] {
                return false;
            }
        }

        // check block
        let x_offset = x / 3 * 3;
        let y_offset = y / 3 * 3;
        for ix in 0..3 {
            for iy in 0..3 {
                let curr_x = ix + x_offset;
                let curr_y = iy + y_offset;
                if curr_x == x && curr_y == y {
                    continue;
                }
                if curr == self.content[curr_x][curr_y] {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn solve(&mut self) {
        for x in 0..9 {
            for y in 0..9 {
                self.content[x][y] = self.base_content[x][y];
            }
        }

        let mut counter: usize = 0;
        let mut up: bool = true;
        loop {
            let x = counter / 9;
            let y = counter % 9;

            if self.base_content[x][y] == 0 {
                up = false;
                for i in self.content[x][y]..9 {
                    self.content[x][y] = i + 1;
                    self.put_cur(x as i32, y as i32);
                    printw(&(i + 1).to_string());
                    if self.validate_pos(x, y) {
                        up = true;
                        break;
                    }
                }
            }

            if up {
                if counter == 80 {
                    mv(0, 40);
                    printw("win!");
                    break;
                }
                counter += 1;
            } else {
                if self.base_content[x][y] == 0 {
                    self.content[x][y] = 0;
                }
                if counter == 0 {
                    mv(0, 40);
                    break;
                }
                counter -= 1;
            }
            mv(0, 40);
            printw("ICI:");
            printw(&counter.to_string());
            refresh();
        };
    }
}

fn print_grid() {
    mv(1, 0);
    printw(FULL_LINE);

    for _ in 0..9 {
        printw(CELL_LINE);
        printw(CELL_LINE);
        printw(CELL_LINE);

        printw(FULL_LINE);
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

    /* Update the screen. */
    refresh();

    let mut grid: Grid = Default::default();
    grid.cur();

    /* Wait for a key press. */
    loop {
        let c = getch();
        let res = DIRECTIONS.iter().position(|&s| s == c);
        if res.is_some() {
            grid.move_cur(c);
        }
        else if c >= 49 && c <= 57 {
            // numbers from 1 to 9
            grid.input_num(c);
        }
        else if c == 127 {
            // backspace
            grid.erase();
        }
        else if c == 99 {
            grid.solve();
            mv(40, 0);
            for y in 0..9 {
                for x in 0..9 {
                    printw(&grid.content[x][y].to_string());
                }
                printw("\n");
            }
            mv(50, 0);
            for y in 0..9 {
                for x in 0..9 {
                    printw(&grid.base_content[x][y].to_string());
                }
                printw("\n");
            }
        } else {
            //printw(&c.to_string());
        }
        refresh();
    }

    /* Terminate ncurses. */
    //endwin();
}
