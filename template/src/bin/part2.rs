use {{crate_name}}::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> usize {
    let _ = parse_input(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "";

        assert_eq!(0, process(INPUT));
    }
}