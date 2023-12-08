use day_8::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let (dirs, map) = parse_input(input);
    dbg!(&dirs, &map);
    let dirs: Vec<_> = dirs.chars().collect();
    let mut i = 0;
    let mut pos = "AAA";
    loop {
        match dirs.get(i % dirs.len()) {
            Some(d) if *d == 'R' => {
                pos = &map[pos].1;
            }
            Some(d) if *d == 'L' => {
                pos = &map[pos].0;
            }
            _ => unreachable!(),
        }
        i += 1;
        if pos == "ZZZ" {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_1() {
        const INPUT: &str = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, process(INPUT));
    }

    #[test]
    fn test_part1_2() {
        const INPUT: &str = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, process(INPUT));
    }
}
