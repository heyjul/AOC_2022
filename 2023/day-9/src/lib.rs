pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect()
}
