#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[allow(clippy::many_single_char_names)]
fn euclid_ext(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, s, t) = euclid_ext(b, a.rem_euclid(b));
    (d, t, s - (a / b) * t)
}

fn chin_rem(ns: &[(i64, i64)]) -> i64 {
    let lcm: i64 = ns.iter().map(|(m, _)| *m).product(); // all primes
    ns.iter()
        .map(|(m, a)| {
            let mm = lcm / m;
            let (_, _, s) = euclid_ext(*m, mm);
            a * s * mm
        })
        .sum::<i64>()
        % lcm
}

fn parse(s: &str) -> crate::Result<(i64, Vec<(i64, i64)>)> {
    let mut lines = s.lines();
    let t = lines
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .parse()?;
    let ns: Vec<_> = lines
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            if let Ok(n) = s.parse() {
                Some((n, i as i64))
            } else {
                None
            }
        })
        .collect();

    Ok((t, ns))
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let (t, ns) = parse(input)?;
    let (n, _) = ns
        .into_iter()
        .min_by_key(|(n, _)| n - t.rem_euclid(*n))
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    Ok((n - t.rem_euclid(n)) * n)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let (_, ns) = parse(input)?;
    let ns: Vec<_> = ns
        .into_iter()
        .map(|(n, o)| (n, (n - o).rem_euclid(n)))
        .collect();
    Ok(chin_rem(&ns))
}

#[cfg(test)]
mod tests {
    #[test]
    fn p2ex1() {
        let inp = "0\n17,x,13,19";
        assert_eq!(super::part2(inp).unwrap(), 3417);
    }
}
