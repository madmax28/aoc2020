#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn transform(subject: i64, loop_sz: i64) -> i64 {
    let mut res = 1;
    for _ in 0..loop_sz {
        res = (res * subject) % 20201227;
    }
    res
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let pubs: Vec<i64> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    if pubs.len() != 2 {
        return Err(crate::Error::boxed(Error::InvalidInput));
    }

    let mut p = 1;
    for loop_sz in 1.. {
        p = (p * 7) % 20201227;

        if p == pubs[0] {
            return Ok(transform(pubs[1], loop_sz));
        }
    }

    unreachable!();
}
