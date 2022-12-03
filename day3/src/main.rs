#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

fn main() {
    println!(
        "Part 1 : {}",
        include_str!("day3.input")
            .lines()
            .map(|l| {
                let (c1, c2) = l.split_at(l.len() / 2);
                let hash = c1.as_bytes().iter().collect::<HashSet<_>>();
                c2.as_bytes().iter().find(|b| hash.contains(*b)).unwrap()
            })
            .map(|c| if c.is_ascii_lowercase() {
                c - b'a' + 1
            } else {
                c - b'A' + 27
            } as u32)
            .sum::<u32>()
    );

    println!(
        "Part 2 : {}",
        include_str!("day3.input")
            .lines()
            .array_chunks()
            .map(|[l1, l2, l3]| {
                let mut hash = HashMap::new();

                l1.as_bytes().iter().for_each(|c| {
                    hash.entry(c).or_insert((true, false, false));
                });
                l2.as_bytes().iter().for_each(|c| {
                    (*hash.entry(c).or_insert((false, true, false))).1 = true;
                });
                l3.as_bytes().iter().for_each(|c| {
                    (*hash.entry(c).or_insert((false, false, true))).2 = true;
                });

                *hash.iter().find(|h| h.1 .0 && h.1 .1 && h.1 .2).unwrap().0
            })
            .map(|c| if c.is_ascii_lowercase() {
                c - b'a' + 1
            } else {
                c - b'A' + 27
            } as u32)
            .sum::<u32>()
    );

    // Found a better way on https://www.geeksforgeeks.org/find-common-elements-three-sorted-arrays/
    println!(
        "Part 2 with a real algorithm : {}",
        include_str!("day3.input")
            .lines()
            .array_chunks()
            .map(|[l1, l2, l3]| {
                let mut l1: Vec<_> = l1.as_bytes().to_owned();
                let mut l2: Vec<_> = l2.as_bytes().to_owned();
                let mut l3: Vec<_> = l3.as_bytes().to_owned();

                l1.sort_unstable();
                l2.sort_unstable();
                l3.sort_unstable();

                let (mut i, mut j, mut k) = (0, 0, 0);

                loop {
                    if l1[i] == l2[j] && l2[j] == l3[k] {
                        return l1[i];
                    } else if l1[i] < l2[j] {
                        i += 1;
                    } else if l2[j] < l3[k] {
                        j += 1;
                    } else {
                        k += 1;
                    }
                }
            })
            .map(|c| if c.is_ascii_lowercase() {
                c - b'a' + 1
            } else {
                c - b'A' + 27
            } as u32)
            .sum::<u32>()
    );
}
