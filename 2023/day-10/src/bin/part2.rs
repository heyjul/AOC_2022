use std::collections::HashSet;

use day_10::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> usize {
    let symbols = input
        .clone()
        .replace('S', "J") // cheating
        .lines()
        .map(str::trim)
        .flat_map(str::chars)
        .collect::<Vec<char>>();

    let len = input.lines().next().unwrap().len();
    let start = input
        .lines()
        .map(str::trim)
        .flat_map(str::as_bytes)
        .position(|x| *x == b'S')
        .unwrap();
    let input = parse_input(input);
    let (mut x, mut y) = (start % len, start / len);
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
            break;
        }

        visited.insert((y, x));
    }

    // https://www.reddit.com/r/adventofcode/comments/18evyu9/comment/kcqycfn/?utm_source=share&utm_medium=web2x&context=3
    let mut count = 0;
    for y in 0..len {
        for x in 0..len {
            if !visited.contains(&(y, x)) {
                if (x + 1..len)
                    .filter(|x| {
                        visited.contains(&(y, *x))
                            && ['|', 'L', 'J'].contains(&symbols.get(y * len + x).unwrap())
                    })
                    .count()
                    % 2
                    == 1
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(10, process(INPUT));
    }
}
