#![allow(warnings)]

use std::{
    collections::{BTreeSet, HashSet},
    io::Write,
    ops::{RangeInclusive, RangeToInclusive},
};
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
    tracing_subscriber::fmt().init();
    // let fresh = process(INPUT);
    let content = std::fs::read_to_string("./day5.txt").unwrap();
    let fresh = process(&content);

    tracing::info!("part1: {} are fresh", fresh);
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

    part2(&mut ranges);

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

fn part2(ranges: &mut Vec<RangeInclusive<usize>>) -> usize {
    let mut num = 0;

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut res = vec![];
    let mut current_range = ranges.first().unwrap().clone();

    for (i, r) in ranges[1..].iter().enumerate() {
        let s1 = current_range.start();
        let e1 = current_range.end();

        let s2 = r.start();
        let e2 = r.end();

        tracing::trace!("{:?} {:?}", (s1..e1), (s2..e2));

        match e1.cmp(s2) {
            std::cmp::Ordering::Less => {
                tracing::trace!("No overlap");
                res.push(current_range);
                current_range = r.clone();
            }
            std::cmp::Ordering::Equal => {
                tracing::trace!("Merging: s1..e2");
                current_range = RangeInclusive::new(*s1, *e2);
            }
            std::cmp::Ordering::Greater => match e1.cmp(e2) {
                std::cmp::Ordering::Less => {
                    tracing::trace!("Merging: s1..e2");
                    current_range = RangeInclusive::new(*s1, *e2);
                }
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                    tracing::trace!("Merging: s1..e1");
                    current_range = RangeInclusive::new(*s1, *e1);
                }
            },
        };

        if i == ranges.len() - 1 - 1 {
            res.push(current_range.clone());
        }

        tracing::trace!("{i}: current_range {:?} - res {:?}", current_range, res);
    }

    let mut num = 0;
    for r in res {
        num += ((r.end() - r.start()) + 1);
    }

    tracing::info!("Part2: {num}");

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve() {
        tracing_subscriber::fmt().init();
        let mut ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];

        assert_eq!(14, part2(&mut ranges));
    }
}
