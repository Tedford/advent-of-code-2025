use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    min: i64,
    max: i64,
}

fn parse(input: &Vec<String>) -> Vec<Range> {
    input
        .iter()
        .map(|x| x.split(","))
        .flatten()
        .map(|x| {
            let parts = x.split('-').collect::<Vec<_>>();
            let min = parts[0].parse::<i64>().unwrap();
            let max = parts[1].parse::<i64>().unwrap();
            Range { min, max }
        })
        .collect()
}

fn find_doubles(range: &Range) -> Vec<i64> {
    (range.min..=range.max)
        .filter(|&i| {
            let s = i.to_string();
            let len = s.len();
            if len % 2 != 0 {
                return false;
            }
            let mid = len / 2;
            let (left, right) = s.split_at(mid);
            left == right
        })
        .collect()
}

fn find_repeats(range: &Range) -> Vec<i64> {
    (range.min..=range.max)
        .filter(|&i| {
            let mut map = HashMap::new();
            let s = i.to_string();
            let length = s.len();
            s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
            if map.keys().count() == 1 && length > 1 {
                return true; // single digit pattern
            }
            let min = map.values().min().unwrap();
            let window = map.values().map(|v| (v / min) as usize).sum();
            if length / window < 2 || length % window != 0 {
                return false;
            }
            let pattern = s[0..window].to_string();
            let target = pattern.repeat(length / window);
            target == s // variable length pattern
        })
        .collect()
}

pub fn part1(input: &Vec<String>) -> i64 {
    let ranges = parse(input);
    let doubles = ranges
        .iter()
        .map(|r| find_doubles(&r))
        .flatten()
        .collect::<Vec<_>>();
    //println!("{:?}", doubles);
    doubles.iter().sum()
}
// 36862281460 too high
pub fn part2(input: &Vec<String>) -> i64 {
    let ranges = parse(input);
    let ids = ranges
        .iter()
        .map(|r| find_repeats(&r))
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", ids);
    ids.iter().sum()
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
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 4174379265);
    }
}
