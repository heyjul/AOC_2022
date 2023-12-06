fn main() {
    let input = include_str!("input.txt");
    let answer = part1(input);
    println!("Part 1 : {answer}");
    let answer = part2(input);
    println!("Part 2 : {answer}");
}

fn part1(input: &str) -> usize {
    let (time, distance) = parse(input);
    time.into_iter()
        .zip(distance.into_iter())
        .map(|(t, d)| (1..t).filter(|t2| (t - t2) * t2 > d).count())
        .product()
}

fn part2(input: &str) -> usize {
    let (time, distance) = parse2(input);
    (1..time).filter(|t2| (time - t2) * t2 > distance).count()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut lines = input.lines();
    let mut get = |x| {
        lines
            .next()
            .unwrap()
            .trim()
            .trim_start_matches(x)
            .split_ascii_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect()
    };
    let time = get("Time:");
    let distance = get("Distance:");

    (time, distance)
}

fn parse2(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let mut get = |x| {
        lines
            .next()
            .unwrap()
            .trim()
            .trim_start_matches(x)
            .split_ascii_whitespace()
            .collect::<String>()
            .parse()
            .unwrap()
    };
    let time = get("Time:");
    let distance = get("Distance:");

    (time, distance)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(288, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(71503, part2(INPUT));
    }
}
