pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == '#' {
                vec.push((x, y));
            }
        }
    }

    vec
}
