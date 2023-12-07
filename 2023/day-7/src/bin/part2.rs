use std::collections::HashMap;

use day_7::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> usize {
    const CARDS: [Card; 13] = [
        Card::J,
        Card::N(2),
        Card::N(3),
        Card::N(4),
        Card::N(5),
        Card::N(6),
        Card::N(7),
        Card::N(8),
        Card::N(9),
        Card::T,
        Card::Q,
        Card::K,
        Card::A,
    ];
    let mut values = parse_input(input);
    values.sort_unstable_by(|(a, _), (b, _)| {
        let mut a_map = a.iter().fold(HashMap::new(), |mut acc, x| {
            if *x == Card::J {
                return acc;
            }
            let entry = acc.entry(x).or_insert(0);
            *entry += 1;
            acc
        });
        let a_j = a.iter().filter(|x| **x == Card::J).count();
        if a_j > 0 {
            if let Some(entry) = a_map.iter_mut().max_by(|x, y| x.1.cmp(&y.1)) {
                *entry.1 += a_j;
            } else {
                a_map.insert(&Card::J, 5);
            }
        }

        let mut b_map = b.iter().fold(HashMap::new(), |mut acc, x| {
            if *x == Card::J {
                return acc;
            }
            let entry = acc.entry(x).or_insert(0);
            *entry += 1;
            acc
        });
        let b_j = b.iter().filter(|x| **x == Card::J).count();
        if b_j > 0 {
            if let Some(entry) = b_map.iter_mut().max_by(|x, y| x.1.cmp(&y.1)) {
                *entry.1 += b_j;
            } else {
                b_map.insert(&Card::J, 5);
            }
        }
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
    fn test_part2() {
        const INPUT: &str = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        assert_eq!(5905, process(INPUT));
    }
}
