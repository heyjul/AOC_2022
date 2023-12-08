use day_8::*;
use itertools::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> usize {
    let (dirs, map) = parse_input(input);
    let mut start: Vec<_> = map.keys().filter(|x| x.ends_with('A')).collect();
    let mut i = 0;
    let mut aaa = Vec::new();
    for _ in 0..usize::MAX {
        for d in dirs.chars() {
            start = start
                .iter()
                .map(|x| match d {
                    'R' => &map[*x].1,
                    'L' => &map[*x].0,
                    _ => unreachable!(),
                })
                .collect();
            i += 1;
            let c = start.clone();
            let ends_woth_z = c
                .iter()
                .enumerate()
                .filter(|(_, x)| x.ends_with('Z'))
                .collect_vec();
            for (ii, _) in ends_woth_z {
                aaa.push(i);
                start.remove(ii);
            }
            if start.is_empty() {
                let mut x = aaa.iter().tuple_windows::<(_, _)>();
                let first = x.next().unwrap();
                return x.fold(lcm(*first.0, *first.1), |acc, (_, b)| lcm(acc, *b));
            }
        }
    }

    unreachable!()
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        assert_eq!(6, process(INPUT));
    }
}
