use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Clone)]
enum Rule {
    Refs(Vec<Vec<i32>>),
    Letter(char),
}

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            let c = s
                .chars()
                .nth(1)
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
            Ok(Rule::Letter(c))
        } else {
            let refs = s
                .split(" | ")
                .map(|refs| -> crate::Result<_> {
                    let refs = refs.split(' ').map(str::parse).collect::<Result<_, _>>()?;
                    Ok(refs)
                })
                .collect::<Result<_, _>>()?;
            Ok(Rule::Refs(refs))
        }
    }
}

fn parse(s: &str) -> crate::Result<(HashMap<i32, Rule>, Vec<&str>)> {
    let mut paragraphs = s.split("\n\n");

    let rules: HashMap<i32, Rule> = paragraphs
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .lines()
        .map(|l| -> crate::Result<_> {
            let mut ps = l.split(": ");
            let id = ps
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?;
            let rule = ps
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?;
            Ok((id, rule))
        })
        .collect::<Result<_, _>>()?;

    let messages = paragraphs
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .lines()
        .collect();

    Ok((rules, messages))
}

fn matches<'a>(
    rules: &'a HashMap<i32, Rule>,
    message: &[char],
    idx: usize,
    cur: &mut Vec<&'a Rule>,
) -> bool {
    if idx == message.len() {
        return cur.is_empty();
    }

    match cur.pop() {
        Some(Rule::Letter(c)) => *c == message[idx] && matches(rules, message, idx + 1, cur),
        Some(Rule::Refs(refs)) => {
            for opt in refs {
                let mut c = cur.clone();
                for r in opt.iter().rev() {
                    c.push(rules.get(&r).unwrap());
                }
                if matches(rules, message, idx, &mut c) {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (rules, messages) = parse(input)?;

    let mut cnt = 0;
    for m in messages {
        let chars: Vec<_> = m.chars().collect();
        if matches(&rules, &chars, 0, &mut vec![rules.get(&0).unwrap()]) {
            cnt += 1;
        }
    }

    Ok(cnt)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let (mut rules, messages) = parse(input)?;
    rules.insert(8, Rule::Refs(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Refs(vec![vec![42, 31], vec![42, 11, 31]]));

    let mut cnt = 0;
    for m in messages {
        let chars: Vec<_> = m.chars().collect();
        if matches(&rules, &chars, 0, &mut vec![rules.get(&0).unwrap()]) {
            cnt += 1;
        }
    }

    Ok(cnt)
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1ex1() {
        let inp = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(super::part1(inp).unwrap(), 2);
    }

    #[test]
    fn p2ex1() {
        let inp = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(super::part1(inp).unwrap(), 3);
        assert_eq!(super::part2(inp).unwrap(), 12);
    }
}
