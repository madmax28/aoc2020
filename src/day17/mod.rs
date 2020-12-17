use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Active,
    Inactive,
}

trait Point {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>>;
    fn from_2d(p: (i32, i32)) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3d(i32, i32, i32);

impl Point for Point3d {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>> {
        let p = *self;
        let it = (-1..=1)
            .flat_map(|dx| iter::repeat(dx).zip(-1..=1))
            .flat_map(|dy| iter::repeat(dy).zip(-1..=1))
            .map(|((x, y), z)| (x, y, z))
            .filter(|p| p != &(0, 0, 0))
            .map(move |d| Point3d(p.0 + d.0, p.1 + d.1, p.2 + d.2));
        Box::new(it)
    }

    fn from_2d(p: (i32, i32)) -> Self {
        Point3d(p.0, p.1, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point4d(i32, i32, i32, i32);

impl Point for Point4d {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>> {
        let p = *self;
        let it = (-1..=1)
            .flat_map(|dx| iter::repeat(dx).zip(-1..=1))
            .flat_map(|dy| iter::repeat(dy).zip(-1..=1))
            .flat_map(|dz| iter::repeat(dz).zip(-1..=1))
            .map(|(((x, y), z), w)| (x, y, z, w))
            .filter(|p| p != &(0, 0, 0, 0))
            .map(move |d| Point4d(p.0 + d.0, p.1 + d.1, p.2 + d.2, p.3 + d.3));
        Box::new(it)
    }

    fn from_2d(p: (i32, i32)) -> Self {
        Point4d(p.0, p.1, 0, 0)
    }
}

#[derive(Debug)]
struct Grid<T> {
    map: HashMap<T, Cell>,
}

impl<T: Point + Eq + Hash + Copy> Grid<T> {
    fn step(&mut self) {
        let candidates: HashSet<T> = self
            .map
            .keys()
            .flat_map(|p| iter::once(*p).chain(p.neighbors()))
            .collect();

        let mut m = HashMap::new();
        for p in candidates {
            let state = self.map.get(&p).unwrap_or(&Cell::Inactive);
            let num_active_neighbors = p
                .neighbors()
                .map(|p| self.map.get(&p).unwrap_or(&Cell::Inactive))
                .filter(|&&s| s == Cell::Active)
                .count();

            match (state, num_active_neighbors) {
                (Cell::Active, 2..=3) => {
                    m.insert(p, Cell::Active);
                }
                (Cell::Inactive, 3) => {
                    m.insert(p, Cell::Active);
                }
                _ => (),
            };
        }
        self.map = m;
    }
}

impl<T: Point + Eq + Hash> FromStr for Grid<T> {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (r, l) in s.lines().enumerate() {
            for (c, v) in l.chars().enumerate() {
                if v == '#' {
                    map.insert(T::from_2d((r as i32, c as i32)), Cell::Active);
                }
            }
        }

        Ok(Grid { map })
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut grid: Grid<Point3d> = input.parse()?;
    for _ in 0..6 {
        grid.step();
    }
    Ok(grid.map.len() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut grid: Grid<Point4d> = input.parse()?;
    for _ in 0..6 {
        grid.step();
    }
    Ok(grid.map.len() as i32)
}
