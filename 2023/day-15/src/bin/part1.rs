use day_15::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .map(|x| {
            x.as_bytes()
                .into_iter()
                .map(|x| *x as usize)
                .fold(0, |acc, x| ((acc + x) * 17) % 256)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(1320, process(INPUT));
    }
}
