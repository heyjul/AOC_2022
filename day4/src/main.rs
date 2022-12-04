fn main() {
    let input: Vec<_> = include_str!("day4.input")
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, c)| {
            let (a, b) = a.split_once('-').unwrap();
            let (c, d) = c.split_once('-').unwrap();

            (
                a.parse::<usize>().unwrap(),
                b.parse::<usize>().unwrap(),
                c.parse::<usize>().unwrap(),
                d.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let part_1 = input
        .iter()
        .filter(|&&(a, b, c, d)| {
            if c > b || a > d {
                return false;
            }

            if b - a > d - c {
                (c..d).all(|x| (a..b).contains(&x))
            } else {
                (a..b).all(|x| (c..d).contains(&x))
            }
        })
        .count();

    let part_2 = input
        .iter()
        .filter(|&&(a, b, c, d)| c <= b && a <= d)
        .count();

    println!("Part 1 : {part_1}");
    println!("Part 2 : {part_2}");
}
