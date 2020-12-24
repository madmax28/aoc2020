use std::collections::{HashMap, HashSet};
use std::iter;

fn neighbors(p: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    vec![
        (p.0 + 1, p.1),
        (p.0 + 1, p.1 - 1),
        (p.0, p.1 - 1),
        (p.0 - 1, p.1),
        (p.0 - 1, p.1 + 1),
        (p.0, p.1 + 1),
    ]
    .into_iter()
}

#[derive(Debug)]
enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Dir {
    fn apply(&self, p: (i32, i32)) -> (i32, i32) {
        match self {
            Dir::East => (p.0 + 1, p.1),
            Dir::SouthEast => (p.0 + 1, p.1 - 1),
            Dir::SouthWest => (p.0, p.1 - 1),
            Dir::West => (p.0 - 1, p.1),
            Dir::NorthWest => (p.0 - 1, p.1 + 1),
            Dir::NorthEast => (p.0, p.1 + 1),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    White,
    Black,
}

fn parse(s: &str) -> Vec<Vec<Dir>> {
    let mut insns = Vec::new();
    for l in s.lines() {
        let mut store = None;
        let mut dirs = Vec::new();
        for c in l.chars() {
            match c {
                'n' => store = Some('n'),
                's' => store = Some('s'),
                'e' => {
                    if let Some(c) = store.take() {
                        match c {
                            'n' => dirs.push(Dir::NorthEast),
                            's' => dirs.push(Dir::SouthEast),
                            _ => panic!(),
                        }
                    } else {
                        dirs.push(Dir::East);
                    }
                }
                'w' => {
                    if let Some(c) = store.take() {
                        match c {
                            'n' => dirs.push(Dir::NorthWest),
                            's' => dirs.push(Dir::SouthWest),
                            _ => panic!(),
                        }
                    } else {
                        dirs.push(Dir::West);
                    }
                }
                _ => panic!(),
            }
        }
        insns.push(dirs);
    }
    insns
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let insns = parse(input);

    let mut colors: HashMap<(i32, i32), Color> = HashMap::new();
    for i in insns {
        let mut p = (0, 0);
        for d in i {
            p = d.apply(p);
        }

        if colors.remove(&p).is_none() {
            colors.insert(p, Color::Black);
        }
    }

    Ok(colors.len() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let insns = parse(input);

    let mut colors: HashMap<(i32, i32), Color> = HashMap::new();
    for i in insns {
        let mut p = (0, 0);
        for d in i {
            p = d.apply(p);
        }

        if colors.remove(&p).is_none() {
            colors.insert(p, Color::Black);
        }
    }

    for _ in 0..100 {
        let mut next = HashMap::new();

        let to_check: HashSet<(i32, i32)> = colors
            .keys()
            .flat_map(|p| iter::once(*p).chain(neighbors(*p)))
            .collect();

        for p in to_check {
            let color = *colors.get(&p).unwrap_or(&Color::White);
            let black_cnt = neighbors(p).filter(|p| colors.contains_key(&p)).count();

            match (color, black_cnt) {
                (Color::White, 2) | (Color::Black, 1..=2) => {
                    next.insert(p, Color::Black);
                }
                _ => (),
            }
        }

        colors = next;
    }

    Ok(colors.len() as i32)
}
