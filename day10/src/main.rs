fn main() {
    let input = map_input(include_str!("day10.input"));

    println!("Part 1 : {}", part_1(&input));
    part_2(&input);
}

fn part_1(vec: &[(isize, isize)]) -> isize {
    vec.iter()
        .filter(|(cycle, _)| (cycle + 20) % 40 == 0)
        .map(|(cycle, strength)| cycle * strength)
        .sum()
}

fn part_2(vec: &[(isize, isize)]) {
    vec.iter().skip(1).fold(0, |sprite, (cycle, strength)| {
        if (sprite..sprite + 3).contains(&((*cycle - 1) % 40)) {
            print!("#");
        } else {
            print!(".");
        }

        if (cycle - 1) % 40 == 0 {
            println!();
        }

        *strength
    });
}

fn map_input(input: &str) -> Vec<(isize, isize)> {
    let mut vec = Vec::new();

    input
        .lines()
        .flat_map(|l| l.split_whitespace().map(|x| str::parse::<isize>(x)))
        .enumerate()
        .fold(1, |strength, (cycle, value)| {
            vec.push((cycle as isize + 1, strength));

            match value {
                Err(_) => strength,
                Ok(value) => strength + value,
            }
        });

    vec
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_1() {
        assert_eq!(13140, part_1(&map_input(include_str!("day10.test"))));
    }

    #[test]
    fn test_2() {
        part_2(&map_input(include_str!("day10.test")));
    }
}
