use day_2::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = part2(input);
    println!("{answer}");
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .map(|game| {
            let max = move |color: Color| -> usize {
                game.subsets
                    .iter()
                    .filter(|x| x.color == color)
                    .map(|x| x.n)
                    .max()
                    .unwrap()
            };

            let r: usize = max(Color::Red);
            let g: usize = max(Color::Green);
            let b: usize = max(Color::Blue);

            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(2286, part2(INPUT));
    }
}
