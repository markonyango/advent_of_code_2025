#![allow(warnings)]

use std::ops::Sub;
const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

const EXPECTED: &str = "..xx.xx@x.x@@.@.@.@@@@@@@.x.@@@.@@@@..@.x@.@@@@.@x.@@@@@@@.@.@.@.@.@@@x.@@@.@@@@.@@@@@@@@.x.x.@@@.x.";

fn main() {
    tracing_subscriber::fmt().init();
    let input = std::fs::read_to_string("./day4.txt").unwrap();
    let mut wall = Wall::new(input.as_str());
    let res = wall.process();

    println!("{} can be taken down.", res);
}

#[derive(Debug)]
struct Wall {
    lines: Vec<char>,
    width: usize,
}

impl Wall {
    fn new(content: &str) -> Self {
        let width = (content.find("\n").unwrap());
        let lines = content.replace("\n", "").chars().collect::<Vec<_>>();
        Self { lines, width }
    }

    fn process(&mut self) -> isize {
        let mut counter = 0;
        for i in 0..self.lines.len() {
            if self.lines[i] == '.' {
                continue;
            }

            let mut res = 0;
            let adjacent = get_adjacent(i, self.width);

            for index in adjacent {
                if let Some(x) = self.lines.get(index)
                    && (*x == '@' || *x == 'x')
                {
                    res += 1;
                }
            }

            if (res < 4) {
                counter += 1;
                self.lines[i] = 'x';
            }
        }

        counter
    }
}

fn get_adjacent(i: usize, width: usize) -> Vec<usize> {
    let mut res = vec![];

    let row = (i / width) + 1;
    let left_edge = i % width == 0;
    // we must prevent the first index to be recognized as a right edge (0 % x => 0)
    let right_edge = if i > 0 {
        i % (row * width - 1) == 0
    } else {
        false
    };
    let top = i <= width;

    if !left_edge {
        //left
        if let Some(x) = i.checked_sub(1) {
            res.push(x);
        }

        // left up
        if let Some(x) = i.checked_sub(width) {
            res.push(x - 1);
        }

        //left down
        res.push(i + width - 1);
    }

    if !right_edge {
        //right
        res.push(i + 1);

        //right up
        if let Some(x) = i.checked_sub(width) {
            res.push(x + 1);
        }

        // right down
        res.push(i + width + 1);
    }

    if !top {
        //up
        if let Some(x) = i.checked_sub(width) {
            res.push(x);
        }
    }

    //down
    res.push(i + width);

    res
}
