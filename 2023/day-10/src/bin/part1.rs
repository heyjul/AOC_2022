use std::collections::HashSet;

use day_10::*;

fn main() {
    let input = include_str!("../input1.txt");
    let answer = process(input);
    println!("Part 1 : {answer}");
}

fn process(input: &str) -> usize {
    let len = input.lines().next().unwrap().len();
    let start = input
        .lines()
        .map(str::trim)
        .flat_map(str::as_bytes)
        .position(|x| *x == b'S')
        .unwrap();
    let input = parse_input(input);
    let (mut x, mut y) = (start % len, start / len);
    let mut dist = 0;
    let mut visited = HashSet::new();

    loop {
        let curr = input[y][x];

        // If up accepts bottom
        if y.checked_sub(1).is_some()
            && curr[0]
            && input[y - 1][x][2]
            && !visited.contains(&(y - 1, x))
        {
            y -= 1;
        }
        // If right accepts left
        else if x + 1 < len && curr[1] && input[y][x + 1][3] && !visited.contains(&(y, x + 1)) {
            x += 1;
        }
        // If bottom accepts top
        else if y + 1 < len && curr[2] && input[y + 1][x][0] && !visited.contains(&(y + 1, x)) {
            y += 1;
        }
        // If left accepts right
        else if x.checked_sub(1).is_some()
            && curr[3]
            && input[y][x - 1][1]
            && !visited.contains(&(y, x - 1))
        {
            x -= 1;
        } else {
            return dist / 2;
        }

        dist += 1;
        visited.insert((y, x));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";

        assert_eq!(8, process(INPUT));
    }
}
