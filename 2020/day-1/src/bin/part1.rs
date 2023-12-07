use std::collections::HashSet;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let mut set = HashSet::new();
    for x in input.lines().filter_map(|x| x.trim().parse::<usize>().ok()) {
        if set.get(&x).is_some() {
            return x * (2020 - x);
        }

        set.insert(2020 - x);
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "1721
        979
        366
        299
        675
        1456";

        assert_eq!(514579, process(INPUT));
    }
}
