use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

fn main() {
    println!("Part 1: {}", solve(include_str!("day9.input"), 1));
    println!("Part 2: {}", solve(include_str!("day9.input"), 9));
}

fn solve(s: &str, tail_size: usize) -> usize {
    let inputs = parse(s);
    let mut head = (0, 0);
    let mut tails = vec![(0, 0); tail_size];

    let set = inputs.iter().fold(HashSet::new(), |mut set, dir| {
        let heads = match dir {
            Dir::Up(count) => (1..=*count)
                .map(|i| (head.0, head.1 + i))
                .collect::<Vec<_>>(),
            Dir::Down(count) => (1..=*count)
                .map(|i| (head.0, head.1 - i))
                .collect::<Vec<_>>(),
            Dir::Left(count) => (1..=*count)
                .map(|i| (head.0 - i, head.1))
                .collect::<Vec<_>>(),
            Dir::Right(count) => (1..=*count)
                .map(|i| (head.0 + i, head.1))
                .collect::<Vec<_>>(),
        };

        heads.iter().for_each(|h| {
            head = *h;
            let current_tail = tails.iter_mut().fold(head, |head, tail| {
                *tail = move_tail(&head, *tail);
                *tail
            });
            set.insert(current_tail);
        });

        set
    });

    set.len()
}

fn move_tail(head: &(isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let diff_x = head.0 - tail.0;
    let diff_y = head.1 - tail.1;

    match (diff_x, diff_y) {
        (x, 0) if x.abs() == 2 => (tail.0 + diff_x / 2, tail.1),
        (0, y) if y.abs() == 2 => (tail.0, tail.1 + diff_y / 2),
        (x, y) if x.abs() == 2 && y.abs() == 1 => (tail.0 + diff_x / 2, head.1),
        (x, y) if y.abs() == 2 && x.abs() == 1 => (head.0, tail.1 + diff_y / 2),
        (x, y) if x.abs() == 2 && y.abs() == 2 => (tail.0 + diff_x / 2, tail.1 + diff_y / 2),
        _ => tail,
    }
}

fn parse(s: &str) -> Vec<Dir> {
    use Dir::*;

    s.lines()
        .map(|s| match s.split_once(' ') {
            Some((dir, count)) if dir == "U" => Up(str::parse(count).unwrap()),
            Some((dir, count)) if dir == "D" => Down(str::parse(count).unwrap()),
            Some((dir, count)) if dir == "L" => Left(str::parse(count).unwrap()),
            Some((dir, count)) if dir == "R" => Right(str::parse(count).unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_1() {
        assert_eq!(13, solve(include_str!("day9.test"), 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(36, solve(include_str!("day9_2.test"), 9));
    }
}
