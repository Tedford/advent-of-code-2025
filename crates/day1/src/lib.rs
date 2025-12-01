extern crate alloc;

#[derive(Debug)]
struct Turn {
    dir: Direction,
    click: i64,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &Vec<String>) -> Vec<Turn> {
    let mut turns: Vec<Turn> = Vec::new();

    input.iter().for_each(|v| {
        let dir = match v.chars().nth(0).unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            other => panic!("Unable to determine direction {}", other),
        };

        let click = v[1..].parse::<i64>().unwrap();

        turns.push(Turn { dir, click });
    });

    turns
}

fn turn_left(current: i64, amount: i64) -> i64 {
    (current - amount).rem_euclid(100)
}

fn turn_left_with_overflow(current: i64, amount: i64) -> (i64, i64) {
    let new = (current - amount).rem_euclid(100);
    let overflow = amount / 100
        + match new > current && new != 0 && current != 0{
            true => 1,
            _ => 0,
        };
    (new, overflow)
}

fn turn_right(current: i64, amount: i64) -> i64 {
    (current + amount) % 100
}

fn turn_right_with_overflow(current: i64, amount: i64) -> (i64, i64) {
    let new = (current + amount) % 100;
    let overflow = amount / 100
        + match new < current && new != 0 && current != 0{
            true => 1,
            _ => 0,
        };
    (new, overflow)
}

pub fn part1(input: &Vec<String>) -> i64 {
    let turns = parse(input);
    println!("starting at 50");
    let result = turns.iter().fold((50i64, 0i64), |acc, x| {
        let new = match x.dir {
            Direction::Left => turn_left(acc.0, x.click),
            Direction::Right => turn_right(acc.0, x.click),
        };
        let count = match new {
            0 => acc.1 + 1,
            _ => acc.1,
        };
        println!("turn {:?} {} clicks to {}", x.dir, x.click, new);
        (new, count)
    });

    result.1
}

pub fn part2(input: &Vec<String>) -> i64 {
    let turns = parse(input);
    println!("starting at 50");
    let result = turns.iter().fold((50i64, 0i64), |acc, x| {
        let new = match x.dir {
            Direction::Left => turn_left_with_overflow(acc.0, x.click),
            Direction::Right => turn_right_with_overflow(acc.0, x.click),
        };
        let count = match new.0 {
            0 => acc.1 + 1,
            _ => acc.1,
        } + new.1;
        println!("turn {:?} {} clicks to {} - overflow {}", x.dir, x.click, new.0, new.1);
        (new.0, count)
    });

    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        include_str!("sample.dat")
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    #[test]
    fn sample_part1_valid() {
        let sample = get_sample();
        let result = part1(&sample);
        assert_eq!(result, 3);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 6);
    }
}
