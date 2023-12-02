#[derive(Debug)]
pub struct Game {
    pub game: usize,
    pub subsets: Vec<Subset>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, subsets) = value.split_once(':').unwrap();
        let game = game
            .trim()
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let subsets = subsets
            .split(';')
            .flat_map(|x| x.split(','))
            .map(Subset::from)
            .collect();
        Game { game, subsets }
    }
}

#[derive(Debug)]
pub struct Subset {
    pub n: usize,
    pub color: Color,
}

impl From<&str> for Subset {
    fn from(value: &str) -> Self {
        let (n, color) = value.trim().split_once(' ').unwrap();
        Self {
            n: n.trim().parse().unwrap(),
            color: color.trim().into(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            _ => unreachable!(),
        }
    }
}
