use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type ValidSet = VecDeque<Vec<i64>>;

pub fn part1(input: &str) -> crate::Result<i64> {
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<i64>, _>>()?;
    const LEN: usize = 25;
    let mut valid: ValidSet = vec![Vec::new(); LEN - 1].into();

    for i in 0..LEN - 1 {
        valid[i].extend(nums[i + 1..LEN].iter().map(|nn| nums[i] + nn));
    }

    for w in nums.windows(LEN).skip(1) {
        let n = w.last().unwrap();
        if !valid.iter().any(|v| v.contains(n)) {
            return Ok(*n);
        }

        valid.pop_front();
        valid.push_back(Vec::new());
        for i in 0..LEN - 1 {
            valid[i].push(n + w[i]);
        }
    }

    Err(crate::Error::boxed(Error::InvalidInput))
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<i64>, _>>()?;
    const INVALID: i64 = 41682220;

    for start in 0..nums.len() {
        let mut sum = nums[start];
        for i in start + 1..nums.len() {
            sum += nums[i];
            match sum.cmp(&INVALID) {
                Ordering::Equal => {
                    return Ok(nums[start..=i].iter().min().unwrap()
                        + nums[start..=i].iter().max().unwrap())
                }
                Ordering::Greater => break,
                _ => (),
            }
        }
    }

    Err(crate::Error::boxed(Error::InvalidInput))
}
