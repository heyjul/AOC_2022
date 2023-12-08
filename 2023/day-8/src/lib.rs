use std::collections::HashMap;

pub fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let dir = lines.next().unwrap().trim();
    (
        dir.to_owned(),
        lines
            .filter_map(|x| x.split_once(" = "))
            .fold(HashMap::new(), |mut acc, (x, y)| {
                let (y1, y2) = y.split_once(", ").unwrap();
                acc.insert(
                    x.trim().to_owned(),
                    (
                        y1.trim().trim_start_matches('(').to_owned(),
                        y2.trim().trim_end_matches(')').to_owned(),
                    ),
                );
                acc
            }),
    )
}
