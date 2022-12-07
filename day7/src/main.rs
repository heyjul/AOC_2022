use std::{collections::HashMap, path::PathBuf, str::FromStr};

const TOTAL_DISK_SPACE: usize = 70_000_000;
const DISK_SPACE_REQUIRED: usize = 30_000_000;

fn main() {
    println!("Part 1: {}", part_1(include_str!("day7.input")));
    println!("Part 2: {}", part_2(include_str!("day7.input")));
}

fn part_1(s: &str) -> usize {
    get(s).values().filter(|v| **v <= 100_000).sum::<usize>()
}

fn part_2(s: &str) -> usize {
    let map = get(s);
    let used_space = map.values().max().unwrap();

    *map.values()
        .filter(|v| TOTAL_DISK_SPACE - (used_space - **v) >= DISK_SPACE_REQUIRED)
        .min()
        .unwrap()
}

fn get(s: &str) -> HashMap<PathBuf, usize> {
    let (_, map) = s.lines().fold(
        (PathBuf::new(), HashMap::new()),
        |(mut path, mut map), line| match Type::from_str(line) {
            Ok(Type::Action(p)) => {
                path = match p.as_str() {
                    ".." => path.parent().unwrap().to_path_buf(),
                    _ => {
                        path.push(p);
                        path
                    }
                };

                (path, map)
            }
            Ok(Type::File(size)) => {
                (*map.entry(path.to_owned()).or_insert(0)) += size;

                let mut current = path.parent();
                while let Some(parent) = current {
                    current = parent.parent();
                    (*map.entry(parent.to_path_buf()).or_insert(0)) += size;
                }

                (path, map)
            }
            _ => (path, map),
        },
    );

    map
}

#[derive(Debug)]
enum Type {
    Action(String),
    File(usize),
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("$ cd ") {
            Some(path) => return Ok(Type::Action(path.to_owned())),
            None => {}
        };

        if let Ok(size) = str::parse(s.split_whitespace().next().unwrap()) {
            return Ok(Type::File(size));
        }

        Err(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        assert_eq!(95437, crate::part_1(input));
    }

    #[test]
    fn part_2() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        assert_eq!(24933642, crate::part_2(input));
    }
}
