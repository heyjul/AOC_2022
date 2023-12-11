use day_11::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let x = input.lines().count();
    let y = input.lines().next().unwrap().trim().len();

    let vec = parse_input(input);
    let empty_x: Vec<_> = (0..x)
        .filter(|x| !vec.iter().any(|(x1, _)| x == x1))
        .collect();
    let empty_y: Vec<_> = (0..y)
        .filter(|y| !vec.iter().any(|(_, y1)| y == y1))
        .collect();

    let mut sum = 0;
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if i == j {
                continue;
            }

            let (x1, y1) = vec[i];
            let (x2, y2) = vec[j];

            if x1 < x2 {
                sum += (x2 - x1) + empty_x.iter().filter(|&&x| x > x1 && x < x2).count();
            }

            if y1 < y2 {
                sum += (y2 - y1) + empty_y.iter().filter(|&&y| y > y1 && y < y2).count();
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        assert_eq!(374, process(INPUT));
    }
}
