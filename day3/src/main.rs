#![allow(warnings)]

fn main() {
    let content = std::fs::read_to_string("./day3.txt").unwrap();
    let mut vec = vec![];
    let mut vec_p2 = vec![];

    for line in content.lines() {
        let mut bank = Bank::new(line);
        let joltage = bank.process();
        let joltage_p2 = bank.process_p2();

        vec.push(joltage);
        vec_p2.push(joltage_p2);
    }

    let sum = vec.iter().fold(0, |acc, curr| acc + curr);
    let sum_p2 = vec_p2.iter().sum::<u64>();

    println!("Part1 - sum is {sum}");
    println!("Part2 - sum is {sum_p2}");
}

struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    fn new(line: &str) -> Self {
        let batteries = line
            .chars()
            .collect::<Vec<_>>()
            .chunks(1)
            .flat_map(|c| c[0].to_digit(10))
            .collect::<Vec<_>>();

        Self { batteries }
    }

    fn find_highest_joltage(vec: &[u32]) -> (usize, &u32) {
        let (index, value) = vec
            .iter()
            .enumerate()
            .reduce(|acc, curr| if acc.1 >= curr.1 { acc } else { curr })
            .unwrap();

        (index, value)
    }

    fn process(&self) -> u32 {
        let mut vec = self.batteries.clone();

        // Only have to look up the second to last index thus making sure
        // the 2 digits are in the correct order (e.g. 81119 - only search 8111 and then 1119)
        let (index1, value1) = Bank::find_highest_joltage(&vec[..vec.len() - 1]);
        let (index2, value2) = Bank::find_highest_joltage(&vec[index1 + 1..]);

        let string = format!("{}{}", value1, value2);

        let res = string.parse::<u32>().unwrap();

        res
    }

    fn process_p2(&self) -> u64 {
        let mut vec = self.batteries.clone();

        let mut current_index = 0;
        let mut res = vec![];

        for i in 0..12 {
            let (index, value) =
                Bank::find_highest_joltage(&vec[current_index..(&self.batteries.len() - 11 + i)]);

            // trace::info!(index, value);
            current_index = current_index + index + 1;
            res.push(value);
        }

        res.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<u64>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    fn process(line: &str) -> u32 {
        let bank = Bank::new(line);
        let joltage = bank.process();

        joltage
    }

    #[test]
    fn line1() {
        let line = "987654321111111";
        assert_eq!(98, process(line));

        let line = "811111111111119";
        assert_eq!(89, process(line));

        let line = "234234234234278";
        assert_eq!(78, process(line));

        let line = "818181911112111";
        assert_eq!(92, process(line));

        let line = "7383383448669853327488444395431356632533467643668433447312354949444179338343352833433277429334563636";
        assert_eq!(99, process(line));
    }

    #[test]
    fn line2() {
        let line = "987654321111111";
        let bank = Bank::new(line);
        assert_eq!(987654321111, bank.process_p2());
    }
}
