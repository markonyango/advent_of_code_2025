#![allow(warnings)]

fn main() {
    let content = std::fs::read_to_string("./day3.txt").unwrap();
    let mut vec = vec![];

    for line in content.lines() {
        let mut bank = Bank::new(line);
        let joltage = bank.process();

        vec.push(joltage);
    }

    let sum = vec.iter().fold(0, |acc, curr| acc + curr);

    println!("Part1 - sum is {sum}");
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
            .max_by(
                |(index_a, value_a), (index_b, value_b)| match value_a.cmp(value_b) {
                    // We must keep the FIRST max index as the candidate - max_by always returns
                    // the LAST instance of the maximum value
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less, //(index_b, value_b),
                    std::cmp::Ordering::Equal => match index_a.cmp(index_b) {
                        // If the values are equal reverse ordering -> we always want the FIRST
                        // index of the maximum value
                        std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                    },
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater, //(index_a, value_a),
                },
            )
            .unwrap();

        (index, value)
    }

    fn process(&self) -> u32 {
        let mut vec = self.batteries.clone();

        let (index1, value1) = Bank::find_highest_joltage(&vec);

        let string = if index1 == 0 {
            // normal case -> highest int was at the beginning
            let (index2, value2) = Bank::find_highest_joltage(&vec[index1 + 1..]);

            format!("{}{}", value1, value2)
        } else if index1 == vec.len() - 1 {
            // special case -> highest int was at the end -> we need to switch 2nd int to the first position
            let (index2, value2) = Bank::find_highest_joltage(&vec[..index1]);

            format!("{}{}", value2, value1)
        } else {
            // normal case -> need to only search right from first index
            let (index2, value2) = Bank::find_highest_joltage(&vec[index1 + 1..]);

            format!("{}{}", value1, value2)
        };

        let res = string.parse::<u32>().unwrap();

        res
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
}
