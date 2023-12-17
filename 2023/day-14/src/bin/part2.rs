use std::collections::HashMap;

use day_14::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> usize {
    let len_y = input.lines().count();
    let len_x = input.lines().next().unwrap().trim().len();

    let mut shapes = parse_input(input);
    let mut memory = HashMap::new();

    let mut i = 0;
    while i < 1000000000 {
        if let Some(x) = memory.get(&shapes) {
            i = 1000000000 - ((1000000000 - x) % (i - x));
        } else {
            memory.insert(shapes.clone(), i);
        }

        shapes = rotate(&mut shapes, Dir::North, len_x, len_y);
        shapes = rotate(&mut shapes, Dir::West, len_x, len_y);
        shapes = rotate(&mut shapes, Dir::South, len_x, len_y);
        shapes = rotate(&mut shapes, Dir::East, len_x, len_y);

        i += 1;
    }

    shapes
        .iter()
        .fold(HashMap::new(), |mut acc, x| {
            if x.shape == Shapes::Cube {
                return acc;
            }
            let entry = acc.entry(x.y).or_insert(0);
            *entry += 1;
            acc
        })
        .iter()
        .map(|(y, nbr)| nbr * (len_y - *y))
        .sum()
}

fn rotate(shapes: &mut Vec<Shape>, dir: Dir, len_x: usize, len_y: usize) -> Vec<Shape> {
    let mut new = Vec::new();

    match dir {
        Dir::North | Dir::West => {
            shapes.sort_unstable_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
        }
        _ => {
            shapes.sort_unstable_by(|a, b| (b.x, b.y).cmp(&(a.x, a.y)));
        }
    }

    for shape in shapes {
        if shape.shape == Shapes::Cube {
            new.push(*shape);
            continue;
        }

        let n = new
            .iter()
            .rev()
            .find(|x| match dir {
                Dir::North | Dir::South => x.x == shape.x,
                _ => x.y == shape.y,
            })
            .map(|x| match dir {
                Dir::North => x.y + 1,
                Dir::South => x.y.checked_sub(1).unwrap_or(0),
                Dir::West => x.x + 1,
                Dir::East => x.x.checked_sub(1).unwrap_or(0),
            })
            .unwrap_or_else(|| match dir {
                Dir::North | Dir::West => 0,
                Dir::South => len_y - 1,
                Dir::East => len_x - 1,
            });

        new.push(match dir {
            Dir::North | Dir::South => Shape { y: n, ..*shape },
            _ => Shape { x: n, ..*shape },
        });
    }

    new
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
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

        assert_eq!(64, process(INPUT));
    }
}
