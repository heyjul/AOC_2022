use std::collections::HashMap;

use day_7::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let mut values = parse_input(input);
    values.sort_unstable_by(|(a, _), (b, _)| {
        let a_map = a.iter().fold(HashMap::new(), |mut acc, x| {
            let entry = acc.entry(x).or_insert(0);
            *entry += 1;
            acc
        });

        let b_map = b.iter().fold(HashMap::new(), |mut acc, x| {
            let entry = acc.entry(x).or_insert(0);
            *entry += 1;
            acc
        });

        (
            a_map.iter().any(|(_, &x)| x == 5),
            a_map.iter().any(|(_, &x)| x == 4),
            a_map.iter().any(|(_, &x)| x == 3) && a_map.iter().any(|(_, &x)| x == 2),
            a_map.iter().any(|(_, &x)| x == 3),
            a_map.iter().filter(|(_, &x)| x == 2).count() == 2,
            a_map.iter().filter(|(_, &x)| x == 2).count() == 1,
            a,
        )
            .partial_cmp(&(
                b_map.iter().any(|(_, &x)| x == 5),
                b_map.iter().any(|(_, &x)| x == 4),
                b_map.iter().any(|(_, &x)| x == 3) && b_map.iter().any(|(_, &x)| x == 2),
                b_map.iter().any(|(_, &x)| x == 3),
                b_map.iter().filter(|(_, &x)| x == 2).count() == 2,
                b_map.iter().filter(|(_, &x)| x == 2).count() == 1,
                b,
            ))
            .unwrap()
    });
    values
        .into_iter()
        .map(|x| x.1)
        .enumerate()
        .map(|(i, x)| (i + 1) * x)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        assert_eq!(6440, process(INPUT));
    }
}
