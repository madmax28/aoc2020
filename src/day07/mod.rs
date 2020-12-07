use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type RuleSet<'a> = HashMap<&'a str, Vec<(i32, &'a str)>>;

fn parse_rules(s: &str) -> crate::Result<RuleSet> {
    let rules = s
        .lines()
        .map(|l| -> crate::Result<_> {
            let ts = l.split(" bags contain ").collect::<Vec<_>>();
            if ts.len() != 2 {
                return Err(crate::Error::boxed(Error::InvalidInput));
            }

            let es: Vec<_> = if ts[1].starts_with("no other bags") {
                Vec::new()
            } else {
                ts[1]
                    .split(", ")
                    .map(|e| -> crate::Result<_> {
                        let e = e
                            .trim_matches('.')
                            .trim_end_matches(" bag")
                            .trim_end_matches(" bags");

                        let cnt = e.trim_matches(|c: char| !c.is_numeric()).parse::<i32>()?;
                        let clr = e.trim_matches(|c: char| c.is_numeric() || c == ' ');

                        Ok((cnt, clr))
                    })
                    .collect::<Result<_, _>>()?
            };

            Ok((ts[0], es))
        })
        .collect::<Result<_, _>>()?;

    Ok(rules)
}

fn find_outer_layers<'a>(clr: &str, rules: &'a RuleSet, layers: &mut HashSet<&'a str>) {
    for (k, v) in rules.iter() {
        if v.iter().any(|(_, c)| *c == clr) {
            layers.insert(k);
            find_outer_layers(*k, rules, layers);
        }
    }
}

fn count_contained_bags(clr: &str, rules: &RuleSet) -> crate::Result<i32> {
    let mut cnt = 0;
    for (ccnt, cclr) in rules
        .get(&clr)
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
    {
        cnt += ccnt + ccnt * count_contained_bags(*cclr, rules)?;
    }
    Ok(cnt)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let rules = parse_rules(input)?;
    let mut layers = HashSet::new();
    find_outer_layers("shiny gold", &rules, &mut layers);
    Ok(layers.len() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let rules = parse_rules(input)?;
    Ok(count_contained_bags("shiny gold", &rules)?)
}
