use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> crate::Result<i32> {
    let sum = input
        .split("\n\n")
        .map(|e| {
            e.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<_>>()
                .len() as i32
        })
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let sum = input
        .split("\n\n")
        .map(|e| {
            let mut ns = HashMap::new();
            for c in e.chars() {
                *ns.entry(c).or_insert(0) += 1;
            }

            let grp_sz = *ns.get(&'\n').unwrap_or(&0) + 1;
            ns.into_iter()
                .filter(|(c, n)| *c != '\n' && *n == grp_sz)
                .count() as i32
        })
        .sum();
    Ok(sum)
}
