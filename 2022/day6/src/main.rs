fn main() {
    println!("{}", part_1(include_str!("day6.input")));
    println!("{}", part_2(include_str!("day6.input")));
}

fn part_1(s: &str) -> usize {
    solve(s, 4)
}

fn part_2(s: &str) -> usize {
    solve(s, 14)
}

fn solve(s: &str, unique: usize) -> usize {
    let s = s.as_bytes();
    let mut letters = s.iter().take(unique).fold([0_u8; 26], |mut v, c| {
        v[(c - b'a') as usize] += 1;
        v
    });

    if letters.iter().filter(|v| **v == 1).count() == unique {
        return unique;
    }

    for i in 0..s.len() - unique {
        letters[(s[i] - b'a') as usize] -= 1;
        letters[(s[i + unique] - b'a') as usize] += 1;

        if letters.iter().filter(|v| **v == 1).count() == unique {
            return i + unique + 1;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        assert_eq!(7, crate::part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, crate::part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, crate::part_1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, crate::part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, crate::part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part_2() {
        assert_eq!(14, crate::part_2("abcdefghijklmn"));
        assert_eq!(19, crate::part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, crate::part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, crate::part_2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, crate::part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, crate::part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
