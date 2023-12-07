pub fn parse_input(input: &str) -> Vec<(Vec<Card>, usize)> {
    input
        .lines()
        .map(|x| {
            let (cards, value) = x.trim().split_once(' ').unwrap();
            (
                cards.chars().map(Card::from).collect(),
                value.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum Card {
    // J of part 2
    J,
    N(usize),
    T,
    // J of part 1
    // J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            x => Self::N(x.to_digit(10).unwrap() as usize),
        }
    }
}
