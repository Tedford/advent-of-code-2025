fn parse(input: &Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn find_largest_2_joltage(batteries: &Vec<i32>) -> i64 {
    let mut fst = batteries[0];
    let mut index = 0;

    for i in 1..batteries.len() - 1 {
        if batteries[i] > fst {
            fst = batteries[i];
            index = i;
        }
        if fst == 9 {
            break;
        }
    }

    let mut snd = batteries[index + 1];

    for i in index + 2..batteries.len() {
        if batteries[i] > snd {
            snd = batteries[i];
        }
        if snd == 9 {
            break;
        }
    }
    format!("{}{}", fst, snd).parse::<i64>().unwrap()
}

fn find_largest_12_joltage(batteries: &Vec<i32>) -> i64 {
    let mut active = batteries[batteries.len() - 12..].to_vec();
    active[0] = batteries[0];
    let mut index = 0; // index of the last assigned battery
    let mut index2 = batteries.len() - active.len() + 1; // index of the first remainder

    for i in 1..index2 {
        if batteries[i] > active[0] {
            active[0] = batteries[i];
            index = i;
        }
        if active[0] == 9 {
            break;
        }
    }
    println!("{} {:?}", index, batteries);
    index += 1;

    // compact the number range
    for digit in 1..active.len() {
        let mut first_match = 0;
        let mut new_index = 0;

        for i in index..=index2 {
            if batteries[i] == active[digit] && first_match == 0 {
                first_match = i;
            }
            if batteries[i] > active[digit] {
                active[digit] = batteries[i];
                new_index = i;
            }
        }

        if new_index > 0 {
            index = new_index + 1;
            index2 += 1;
        } else if first_match > 0 {
            index = first_match + 1;
            index2 += 1;
        } else {
            break;
        }
    }

    let s = active
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", s);
    s.parse::<i64>().unwrap()
}

pub fn part1(input: &Vec<String>) -> i64 {
    let banks = parse(input);
    let joltages = banks
        .iter()
        .map(|x| find_largest_2_joltage(x))
        .collect::<Vec<i64>>();
    joltages.iter().sum()
}


// 173843907657110 too low
// 173960689460215 too high
pub fn part2(input: &Vec<String>) -> i64 {
    let banks = parse(input);
    let joltages = banks
        .iter()
        .map(|x| find_largest_12_joltage(x))
        .collect::<Vec<i64>>();
    println!("{:?}", joltages);
    joltages.iter().sum()
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
        assert_eq!(result, 357);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 3121910778619);
    }


    #[test]
    fn nine_heavy() {
        let sample = vec!["9847555865827676657688569759758758555445794968796565767687569857578579948785658658585576858447769967".to_string()];
        let result = part2(&sample);
        assert_eq!(result, 999999999997);
    }
    #[test]
    fn mixed_compaction() {
        let sample = vec!["4238131312372266238322323992851888128123122242282434742712328963325226342472212242762454422754113423".to_string()];
        let result = part2(&sample);
        assert_eq!(result, 999777543423);
    }

    #[test]
    fn full_last_12() {
        let sample =vec!["2222544122212134422245322622323232121113423222422311232232422322223221212233652414122759322232231122".to_string()];
        let result = part2(&sample);
        assert_eq!(result, 932232231122);
    }
}
