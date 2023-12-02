use day_2::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = part1(input);
    println!("{answer}");
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Game::from)
        .filter_map(|game| {
            let impossible = game.subsets.into_iter().any(|x| match x {
                Subset {
                    color: Color::Red,
                    n,
                } if n > 12 => true,
                Subset {
                    color: Color::Green,
                    n,
                } if n > 13 => true,
                Subset {
                    color: Color::Blue,
                    n,
                } if n > 14 => true,
                _ => false,
            });

            if impossible {
                None
            } else {
                Some(game.game)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(8, part1(INPUT));
    }
}
