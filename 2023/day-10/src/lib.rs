pub fn parse_input(input: &str) -> Vec<Vec<[bool; 4]>> {
    input
        .lines()
        .map(|x| {
            x.trim()
                .as_bytes()
                .into_iter()
                .map(|x| match x {
                    b'|' => [true, false, true, false],
                    b'-' => [false, true, false, true],
                    b'L' => [true, true, false, false],
                    b'J' => [true, false, false, true],
                    b'7' => [false, false, true, true],
                    b'F' => [false, true, true, false],
                    b'.' => [false; 4],
                    b'S' => [true; 4],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}
