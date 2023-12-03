use std::collections::HashMap;

use day_3::*;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = part2(input);
    println!("{answer}");
}

fn part2(input: &str) -> u32 {
    let width = input.lines().next().map(|l| l.len()).unwrap_or(0);
    let symbol_positions = get_symbol_positions(input);
    let numbers = get_numbers(input, width);

    numbers
        .into_iter()
        .fold(HashMap::new(), |mut acc, x| {
            let adgacents: Vec<usize> = symbol_positions
                .iter()
                .filter(|p| x.is_adjacent(**p, width))
                .map(|x| *x)
                .collect();

            for p in adgacents {
                let entry = acc.entry(p).or_insert(Vec::new());
                entry.push(x.value);
            }

            acc
        })
        .iter()
        .filter(|(_, x)| x.len() == 2)
        .map(|(_, x)| x.iter().product::<u32>())
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
        .filter(|(_, x)| *x == '*')
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
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

        assert_eq!(467835, part2(INPUT));
    }
}
