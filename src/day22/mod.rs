use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn parse(s: &str) -> crate::Result<(VecDeque<i32>, VecDeque<i32>)> {
    let mut decks = Vec::new();
    for p in s.split("\n\n") {
        decks.push(
            p.lines()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        );
    }

    if decks.len() != 2 {
        Err(crate::Error::boxed(Error::InvalidInput))
    } else {
        let p2 = decks.pop().unwrap();
        let p1 = decks.pop().unwrap();
        Ok((p1, p2))
    }
}

fn play_p1(decks: &mut (VecDeque<i32>, VecDeque<i32>)) {
    while !decks.0.is_empty() && !decks.1.is_empty() {
        let p1 = decks.0.pop_front().unwrap();
        let p2 = decks.1.pop_front().unwrap();

        if p1 > p2 {
            decks.0.push_back(p1);
            decks.0.push_back(p2);
        } else {
            decks.1.push_back(p2);
            decks.1.push_back(p1);
        }
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut decks = parse(input)?;
    play_p1(&mut decks);

    let winner = if !decks.0.is_empty() {
        decks.0
    } else {
        decks.1
    };

    let mut cnt = 0;
    for (i, n) in winner.iter().rev().enumerate() {
        cnt += (i + 1) as i32 * n;
    }

    Ok(cnt)
}

fn play_p2(decks: &mut (VecDeque<i32>, VecDeque<i32>)) -> i32 {
    let mut visited = HashSet::new();
    while !decks.0.is_empty() && !decks.1.is_empty() {
        if !visited.insert((decks.0.clone(), decks.1.clone())) {
            return 1;
        }

        let p1 = decks.0.pop_front().unwrap();
        let p2 = decks.1.pop_front().unwrap();

        let winner = if p1 <= decks.0.len() as i32 && p2 <= decks.1.len() as i32 {
            let d1 = decks.0.iter().cloned().take(p1 as usize).collect();
            let d2 = decks.1.iter().cloned().take(p2 as usize).collect();
            play_p2(&mut (d1, d2))
        } else if p1 > p2 {
            1
        } else {
            2
        };

        if winner == 1 {
            decks.0.push_back(p1);
            decks.0.push_back(p2);
        } else {
            decks.1.push_back(p2);
            decks.1.push_back(p1);
        }
    }

    if decks.1.is_empty() {
        1
    } else {
        0
    }
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut decks = parse(input)?;
    play_p2(&mut decks);

    let winner = if !decks.0.is_empty() {
        decks.0
    } else {
        decks.1
    };

    let mut cnt = 0;
    for (i, n) in winner.iter().rev().enumerate() {
        cnt += (i + 1) as i32 * n;
    }

    Ok(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2ex1() {
        let inp = "Player 1:
43
19

Player 2:
2
29
14";
        part2(inp).unwrap();
    }

    #[test]
    fn p2ex2() {
        let inp = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(part2(inp).unwrap(), 291);
    }
}
