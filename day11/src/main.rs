use std::{collections::HashMap, str::Lines};

use itertools::{Chunk, Itertools};

fn main() {
    println!("Part 1 : {}", solve(include_str!("day11.input"), 20, false));
    println!(
        "Part 2 : {}",
        solve(include_str!("day11.input"), 10_000, true)
    );
}

fn solve(input: &str, nbr: usize, part2: bool) -> usize {
    let mut monkeys = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(Monkey::from)
        .collect::<Vec<_>>();

    let modulus = monkeys.iter().map(|x| x.test).product();

    for _ in 0..nbr {
        let mut map =
            monkeys
                .iter_mut()
                .enumerate()
                .fold(HashMap::new(), |mut map, (i, monkey)| {
                    let mut new_items = map.remove(&i).unwrap_or(Vec::new());
                    monkey.items.append(&mut new_items);

                    let mut monkey_items = if !part2 {
                        monkey.inspect()
                    } else {
                        monkey.inspect2(modulus)
                    };

                    let mut monkey_true = monkey_items.remove(0);
                    map.entry(monkey.r#true)
                        .and_modify(|items: &mut Vec<usize>| {
                            items.append(&mut monkey_true);
                        })
                        .or_insert(monkey_true);

                    let mut monkey_false = monkey_items.remove(0);
                    map.entry(monkey.r#false)
                        .and_modify(|items: &mut Vec<usize>| {
                            items.append(&mut monkey_false);
                        })
                        .or_insert(monkey_false);

                    map
                });

        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let mut new_items = map.remove(&i).unwrap_or(Vec::new());
            monkey.items.append(&mut new_items);
        }
    }
    monkeys.sort_unstable_by_key(|a| a.inspected);

    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|x| x.inspected)
        .product::<usize>()
}

impl Monkey {
    fn inspect(&mut self) -> Vec<Vec<usize>> {
        let mut monkeys = vec![Vec::new(), Vec::new()];

        while let Some(item) = self.items.pop() {
            self.inspected += 1;

            let worry = self.op.result(item) / 3;

            if worry % self.test == 0 {
                monkeys.get_mut(0).unwrap().push(worry);
            } else {
                monkeys.get_mut(1).unwrap().push(worry);
            }
        }

        monkeys
    }

    fn inspect2(&mut self, modulus: usize) -> Vec<Vec<usize>> {
        let mut monkeys = vec![Vec::new(), Vec::new()];

        while let Some(item) = self.items.pop() {
            self.inspected += 1;

            /*
                explanation of how it works :

                (a mod kn) mod n = a mod n for any integer * k.
                So instead of storing `a` we store `a mod kn` where k = the product of all of the other checks

                *edit: I mean positive integer here. negative mod is not well-defined, zero mod is not defined
                link : https://www.reddit.com/r/adventofcode/comments/zifqmh/comment/izrfgit/?utm_source=share&utm_medium=web2x&context=3
            */
            let worry = self.op.result(item) % modulus;

            if worry % self.test == 0 {
                monkeys.get_mut(0).unwrap().push(worry);
            } else {
                monkeys.get_mut(1).unwrap().push(worry);
            }
        }

        monkeys
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    r#true: usize,
    r#false: usize,
    inspected: usize,
}

#[derive(Debug)]
enum Operation {
    Add(Option<usize>),
    Mult(Option<usize>),
}

impl Operation {
    fn result(&self, value: usize) -> usize {
        match self {
            Operation::Add(x) => value + x.unwrap_or(value),
            Operation::Mult(x) => value * x.unwrap_or(value),
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .unwrap()
        {
            ("*", value) => Operation::Mult(str::parse(value).ok()),
            ("+", value) => Operation::Add(str::parse(value).ok()),
            _ => unreachable!(),
        }
    }
}

impl From<Chunk<'_, Lines<'_>>> for Monkey {
    fn from(chunk: Chunk<Lines>) -> Self {
        let mut iter = chunk.skip(1);
        let items = iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| str::parse::<usize>(x).unwrap())
            .collect();
        let op = Operation::from(iter.next().unwrap().trim());
        let test = iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .map(|x| str::parse(x).unwrap())
            .unwrap();
        let r#true = iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .map(|x| str::parse(x).unwrap())
            .unwrap();
        let r#false = iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .map(|x| str::parse(x).unwrap())
            .unwrap();

        Self {
            items,
            op,
            test,
            r#true,
            r#false,
            inspected: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_1() {
        assert_eq!(10605, solve(include_str!("day11.test"), 20, false));
    }

    #[test]
    fn test_2() {
        assert_eq!(2713310158, solve(include_str!("day11.test"), 10_000, true));
    }
}
