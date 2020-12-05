#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn parse_ids(s: &str) -> crate::Result<Vec<i32>> {
    Ok(s.lines()
        .map(|l| {
            i32::from_str_radix(
                &l.replace("F", "0")
                    .replace("B", "1")
                    .replace("R", "1")
                    .replace("L", "0"),
                2,
            )
        })
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let id = *parse_ids(input)?
        .iter()
        .max()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    Ok(id)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut ids = parse_ids(input)?;
    ids.sort();

    let id = ids
        .windows(2)
        .filter_map(|ns| {
            if ns[1] - ns[0] == 2 {
                Some(ns[0] + 1)
            } else {
                None
            }
        })
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    Ok(id)
}
