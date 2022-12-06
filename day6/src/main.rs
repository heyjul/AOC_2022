use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    println!("{}", part_1(include_str!("day6.input")));
    println!("{}", part_2(include_str!("day6.input")));
}

fn part_1(s: &str) -> usize {
    s.chars()
        .tuple_windows()
        .enumerate()
        .find(|(_, (a, b, c, d))| [a, b, c, d].iter().collect::<HashSet<_>>().iter().count() == 4)
        .unwrap()
        .0
        + 4
}

fn part_2(s: &str) -> usize {
    let s = s.as_bytes();
    let mut map = s.iter().take(14).fold(HashMap::new(), |mut map, c| {
        (*map.entry(c).or_insert(0)) += 1;
        map
    });

    if map.values().filter(|v| **v == 1).count() == 14 {
        return 14;
    }

    for i in 0..s.len() - 14 {
        (*map.get_mut(&s[i]).unwrap()) -= 1;
        (*map.entry(&s[i + 14]).or_insert(0)) += 1;

        if map.values().filter(|v| **v == 1).count() == 14 {
            return i + 15;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        assert_eq!(7, crate::part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, crate::part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, crate::part_1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, crate::part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, crate::part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part_2() {
        assert_eq!(14, crate::part_2("abcdefghijklmn"));
        assert_eq!(19, crate::part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, crate::part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, crate::part_2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, crate::part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, crate::part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
