use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type Id = i32;
type IdSet = HashMap<String, Id>;
type RuleSet = HashMap<Id, Vec<(i32, Id)>>;

fn parse_rules(s: &str) -> crate::Result<(IdSet, RuleSet)> {
    let mut id_cnt = -1;
    let mut ids = HashMap::new();
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
                        let mut ws = e.split(' ');
                        let cnt: i32 = ws
                            .next()
                            .and_then(|w| w.parse().ok())
                            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

                        let clr: Vec<_> = ws.take_while(|w| !w.starts_with("bag")).collect();
                        let clr = *ids.entry(clr.join(" ")).or_insert_with(|| {
                            id_cnt += 1;
                            id_cnt
                        });

                        Ok((cnt, clr))
                    })
                    .collect::<Result<_, _>>()?
            };

            Ok((
                *ids.entry(ts[0].to_string()).or_insert_with(|| {
                    id_cnt += 1;
                    id_cnt
                }),
                es,
            ))
        })
        .collect::<Result<_, _>>()?;
    Ok((ids, rules))
}

fn find_outer_layers(clr: i32, rules: &RuleSet, layers: &mut HashSet<Id>) {
    for (k, v) in rules.iter() {
        if v.iter().any(|(_, c)| *c == clr) {
            layers.insert(*k);
            find_outer_layers(*k, rules, layers);
        }
    }
}

fn count_contained_bags(clr: i32, rules: &RuleSet) -> crate::Result<i32> {
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
    let (ids, rules) = parse_rules(input)?;

    let mut layers = HashSet::new();
    find_outer_layers(
        *ids.get("shiny gold")
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
        &rules,
        &mut layers,
    );

    Ok(layers.len() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let (ids, rules) = parse_rules(input)?;
    Ok(count_contained_bags(
        *ids.get("shiny gold")
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
        &rules,
    )?)
}
