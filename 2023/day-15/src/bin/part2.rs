use day_15::*;

fn main() {
    let input = include_str!("../input2.txt");
    let answer = process(input);
    println!("Part 2 : {answer}");
}

fn process(input: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    let input = parse_input(input);

    input.into_iter().for_each(|x| {
        let label: String = x.chars().take_while(char::is_ascii_alphabetic).collect();
        let operation: String = x.chars().skip(label.len()).collect();

        let i = label
            .as_bytes()
            .into_iter()
            .map(|x| *x as usize)
            .fold(0, |acc, x| ((acc + x) * 17) % 256);

        let mut operation = operation.split_inclusive(|x: char| x.is_ascii_punctuation());

        let op = operation.next().unwrap();
        let val = operation.next();

        match op {
            "=" => {
                let content = &mut boxes[i];
                if let Some(content) = content.iter_mut().find(|x| x.0 == label) {
                    content.1 = val.unwrap().parse().unwrap();
                } else {
                    content.push((label, val.unwrap().parse().unwrap()));
                }
            }
            "-" => {
                let content = &mut boxes[i];
                if let Some(i) = content.iter().position(|x| x.0 == label) {
                    content.remove(i);
                }
            }
            _ => unreachable!(),
        };
    });

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(move |(j, x)| x.1 * (j + 1) * (i + 1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(145, process(INPUT));
    }
}
