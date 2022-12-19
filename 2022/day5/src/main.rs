use std::str::FromStr;

use std::collections::HashMap;

fn main() {
    let input = include_str!("day5.input");

    let mut crates = get_crates(input);
    let instructions = get_instructions(input);

    instructions.into_iter().for_each(|i| {
        i.part_1(&mut crates);
    });

    let mut keys = crates.keys().collect::<Vec<_>>();
    keys.sort_unstable();

    let message = keys
        .iter()
        .map(|k| {
            let last = crates.get(k).unwrap().last().unwrap();
            char::from(*last)
        })
        .collect::<String>();

    println!("Part 1 : {message}");

    let mut crates = get_crates(input);
    let instructions = get_instructions(input);

    instructions.into_iter().for_each(|i| {
        i.part_2(&mut crates);
    });

    let mut keys = crates.keys().collect::<Vec<_>>();
    keys.sort_unstable();

    let message = keys
        .iter()
        .map(|k| {
            let last = crates.get(k).unwrap().last().unwrap();
            char::from(*last)
        })
        .collect::<String>();

    println!("Part 2 : {message}");
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    let instructions: Vec<_> = input
        .lines()
        .filter_map(|l| Instruction::from_str(l).ok())
        .collect();
    instructions
}

fn get_crates(input: &str) -> HashMap<usize, Vec<u8>> {
    let crates: HashMap<usize, Vec<u8>> = input.lines().fold(HashMap::new(), |mut map, l| {
        l.as_bytes().chunks(4).enumerate().for_each(|(i, c)| {
            if c.starts_with(&[b'[']) {
                (*map.entry(i + 1).or_insert(Vec::new())).insert(0, c[1]);
            }
        });

        map
    });
    crates
}

struct Instruction {
    r#move: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace();
        let parts: Vec<usize> = parts
            .skip(1)
            .step_by(2)
            .map_while(|p| str::parse(p).ok())
            .collect();
        if parts.len() != 3 {
            return Err(());
        }

        Ok(Instruction {
            r#move: parts[0],
            from: parts[1],
            to: parts[2],
        })
    }
}

impl Instruction {
    fn part_1(self, hash: &mut HashMap<usize, Vec<u8>>) {
        let mut from = Vec::with_capacity(self.r#move);

        hash.entry(self.from).and_modify(|x| {
            (0..self.r#move).for_each(|_| {
                from.push(x.pop().unwrap());
            });
        });

        hash.entry(self.to).and_modify(|x| x.append(&mut from));
    }

    fn part_2(self, hash: &mut HashMap<usize, Vec<u8>>) {
        let mut from = Vec::with_capacity(self.r#move);

        hash.entry(self.from).and_modify(|x| {
            (0..self.r#move).for_each(|_| {
                from.insert(0, x.pop().unwrap());
            });
        });

        hash.entry(self.to).and_modify(|x| x.append(&mut from));
    }
}
