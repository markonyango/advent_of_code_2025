use std::fmt::Display;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let content = std::fs::read_to_string("day1.txt")?;
    let lines = content.lines();
    let directions = lines.map(|l| parse(l)).collect::<Vec<_>>();

    let (final_position, exactly_zero, hit_and_passed_zero) = process(&directions, 50);

    println!("Final dial position is {}", final_position);
    println!("Dial was at 0 {} times (final position)", exactly_zero);
    println!("0 was crossed {} times", hit_and_passed_zero);
    assert_eq!(exactly_zero, 1180);

    Ok(())
}

pub fn parse(line: &str) -> Direction {
    let direction = &line[..1];
    let dial = line[1..]
        .parse::<i32>()
        .expect(format!("line: {}", line).as_str());

    match direction {
        "L" => Direction::Left(dial),
        "R" => Direction::Right(dial),
        _ => panic!("Unknown direction"),
    }
}

fn calculate_rotations(dial: i32, direction: &Direction) -> f32 {
    match direction {
        Direction::Left(v) => (dial - v) as f32 / 100f32,
        Direction::Right(v) => (dial + v) as f32 / 100f32,
    }
}

#[tracing::instrument]
fn spin(dial: i32, rotations: i32) -> (i32, i32) {
    let dial_long = dial + rotations;
    let mut revolutions = (dial_long / 100).abs();

    if dial != 0 && dial_long <= 0 {
        revolutions += 1;
    }

    (dial_long.rem_euclid(100), revolutions)
}

#[tracing::instrument(skip(dirs, dial))]
pub fn process(dirs: &[Direction], dial: i32) -> (i32, i32, i32) {
    let mut dial = dial;
    let mut counter = 0;
    let mut counter2 = 0;

    for direction in dirs {
        let rotations = calculate_rotations(dial, direction);

        let c = spin(dial, direction.into());

        counter2 += c.1;
        dial = ((rotations.fract() * 100f32).round() as i32).rem_euclid(100);

        if dial == 0 {
            counter += 1;
        }
    }

    (dial, counter, counter2)
}

#[derive(Debug)]
pub enum Direction {
    Left(i32),
    Right(i32),
}

impl From<&Direction> for i32 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Left(v) => -v,
            Direction::Right(v) => 1 * v,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left(v) => write!(f, "L({})", v),
            Direction::Right(v) => write!(f, "R({})", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn can_solve_part1() {
        let lines = INPUT.lines();

        let dial = 50i32;
        let dirs = lines.map(|l| parse(l)).collect::<Vec<_>>();
        let res = process(&dirs, dial);

        assert_eq!(res.0, 32);
        assert_eq!(res.1, 3);
    }

    #[test]
    fn can_calculate_rotations() {
        let start = 50;
        let direction = Direction::Left(1000);

        let rotations = calculate_rotations(start, &direction);
        assert_eq!(rotations, -9.5);

        let direction = Direction::Right(1000);
        let rotations = calculate_rotations(start, &direction);
        assert_eq!(rotations, 10.5);

        assert_eq!(rotations.fract(), 0.5);

        let direction = Direction::Left(68);
        let rotations = calculate_rotations(start, &direction);
        assert_eq!(rotations.trunc(), -0.0);
        assert_eq!(rotations.fract() * 100f32, -18.0);
        assert_eq!((-18i32).rem_euclid(100), 82);
    }

    #[test]
    fn can_solve_part2() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
        let lines = INPUT.lines();

        let dial = 50;
        let dirs = lines.map(|l| parse(l)).collect::<Vec<_>>();
        let res = process(&dirs, dial);

        assert_eq!(res.2, 6);
    }
}

//
// Circle has 360°
// 360° / 100 -> Lock has 3.6° for each dial position
// Starting position is the position 50 -> find angular representation =? 50 * 3.6 = 180° (duh)
// Figure out math for rotations
//
// Rotation 1000 -> 1000 x 3.6 = 3600°
// Full Circle 360°
// => 3600 / 360 = 10 rotations
//
// Example AoC (starting position 50 -> 180°):
//
// Position 50 -> 180°
// left 68 -> (68 x 3.6)° -> 244.8°
// => 180° - 244.8° = -64.8°
// => -64.8° / 360° => -0.18
// => 64.8° => 64.8°/ 3.6° = 18
//
// Position 50 -> 180°
// right 1000 -> (1000 x 3.6)° -> 3600°
// => 180° + 3600° = 3780°
// => 3780° / 360° => 10.5
// => 10 rotations
// => remainder is final position
//
