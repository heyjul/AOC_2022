fn main() {
    println!(
        "Part 1 : {}",
        include_str!("day2.input")
            .lines()
            .map(|l| l.split_once(' ').map_or(0, |(a, b)| {
                let a = (a.bytes().next().unwrap() - b'A') as i32;
                let b = (b.bytes().next().unwrap() - b'X') as i32;

                b + 1
                    + match a - b {
                        -2 | 1 => 0,
                        0 => 3,
                        _ => 6,
                    }
            }))
            .sum::<i32>()
    );

    println!(
        "Part 2 : {}",
        include_str!("day2.input")
            .lines()
            .map(|l| l.split_once(' ').map_or(0, |(a, b)| {
                let a = (a.bytes().next().unwrap() - b'A') as i32;
                let b = (b.bytes().next().unwrap() - b'X') as i32;

                b * 3
                    + match a + b {
                        0 => 3,
                        4 => 1,
                        x => x,
                    }
            }))
            .sum::<i32>()
    );
}
