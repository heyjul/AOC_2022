use std::collections::HashMap;

use day_4::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = part2(input);
    println!("{answer}");
}

fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "";

        assert_eq!(467835, part2(INPUT));
    }
}
