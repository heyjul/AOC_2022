fn main() {
    let mut vec = Vec::new();
    let last = include_str!("day1.input")
        .lines()
        .fold(0, |acc, line| match line.parse::<i32>() {
            Ok(x) => acc + x,
            Err(_) => {
                vec.push(acc);
                0
            }
        });
    vec.push(last);
    vec.sort_unstable_by(|a, b| b.cmp(a));

    println!("Part 1 : {}", vec.iter().next().unwrap());
    println!("Part 2 : {}", vec.iter().take(3).sum::<i32>());
}
