use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("day8.input");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let map = map_input(input);

    let set = map.iter().fold(HashSet::new(), |mut set, (k, v)| {
        v.iter().enumerate().for_each(|(i, x)| {
            let coordinates = match k {
                (Some(x), None) => (*x, i),
                (None, Some(y)) => (i, *y),
                _ => unreachable!(),
            };

            if i == 0
                || i == v.len() - 1
                || x > v[0..i].iter().max().unwrap()
                || x > v[i + 1..v.len()].iter().max().unwrap()
            {
                set.insert(coordinates);
            }
        });

        set
    });

    set.len()
}

fn part_2(input: &str) -> usize {
    let map = map_input(input);

    let map = map.iter().fold(HashMap::new(), |mut map, (k, v)| {
        v.iter().enumerate().for_each(|(i, x)| {
            let coordinates = match k {
                (Some(x), None) => (*x, i),
                (None, Some(y)) => (i, *y),
                _ => unreachable!(),
            };

            let left = (v[0..i].iter().rev().take_while(|y| *y < x).count() + 1).min(i);
            let right =
                (v[i + 1..v.len()].iter().take_while(|y| *y < x).count() + 1).min(v.len() - i - 1);

            (*map.entry(coordinates).or_insert(1)) *= left * right;
        });

        map
    });

    *map.values().max().unwrap()
}

fn map_input(input: &str) -> HashMap<(Option<usize>, Option<usize>), Vec<u8>> {
    let map = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |map, (y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .fold(map, |mut map, (x, v)| {
                    (*map.entry((Some(x), None)).or_insert(Vec::new())).push(v - b'0');
                    (*map.entry((None, Some(y))).or_insert(Vec::new())).push(v - b'0');
                    map
                })
        });

    map
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_1() {
        assert_eq!(21, part_1(include_str!("day8.test")));
    }

    #[test]
    fn test_2() {
        assert_eq!(8, part_2(include_str!("day8.test")));
    }
}
