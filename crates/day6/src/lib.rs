#[derive(Debug, Clone)]
enum MathOperand {
    Multiply,
    Add,
}

#[derive(Debug)]
struct MathOp {
    factors: Vec<i64>,
    operand: MathOperand,
}

fn transpose(matrix: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut transposed = vec![vec![0; matrix.len()]; matrix[0].len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }
    transposed
}

fn get_operands(input: &String) -> Vec<MathOperand> {
    input
        .split_ascii_whitespace()
        .map(|c| match c {
            "*" => MathOperand::Multiply,
            "+" => MathOperand::Add,
            x => panic!("Unknown operand {} specified", x),
        })
        .collect()
}

fn parse(input: &Vec<String>) -> Vec<MathOp> {
    let mut ops = vec![];

    let operands: Vec<MathOperand> = get_operands(input.last().unwrap());

    let matrix = transpose(
        &input[..input.len() - 1]
            .iter()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect()
            })
            .collect(),
    );

    let rows = matrix.len();

    for r in 0..rows {
        ops.push(MathOp {
            operand: operands[r].clone(),
            factors: matrix[r].clone(),
        });
    }

    ops
}

fn parse_vertically(input: &Vec<String>) -> Vec<MathOp> {
    let mut ops = vec![];

    let operands: Vec<MathOperand> = get_operands(input.last().unwrap());

    let mut index = 0;
    let mut grouping = 0;
    let mut factors = vec![];

    let cols = *vec![input[0].len(), input[1].len(), input[2].len()]
        .iter()
        .max()
        .unwrap();

    while index < cols {
        let mut factor = vec![];
        
        for r in 0..input.len() - 1 {
            let chars = input[r].chars().collect::<Vec<char>>();

            factor.push(match index < chars.len()   {
                true => chars[index],
                _ => ' ',
            });
        }
        index += 1;
        if factor.iter().all(|c| c.is_whitespace()) {
            // group boundary found
            ops.push(MathOp {
                factors: factors.clone(),
                operand: operands[grouping].clone(),
            });
            grouping += 1;
            factors.clear();
        } else {
            println!("Captured {:?}", factor);
            factors.push(
                factor
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .trim()
                    .parse::<i64>()
                    .unwrap(),
            );
        }
        factor.clear();
    }
    ops.push(MathOp {
        factors: factors.clone(),
        operand: operands[grouping].clone(),
    });
    ops
}

fn calculate_worksheet(worksheet: &Vec<MathOp>) -> i64 {
    worksheet
        .iter()
        .map(|m| {
            m.factors.iter().fold(
                match m.operand {
                    MathOperand::Add => 0,
                    MathOperand::Multiply => 1,
                },
                |acc, factor| match m.operand {
                    MathOperand::Add => acc + factor,
                    MathOperand::Multiply => acc * factor,
                },
            )
        })
        .sum()
}

pub fn part1(input: &Vec<String>) -> i64 {
    let worksheet = parse(input);
    calculate_worksheet(&worksheet)
}

pub fn part2(input: &Vec<String>) -> i64 {
    let worksheet = parse_vertically(input);
    println!("worksheet:\n{:?}", worksheet);
    calculate_worksheet(&worksheet)
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
        assert_eq!(result, 4277556);
    }

    #[test]
    fn sample_part2_valid() {
        let sample = get_sample();
        let result = part2(&sample);
        assert_eq!(result, 3263827);
    }
}
