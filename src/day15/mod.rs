#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn find_nth(mut start: Vec<i32>, tgt: i32) -> crate::Result<i32> {
    let mut mem = Vec::new();
    let mut last = start
        .pop()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    for t in 2..=tgt {
        let tmp = last as usize;

        if let Some(n) = start.pop() {
            last = n;
        } else if mem.len() > tmp && mem[tmp] > 0 {
            last = t - 1 - mem[tmp];
        } else {
            last = 0;
        }

        if tmp + 1 > mem.len() {
            mem.resize(tmp + 1, 0);
        }
        mem[tmp] = t - 1;
    }

    Ok(last)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let start: Vec<_> = input
        .split(',')
        .map(str::parse)
        .rev()
        .collect::<Result<_, _>>()?;

    Ok(find_nth(start, 2020)?)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let start: Vec<_> = input
        .split(',')
        .map(str::parse)
        .rev()
        .collect::<Result<_, _>>()?;

    Ok(find_nth(start, 30000000)?)
}
