fn main() {
    let input = include_str!("../input1.txt");
    let answer = part1(input);
    println!("{answer}");
}

fn part1(input: &str) -> u32 {
    input
    .lines()
    .map(|x| x.split_once(':').unwrap().1)
    .map(|x| x.split_once('|').unwrap())
    .map(|(x, y)| {
        let win: Vec<_> = x.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect();
        let count = y.split(' ').filter_map(|x| x.parse::<u32>().ok()).filter(|x| win.contains(&x)).count();
        if count == 0 { 0 } else { 2_u32.pow(count as u32 - 1) } 
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, part1(INPUT));
    }
}
