#![allow(warnings)]

use std::ops::RangeInclusive;
const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn main() {
    // let fresh = process(INPUT);
    let content = std::fs::read_to_string("./day5.txt").unwrap();
    let fresh = process(&content);

    println!("{} are fresh", fresh);

    // assert_eq!(fresh, 3);
}

fn process(content: &str) -> usize {
    let mut ranges = vec![];

    let mut current_index = 0;
    for line in content[current_index..].lines() {
        current_index = current_index + line.len() + 1;

        if line.len() == 0 {
            break;
        }

        let sep_index = line.find("-").unwrap();
        let start = line[..sep_index].parse::<usize>().unwrap();
        let end = line[sep_index + 1..].parse::<usize>().unwrap();

        ranges.push(start..=end);
    }

    let mut how_many_are_fresh = 0;
    for line in content[current_index..].lines() {
        let id = line.parse::<usize>().unwrap();
        if is_fresh(&id, &ranges) {
            how_many_are_fresh += 1;
        }
    }

    how_many_are_fresh
}

fn is_fresh(id: &usize, ranges: &Vec<RangeInclusive<usize>>) -> bool {
    ranges.iter().any(|r| r.contains(id))
}
