use day_9::*;
use itertools::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> isize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(|line| line.first().unwrap() - do_stuff(&line))
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
        return sub.first().unwrap() - val;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        assert_eq!(2, process(INPUT));
    }
}
