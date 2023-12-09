use day_9::*;
use itertools::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> isize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(|line| do_stuff(&line) + line.last().unwrap())
        .sum()
}

fn do_stuff(value: &Vec<isize>) -> isize {
    let sub = value
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if sub.iter().any(|x| *x != 0) {
        let val = do_stuff(&sub);
        return val + sub.last().unwrap();
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        assert_eq!(114, process(INPUT));
    }
}
