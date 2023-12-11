use day_11::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input, 1_000_000);
    println!("Part 2 : {answer}");
}

fn process(input: &str, ratio: usize) -> usize {
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
                sum +=
                    (x2 - x1) + empty_x.iter().filter(|&&x| x > x1 && x < x2).count() * (ratio - 1);
            }

            if y1 < y2 {
                sum +=
                    (y2 - y1) + empty_y.iter().filter(|&&y| y > y1 && y < y2).count() * (ratio - 1);
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
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

        assert_eq!(1030, process(INPUT, 10));
    }
}
