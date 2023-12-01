fn main() {
    let input = include_str!("part1.txt");
    let answer = part1(input);
    println!("{answer}");
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let numbers: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();
        let n1 = numbers.first().unwrap();
        let n2 = numbers.last().unwrap();

        n1.to_digit(10).unwrap() * 10 + n2.to_digit(10).unwrap()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        assert_eq!(142, part1(INPUT));
    }
}