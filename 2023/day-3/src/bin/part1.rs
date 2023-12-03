use day_3::*;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = part1(input);
    println!("{answer}");
}

fn part1(input: &str) -> u32 {
    let width = input.lines().next().map(|l| l.len()).unwrap_or(0);
    // dbg!(width);
    let symbol_positions = get_symbol_positions(input);
    // dbg!(&symbol_positions);
    let numbers = get_numbers(input, width);

    dbg!(&numbers);

    numbers
        .into_iter()
        .filter_map(|x| {
            if symbol_positions.iter().any(|p| x.is_adjacent(*p, width)) {
                dbg!("match", &x.value);
                Some(x.value)
            } else {
                None
            }
        })
        .sum()
}

fn get_numbers(input: &str, w: usize) -> Vec<Number> {
    input
        .lines()
        .map(str::trim)
        .enumerate()
        .map(|(i, x)| {
            x.chars()
                .enumerate()
                .group_by(|(_, x)| x.is_ascii_digit())
                .into_iter()
                .filter(|(x, _)| *x)
                .map(|(_, x)| {
                    let mut value = String::new();
                    let mut start = usize::MAX;
                    let mut stop = 0;

                    for (j, x) in x {
                        value.push(x);
                        start = start.min(i * w + j);
                        stop = stop.max(i * w + j);
                    }

                    Number {
                        value: value.parse().unwrap(),
                        start,
                        stop,
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn get_symbol_positions(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .enumerate()
        .filter(|(_, x)| *x != '.' && !x.is_ascii_digit())
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;

        assert_eq!(4361, part1(INPUT));
    }
}
