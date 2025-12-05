#[derive(Debug,Clone)]
struct Floor {
    occupied: bool,
}

fn parse(input: &Vec<String>) -> Vec<Vec<Floor>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Floor {
                        occupied: false,
                    },
                    '@' => Floor {
                        occupied: true,
                    },
                    x => panic!("Unexpected floor tile {} detected", x),
                })
                .collect()
        })
        .collect()
}

fn count_has_four_spaces(floor: &Vec<Vec<Floor>>) -> i64 {
    let cols = floor[0].len();
    let rows = floor.len();
    let mut accessible = 0i64;

    for r in 0..rows {
        for c in 0..cols {
            let mut occupied = 0;

            // nw
            if r > 0 && c > 0 && floor[r - 1][c - 1].occupied {
                occupied += 1;
            }
            // n
            if r > 0 && floor[r - 1][c].occupied {
                occupied += 1;
            }
            // ne
            if r > 0 && c + 1 < cols && floor[r - 1][c + 1].occupied {
                occupied += 1;
            }
            // w
            if c > 0 && floor[r][c - 1].occupied {
                occupied += 1;
            }
            // e
            if c + 1 < cols && floor[r][c + 1].occupied {
                occupied += 1;
            }
            // sw
            if r + 1 < rows && c > 0 && floor[r + 1][c - 1].occupied {
                occupied += 1;
            }
            // s
            if r + 1 < rows && floor[r + 1][c].occupied {
                occupied += 1;
            }
            // se
            if r + 1 < rows && c + 1 < cols && floor[r + 1][c + 1].occupied {
                occupied += 1;
            }

            if floor[r][c].occupied && occupied < 4 {
                println!("{},{} accessible", r, c);
                accessible += 1;
            }
        }
    }
    accessible
}

fn count_and_remove(floor: &Vec<Vec<Floor>>) -> (i64,Vec<Vec<Floor>>) {
    let cols = floor[0].len();
    let rows = floor.len();
    let mut accessible = 0i64;
    let mut result =floor.clone();

    for r in 0..rows {
        for c in 0..cols {
            let mut occupied = 0;

            // nw
            if r > 0 && c > 0 && floor[r - 1][c - 1].occupied {
                occupied += 1;
            }
            // n
            if r > 0 && floor[r - 1][c].occupied {
                occupied += 1;
            }
            // ne
            if r > 0 && c + 1 < cols && floor[r - 1][c + 1].occupied {
                occupied += 1;
            }
            // w
            if c > 0 && floor[r][c - 1].occupied {
                occupied += 1;
            }
            // e
            if c + 1 < cols && floor[r][c + 1].occupied {
                occupied += 1;
            }
            // sw
            if r + 1 < rows && c > 0 && floor[r + 1][c - 1].occupied {
                occupied += 1;
            }
            // s
            if r + 1 < rows && floor[r + 1][c].occupied {
                occupied += 1;
            }
            // se
            if r + 1 < rows && c + 1 < cols && floor[r + 1][c + 1].occupied {
                occupied += 1;
            }

            if floor[r][c].occupied && occupied < 4 {
                println!("{},{} accessible", r, c);
                accessible += 1;
                result[r][c].occupied = false;
            }
        }
    }
    (accessible, result)
}

pub fn part1(input: &Vec<String>) -> i64 {
    let floor = parse(input);
    count_has_four_spaces(&floor)
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut floor = parse(input);
    let mut total_removed = 0i64;
    let mut cleaning = true;

    while cleaning {
        let (removed, new_floor) = count_and_remove(&floor);
        total_removed += removed;
        floor = new_floor;
        if removed == 0 {
            cleaning = false;
        }
    }

    total_removed
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
        assert_eq!(result, 13);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 43);
    }
}
