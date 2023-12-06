use {{crate_name}}::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let _ = parse_input(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "";

        assert_eq!(0, process(INPUT));
    }
}