fn main() {
    let input = include_str!("../input2.txt");
    let answer = part2(input);
    println!("{answer}");
}

fn part2(input: &str) -> u32 {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let n1 = (0..line.len())
                .find_map(|i| {
                    let slice = line[i..(i + 5).min(line.len())].as_bytes();
                    let first = slice.first().unwrap();
                    if first.is_ascii_digit() {
                        Some(first - b'0')
                    } else {
                        DIGITS
                            .iter()
                            .position(|digit| slice.starts_with(digit.as_bytes()))
                            .map(|pos| pos as u8 + 1)
                    }
                })
                .unwrap();

            let n2 = (0..line.len())
                .find_map(|i| {
                    let slice = line
                        [((line.len() as isize - i as isize - 5).max(0) as usize)..line.len() - i]
                        .as_bytes();
                    let last = slice.last().unwrap();
                    if last.is_ascii_digit() {
                        Some(last - b'0')
                    } else {
                        DIGITS
                            .iter()
                            .position(|digit| slice.ends_with(digit.as_bytes()))
                            .map(|pos| pos as u8 + 1)
                    }
                })
                .unwrap();

            (n1 * 10 + n2) as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;

        assert_eq!(281, part2(INPUT));
    }
}
