use std::collections::HashMap;

use day_14::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let len = input.lines().count();
    let shapes = parse_input(input);
    let mut new = Vec::new();

    for shape in shapes {
        if shape.shape == Shapes::Cube {
            new.push(shape);
            continue;
        }

        let y = new
            .iter()
            .rev()
            .find(|x| x.x == shape.x)
            .map(|x| x.y + 1)
            .unwrap_or(0);

        new.push(Shape { y, ..shape });
    }

    new.iter()
        .fold(HashMap::new(), |mut acc, x| {
            if x.shape == Shapes::Cube {
                return acc;
            }
            let entry = acc.entry(x.y).or_insert(0);
            *entry += 1;
            acc
        })
        .iter()
        .map(|(y, nbr)| nbr * (len - *y))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";

        assert_eq!(136, process(INPUT));
    }
}
