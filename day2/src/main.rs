#![allow(unused)]

const INPUT: &str = "749639-858415,65630137-65704528,10662-29791,1-17,9897536-10087630,1239-2285,1380136-1595466,8238934-8372812,211440-256482,623-1205,102561-122442,91871983-91968838,62364163-62554867,3737324037-3737408513,9494926669-9494965937,9939271919-9939349036,83764103-83929201,24784655-24849904,166-605,991665-1015125,262373-399735,557161-618450,937905586-937994967,71647091-71771804,8882706-9059390,2546-10476,4955694516-4955781763,47437-99032,645402-707561,27-86,97-157,894084-989884,421072-462151
";

fn main() {
    let res = parse(INPUT).unwrap();
    let sum = res
        .into_iter()
        .reduce(|a, c| a + c)
        .expect("Could find sum");

    println!("Sum: {sum}");
}

fn get_range(input: &str, index: usize) -> (i64, i64) {
    let input = input.trim();
    let first = input[..index].parse::<i64>().expect(
        format!(
            "Could not parse {input} (index: {index} - {} - {}",
            &input[..index],
            &input[index + 1..]
        )
        .as_str(),
    );
    let last = input[index + 1..]
        .parse::<i64>()
        .expect(format!("Could not parse {input} (index: {index})").as_str());

    (first, last)
}

fn find_invalid_ids(range: (i64, i64)) -> Vec<i64> {
    let mut res = vec![];
    for id in range.0..=range.1 {
        let string = id.to_string();

        let length = string.len();

        if length % 2 != 0 {
            continue;
        }

        let first_half = &string[..length / 2];
        let second_half = &string[length / 2..];

        if first_half.eq(second_half) {
            res.push(id);
        }
    }

    res
}

fn parse(input: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let lines = input.split(",").collect::<Vec<_>>();
    let mut result = vec![];

    for line in lines {
        let index = match line.find("-") {
            Some(index) => index,
            None => return Err("delimiter not found".to_string().into()),
        };

        let range = get_range(line, index);

        let invalid_ids = find_invalid_ids(range);
        result.extend_from_slice(&invalid_ids);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn can_find_invalid_ids() {
        let res = find_invalid_ids((11, 22));

        assert_eq!(res, vec![11, 22]);
    }

    #[test]
    fn can_solve() {
        let invalid_ids = parse(INPUT);
        let sum = invalid_ids
            .expect("Could parse ids")
            .into_iter()
            .reduce(|a, c| a + c)
            .expect("Could find sum");

        assert_eq!(sum, 1227775554);
    }
}
