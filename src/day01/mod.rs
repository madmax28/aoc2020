#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn find_n(nums: &[i32], sum: i32, start: usize, n: i32, vs: &mut Vec<i32>) -> bool {
    for i in start..nums.len() {
        let ssum = sum + nums[i];
        if ssum > 2020 {
            continue;
        }

        if (n == 1 && ssum == 2020) || (n > 1 && find_n(nums, ssum, i + 1, n - 1, vs)) {
            vs.push(nums[i]);
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let nums = input
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vs = Vec::new();
    if find_n(&nums, 0, 0, 2, &mut vs) {
        return Ok(vs.iter().product());
    }
    Err(crate::Error::boxed(Error::InvalidInput))
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let nums = input
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vs = Vec::new();
    if find_n(&nums, 0, 0, 3, &mut vs) {
        return Ok(vs.iter().product());
    }
    Err(crate::Error::boxed(Error::InvalidInput))
}
