use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut chain: Vec<i32> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    chain.push(0); // outlet joltage
    chain.push(chain.iter().max().unwrap() + 3); // built-in adapter
    chain.sort_unstable();

    let mut diffs = HashMap::new();
    for ns in chain.windows(2) {
        *diffs.entry(ns[1] - ns[0]).or_insert(0) += 1;
    }

    let ones = diffs
        .get(&1)
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    let threes = diffs
        .get(&3)
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    Ok(ones * threes)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let mut chain: Vec<i32> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    chain.push(0); // outlet joltage
    chain.push(chain.iter().max().unwrap() + 3); // built-in adapter
    chain.sort_unstable();

    let mut seqs = vec![0];
    for d in chain.windows(2).map(|ns| ns[1] - ns[0]) {
        if d == 1 {
            *seqs.last_mut().unwrap() += 1;
        } else {
            seqs.push(0);
        }
    }

    let n = seqs.iter().fold(1i64, |acc, n| match n {
        2 => acc * 2,
        3 => acc * 4,
        4 => acc * 7,
        _ => acc,
    });
    Ok(n)
}
