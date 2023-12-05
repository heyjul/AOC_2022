fn main() {
    let input = include_str!("input.txt");
    let answer = part1(input);
    println!("Part 1 : {answer}");
    let answer = part2(input);
    println!("Part 2 : {answer}");
}

fn part1(input: &str) -> u64 {
    let mut cleansed = input
        .split(|x: char| x.is_ascii_alphabetic() || x.is_ascii_punctuation())
        .map(|x| {
            x.lines()
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|x| x.len() != 0);

    let seeds: Vec<u64> = cleansed
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let converters: Vec<Vec<_>> = cleansed
        .map(|x| x.into_iter().map(Converter::from).collect())
        .collect();

    seeds
        .into_iter()
        .map(|seed| {
            converters.iter().fold(seed, |seed, x| {
                x.iter().find_map(|x| x.next(seed)).unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let mut cleansed = input
        .split(|x: char| x.is_ascii_alphabetic() || x.is_ascii_punctuation())
        .map(|x| {
            x.lines()
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|x| x.len() != 0);

    let seeds: Vec<_> = cleansed
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| {
            let start = x.first().unwrap().parse::<u64>().unwrap();
            let count = x.last().unwrap().parse::<u64>().unwrap();
            start..start + count
        })
        .collect();

    let converters: Vec<Vec<_>> = cleansed
        .map(|x| x.into_iter().map(Converter::from).collect())
        .collect();

    seeds
        .into_iter()
        .filter_map(|range| {
            range
                .map(|seed| {
                    converters.iter().fold(seed, |seed, x| {
                        x.iter().find_map(|x| x.next(seed)).unwrap_or(seed)
                    })
                })
                .min()
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Converter {
    dest: u64,
    source: u64,
    range: u64,
}

impl From<&str> for Converter {
    fn from(value: &str) -> Self {
        let values: Vec<_> = value.splitn(3, ' ').collect();
        Self {
            dest: values[0].parse().unwrap(),
            source: values[1].parse().unwrap(),
            range: values[2].parse().unwrap(),
        }
    }
}

impl Converter {
    fn next(&self, value: u64) -> Option<u64> {
        if self.source <= value && self.source + self.range >= value {
            Some(self.dest + (value - self.source))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        assert_eq!(35, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        assert_eq!(46, part2(INPUT));
    }
}
