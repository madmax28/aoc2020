use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
struct Range(RangeInclusive<i32>);

impl FromStr for Range {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ns = s.split('-').map(str::parse);
        let n1 = ns
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))??;
        let n2 = ns
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))??;
        Ok(Range(n1..=n2))
    }
}

#[derive(Debug)]
struct Constraint(Vec<Range>);

impl Constraint {
    fn matches(&self, n: i32) -> bool {
        self.0.iter().any(|r| r.0.contains(&n))
    }
}

impl FromStr for Constraint {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Constraint(
            s.split(" or ").map(str::parse).collect::<Result<_, _>>()?,
        ))
    }
}

type RuleSet<'a> = HashMap<&'a str, Constraint>;

#[derive(Debug)]
struct Ticket(Vec<i32>);

impl FromStr for Ticket {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket(
            s.split(',').map(str::parse).collect::<Result<_, _>>()?,
        ))
    }
}

// glorious!
fn parse(s: &str) -> crate::Result<(RuleSet, Ticket, Vec<Ticket>)> {
    let mut ps = s.split("\n\n");

    let rules = ps
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .lines()
        .map(|l| -> crate::Result<_> {
            let mut tokens = l.split(": ");
            let field = tokens
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
            let rule = tokens
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?;
            Ok((field, rule))
        })
        .collect::<Result<_, _>>()?;

    let mine = ps
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .lines()
        .nth(1)
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .parse()?;

    let nearby = ps
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .lines()
        .skip(1)
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((rules, mine, nearby))
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (rules, _, nearby) = parse(input)?;

    let mut sum = 0;
    for t in nearby {
        for n in t.0 {
            if rules.values().all(|r| !r.matches(n)) {
                sum += n;
            }
        }
    }
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let (rules, mine, nearby) = parse(input)?;

    let nearby = nearby
        .into_iter()
        .filter(|t| t.0.iter().all(|n| rules.values().any(|r| r.matches(*n))));

    let mut candidates: Vec<Vec<&str>> = {
        let fields: Vec<&str> = rules.keys().cloned().collect();
        let len = fields.len();
        vec![fields; len]
    };

    for t in nearby {
        for (i, n) in t.0.iter().enumerate() {
            if i >= candidates.len() {
                return Err(crate::Error::boxed(Error::InvalidInput));
            }

            candidates[i] = candidates[i]
                .iter()
                .cloned()
                .filter(|f| rules[f].matches(*n))
                .collect();
        }
    }

    let mut positions = HashMap::new();
    while candidates.iter().any(|cands| !cands.is_empty()) {
        let (i, f) = candidates
            .iter()
            .enumerate()
            .find_map(|(i, f)| if f.len() == 1 { Some((i, f[0])) } else { None })
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
        positions.insert(i, f);

        candidates = candidates
            .into_iter()
            .map(|cands| cands.into_iter().filter(|&ff| ff != f).collect())
            .collect();
    }

    let prod = positions
        .iter()
        .filter_map(|(&i, f)| {
            if f.starts_with("departure") {
                Some(mine.0[i] as i64)
            } else {
                None
            }
        })
        .product();

    Ok(prod)
}
