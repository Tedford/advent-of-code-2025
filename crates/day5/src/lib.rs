// use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Lot {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Inventory {
    fresh: Vec<Lot>,
    available: Vec<i64>,
}

fn parse(input: &Vec<String>) -> Inventory {
    let mut fresh = vec![];
    let mut available = vec![];

    let mut reading_fresh = true;

    for i in 0..input.len() {
        if reading_fresh && input[i] == "" {
            reading_fresh = false;
        } else if reading_fresh {
            if let Some((a, b)) = input[i].split_once('-') {
                let start = a.trim().parse::<i64>().expect("invalid start");
                let end = b.trim().parse::<i64>().expect("invalid end");
                fresh.push(Lot { start, end });
            } else {
                panic!("invalid range format");
            }
        } else {
            available.push(input[i].parse::<i64>().unwrap());
        }
    }

    Inventory { fresh, available }
}

pub fn part1(input: &Vec<String>) -> i64 {
    let inventory = parse(input);
    let mut fresh = 0i64;

    for ingredient in &inventory.available {
        for lot in &inventory.fresh {
            if lot.start <= *ingredient && lot.end >= *ingredient {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

fn condense_ranges(lots: &Vec<Lot>) -> (bool, Vec<Lot>) {
    let mut new_lots: Vec<Lot> = vec![];
    let mut modified = false;

    for lot in lots {
        let mut covered = false;

        for i in 0..new_lots.len() {
            let l2 = &new_lots[i];

            if l2.start <= lot.start && l2.end >= lot.end {
                // proper subset, already covered -- discard
                println!("{:?} is a subset of {:?}", lot, l2);
                covered = true;
                modified = true;
                break;
            } else if l2.start <= lot.start && l2.end > lot.start && l2.end <= lot.end {
                // extends the range to the right
                let l = Lot {
                    start: l2.start,
                    end: lot.end,
                };
                println!("Extending {:?} to the right with {:?} -> {:?}", l2, lot, l);
                modified = true;
                covered = true;
                new_lots[i] = l;
                break;
            } else if lot.start <= l2.start && l2.start <= lot.end && lot.end <= l2.end {
                // extends the range to the left
                let l = Lot {
                    start: lot.start,
                    end: l2.end,
                };
                println!("Extending {:?} to the left with {:?} -> {:?}", l2, lot, l);
                modified = true;
                covered = true;
                new_lots[i] = l;
                break;
            }
        }

        if !covered {
            new_lots.push(lot.clone());
        }
    }
    (modified, new_lots)
}

// 552728275049930 too high
// 355739580603763 too high
pub fn part2(input: &Vec<String>) -> i64 {
    let inventory = parse(input);

    let mut lots = inventory.fresh;
    let mut compacting = true;

    while compacting {
        let (modified, new_lots) = condense_ranges(&lots);
        lots = new_lots;
        compacting = modified;
    }

    println!("{:?}", lots);

    lots.iter().map(|lot| lot.end - lot.start + 1).sum::<i64>()
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
        assert_eq!(result, 14);
    }
}
