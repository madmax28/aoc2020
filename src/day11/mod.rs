use std::collections::HashMap;
use std::iter;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Floor,
    Vacant,
    Occupied,
}

fn directions() -> impl Iterator<Item = (i32, i32)> {
    (-1..=1)
        .flat_map(|dr| iter::repeat(dr).zip(-1..=1))
        .filter(|p| p != &(0, 0))
}

#[derive(Debug)]
struct Grid {
    map: HashMap<(i32, i32), Space>,
}

impl Grid {
    fn look_p1(&self, pos: (i32, i32), dir: (i32, i32)) -> Space {
        *self
            .map
            .get(&(pos.0 + dir.0, pos.1 + dir.1))
            .unwrap_or(&Space::Floor)
    }

    fn stabilize_p1(&mut self) {
        let mut m = HashMap::new();
        while m != self.map {
            for (k, v) in &self.map {
                let num_occupied = directions()
                    .map(|d| self.look_p1(*k, d))
                    .filter(|&s| s == Space::Occupied)
                    .count();

                match (v, num_occupied) {
                    (Space::Vacant, 0) => m.insert(*k, Space::Occupied),
                    (Space::Occupied, 4..=8) => m.insert(*k, Space::Vacant),
                    _ => m.insert(*k, *v),
                };
            }

            std::mem::swap(&mut m, &mut self.map);
        }
    }

    fn look_p2(&self, mut pos: (i32, i32), dir: (i32, i32)) -> Space {
        iter::repeat_with(move || {
            pos.0 += dir.0;
            pos.1 += dir.1;
            pos
        })
        .find_map(|p| match self.map.get(&p) {
            Some(Space::Floor) => None,
            Some(&s) => Some(s),
            None => Some(Space::Floor),
        })
        .unwrap()
    }

    fn stabilize_p2(&mut self) {
        let mut m = HashMap::new();
        while m != self.map {
            for (k, v) in &self.map {
                let num_occupied = directions()
                    .map(|d| self.look_p2(*k, d))
                    .filter(|&s| s == Space::Occupied)
                    .count();

                match (v, num_occupied) {
                    (Space::Vacant, 0) => m.insert(*k, Space::Occupied),
                    (Space::Occupied, 5..=8) => m.insert(*k, Space::Vacant),
                    _ => m.insert(*k, *v),
                };
            }

            std::mem::swap(&mut m, &mut self.map);
        }
    }
}

impl FromStr for Grid {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (r, l) in s.lines().enumerate() {
            for (c, v) in l.chars().enumerate() {
                map.insert(
                    (r as i32, c as i32),
                    match v {
                        'L' => Space::Vacant,
                        _ => Space::Floor,
                    },
                );
            }
        }

        Ok(Grid { map })
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut grid: Grid = input.parse()?;
    grid.stabilize_p1();
    Ok(grid.map.values().filter(|&v| *v == Space::Occupied).count() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut grid: Grid = input.parse()?;
    grid.stabilize_p2();
    Ok(grid.map.values().filter(|&v| *v == Space::Occupied).count() as i32)
}
