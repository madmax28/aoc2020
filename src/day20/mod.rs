use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::iter;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pair(i32, i32);

impl Pair {
    fn rotate(&self, sz: Pair) -> Pair {
        let mut p = *self;

        p.0 = p.0 * 2 - sz.0 + 1;
        p.1 = p.1 * 2 - sz.1 + 1;

        let tmp = p.0;
        p.0 = -p.1;
        p.1 = tmp;

        p.0 = (p.0 + sz.0 - 1) / 2;
        p.1 = (p.1 + sz.1 - 1) / 2;

        p
    }

    fn flip(&self, sz: Pair) -> Pair {
        Pair(sz.0 - 1 - self.0, self.1)
    }

    fn up(&self) -> Pair {
        Pair(self.0, self.1 - 1)
    }

    fn left(&self) -> Pair {
        Pair(self.0 - 1, self.1)
    }
}

impl From<(i32, i32)> for Pair {
    fn from(tuple: (i32, i32)) -> Self {
        Pair(tuple.0, tuple.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

const EDGES: &[Edge] = &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];

#[derive(Debug)]
enum Trans {
    Rot,
    Flip,
}

const TRANS: &[Option<Trans>] = &[
    None,
    Some(Trans::Rot),
    Some(Trans::Rot),
    Some(Trans::Rot),
    Some(Trans::Flip),
    Some(Trans::Rot),
    Some(Trans::Rot),
    Some(Trans::Rot),
];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Orientation {
    rot: i32,
    flip: bool,
}

impl Orientation {
    fn rotate(&mut self) {
        if self.flip {
            self.rot -= 1;
        } else {
            self.rot += 1;
        }
        self.rot = (self.rot + 4) % 4;
    }

    fn flip(&mut self) {
        self.flip = !self.flip;
    }
}

#[derive(Debug, Clone)]
struct Image {
    sz: Pair,
    pixels: HashMap<Pair, (char, bool)>,
    orientation: Orientation,
    key_cache: HashMap<(Edge, Orientation), i32>,
}

impl Image {
    fn new(sz: Pair, pixels: HashMap<Pair, (char, bool)>) -> Self {
        Image {
            sz,
            pixels,
            orientation: Orientation {
                rot: 0,
                flip: false,
            },
            key_cache: HashMap::new(),
        }
    }

    fn populate_cache(&mut self) {
        let mut img = self.clone();
        for trans in TRANS {
            match trans {
                Some(Trans::Rot) => {
                    img.rotate_pixels();
                    img.rotate();
                }
                Some(Trans::Flip) => {
                    img.flip_pixels();
                    img.flip();
                }
                None => (),
            }

            for edge in EDGES {
                let key = img.calc_key(*edge);
                self.key_cache.insert((*edge, img.orientation.clone()), key);
            }
        }
    }

    fn coords(&self) -> impl Iterator<Item = Pair> {
        let sz = self.sz;
        (0..sz.1)
            .flat_map(move |y| iter::repeat(y).zip(0..sz.0))
            .map(|(y, x)| Pair(x, y))
    }

    fn apply_pattern<'a>(&mut self, p: Pair, ps: impl IntoIterator<Item = &'a Pair>) {
        for d in ps {
            self.pixels.get_mut(&Pair(p.0 + d.0, p.1 + d.1)).unwrap().1 = true;
        }
    }

    fn matches_pattern<'a>(&self, p: Pair, ps: impl IntoIterator<Item = &'a Pair>) -> bool {
        ps.into_iter().all(|d| {
            if let Some((c, _)) = self.pixels.get(&Pair(p.0 + d.0, p.1 + d.1)) {
                *c == '#'
            } else {
                false
            }
        })
    }

    fn crop(&mut self) {
        let mut pixels = HashMap::new();
        for src in self
            .coords()
            .filter(|p| p.0 != 0 && p.0 != self.sz.0 - 1 && p.1 != 0 && p.1 != self.sz.1 - 1)
        {
            let dst = Pair(src.0 - 1, src.1 - 1);
            pixels.insert(dst, *self.pixels.get(&src).unwrap());
        }

        self.sz = Pair(self.sz.0 - 2, self.sz.1 - 2);
        self.pixels = pixels;
        self.key_cache.clear();
    }

    fn apply_orientation(&mut self) {
        for _ in 0..self.orientation.rot {
            self.rotate_pixels();
        }

        if self.orientation.flip {
            self.flip_pixels();
        }

        self.orientation = Orientation {
            rot: 0,
            flip: false,
        };

        self.key_cache.clear();
    }

    fn rotate(&mut self) {
        self.orientation.rotate();
    }

    fn rotate_pixels(&mut self) {
        let mut pixels = HashMap::new();
        for src in self.coords() {
            let dst = src.rotate(self.sz);
            pixels.insert(dst, *self.pixels.get(&src).unwrap());
        }
        self.pixels = pixels;
        self.key_cache.clear();
    }

    fn flip(&mut self) {
        self.orientation.flip();
    }

    fn flip_pixels(&mut self) {
        let mut pixels = HashMap::new();
        for src in self.coords() {
            let dst = src.flip(self.sz);
            pixels.insert(dst, *self.pixels.get(&src).unwrap());
        }
        self.pixels = pixels;
        self.key_cache.clear();
    }

    fn key(&self, edge: Edge) -> i32 {
        *self
            .key_cache
            .get(&(edge, self.orientation.clone()))
            .unwrap()
    }

    fn calc_key(&self, edge: Edge) -> i32 {
        let ps: Vec<Pair> = match edge {
            Edge::Top => (0..self.sz.0)
                .zip(iter::repeat(0))
                .map(Pair::from)
                .collect(),
            Edge::Right => iter::repeat(self.sz.0 - 1)
                .zip(0..self.sz.1)
                .map(Pair::from)
                .collect(),
            Edge::Bottom => (0..self.sz.0)
                .zip(iter::repeat(self.sz.1 - 1))
                .map(Pair::from)
                .collect(),
            Edge::Left => iter::repeat(0).zip(0..self.sz.1).map(Pair::from).collect(),
        };

        let key = ps
            .iter()
            .fold(0i32, |acc, p| match self.pixels.get(&p).unwrap().0 {
                '#' => (acc << 1) + 1,
                _ => acc << 1,
            });
        key
    }
}

#[allow(dead_code)]
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.sz.1 {
            for x in 0..self.sz.0 {
                write!(f, "{}", self.pixels.get(&Pair(x, y)).unwrap().0)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Image {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sz = Pair(0, 0);
        let pixels = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
            .map(|(y, (x, c))| {
                sz.0 = std::cmp::max(sz.0, x as i32 + 1);
                sz.1 = std::cmp::max(sz.1, y as i32 + 1);
                (Pair(x as i32, y as i32), (c, false))
            })
            .collect();

        if sz.0 != sz.1 {
            return Err(crate::Error::boxed(Error::InvalidInput));
        }

        Ok(Image::new(sz, pixels))
    }
}

struct Tile {
    id: i32,
    img: Image,
}

impl FromStr for Tile {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id: i32 = lines
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()?;

        let img = lines.collect::<Vec<_>>();
        let img = img.join("\n").parse()?;

        Ok(Tile { id, img })
    }
}

fn match_tiles(sz: i32, p: Pair, cur: &mut HashMap<Pair, Tile>, rem: &mut VecDeque<Tile>) -> bool {
    if rem.is_empty() {
        return true;
    }

    for _ in 0..rem.len() {
        let mut tile = rem.pop_front().unwrap();

        let above = cur.get(&p.up()).map(|above| above.img.key(Edge::Bottom));
        let left = cur.get(&p.left()).map(|left| left.img.key(Edge::Right));

        for trans in TRANS {
            match trans {
                Some(Trans::Rot) => tile.img.rotate(),
                Some(Trans::Flip) => tile.img.flip(),
                None => (),
            }

            if let Some(false) = above.map(|k| k == tile.img.key(Edge::Top)) {
                continue;
            }

            if let Some(false) = left.map(|k| k == tile.img.key(Edge::Left)) {
                continue;
            }

            cur.insert(p, tile);
            let pp = Pair((p.0 + 1) % sz, if p.0 == sz - 1 { p.1 + 1 } else { p.1 });
            if match_tiles(sz, pp, cur, rem) {
                return true;
            }
            tile = cur.remove(&p).unwrap();
        }

        rem.push_back(tile);
    }

    false
}

fn solve(input: &str) -> crate::Result<HashMap<Pair, Tile>> {
    let mut tiles: Vec<Tile> = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let sz = (tiles.len() as f32).sqrt() as i32;

    for t in &mut tiles {
        t.img.populate_cache();
    }

    let mut res = HashMap::new();
    if !match_tiles(sz, Pair(0, 0), &mut res, &mut tiles.into()) {
        return Err(crate::Error::boxed(Error::InvalidInput));
    }

    Ok(res)
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let tiles = solve(input)?;
    let sz = (tiles.len() as f32).sqrt() as i32;
    let p = [(0, 0), (0, sz - 1), (sz - 1, 0), (sz - 1, sz - 1)]
        .iter()
        .map(|p| tiles.get(&Pair::from(*p)).unwrap().id as i64)
        .product();
    Ok(p)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let mut tiles = solve(input)?;

    for tile in tiles.values_mut() {
        tile.img.apply_orientation();
        tile.img.crop();
    }

    let mut img = {
        let tile_sz = tiles.values().next().unwrap().img.sz.0;
        let total_sz = (tiles.len() as f32).sqrt() as i32 * tile_sz;

        let mut pixels = HashMap::new();
        for p in (0..total_sz)
            .flat_map(|y| iter::repeat(y).zip(0..total_sz))
            .map(Pair::from)
        {
            let tile_pos = Pair(p.0 / tile_sz, p.1 / tile_sz);
            let tile_pixel = Pair(p.0 % tile_sz, p.1 % tile_sz);

            let tile = tiles.get(&tile_pos).unwrap();
            pixels.insert(p, *tile.img.pixels.get(&tile_pixel).unwrap());
        }

        Image::new(Pair(total_sz, total_sz), pixels)
    };

    let monster: Vec<Pair> = include_str!("../../input/day20_monster")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
        .filter_map(|(y, (x, c))| {
            if c == '#' {
                Some(Pair(x as i32, y as i32))
            } else {
                None
            }
        })
        .collect();

    for trans in TRANS {
        match trans {
            Some(Trans::Rot) => img.rotate_pixels(),
            Some(Trans::Flip) => img.flip_pixels(),
            None => (),
        }

        for p in img.coords() {
            if img.matches_pattern(p, &monster) {
                img.apply_pattern(p, &monster);
            }
        }
    }

    let safe_cnt = img
        .pixels
        .values()
        .filter(|(c, b)| *c == '#' && !(*b))
        .count();

    Ok(safe_cnt as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_point() {
        let sz = Pair(4, 4);
        let mut p = Pair(1, 0);
        p = p.rotate(sz);
        assert_eq!(p, Pair(3, 1));
        p = p.rotate(sz);
        assert_eq!(p, Pair(2, 3));
        p = p.rotate(sz);
        assert_eq!(p, Pair(0, 2));
        p = p.rotate(sz);
        assert_eq!(p, Pair(1, 0));
    }

    #[test]
    fn crop_image() {
        let s = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let s_crop = "#..#....
...##..#
###.#...
#.##.###
#...#.##
#.#.#..#
.#....#.
##...#.#";

        let mut img: Image = s.parse().unwrap();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
        img.crop();
        assert_eq!(&img.to_string(), &(s_crop.to_string() + "\n"));
    }

    #[test]
    fn rotate_image() {
        let s = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let s_rot = ".#..#####.
.#.####.#.
###...#..#
#..#.##..#
#....#.##.
...##.##.#
.#...#....
#.#.##....
##.###.#.#
#..##.#...";

        let mut img: Image = s.parse().unwrap();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
        img.rotate_pixels();
        assert_eq!(&img.to_string(), &(s_rot.to_string() + "\n"));
        img.rotate_pixels();
        img.rotate_pixels();
        img.rotate_pixels();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
    }

    #[test]
    fn rotate_image_orientation() {
        let s = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let s_rot = ".#..#####.
.#.####.#.
###...#..#
#..#.##..#
#....#.##.
...##.##.#
.#...#....
#.#.##....
##.###.#.#
#..##.#...";

        let mut img: Image = s.parse().unwrap();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
        img.rotate();
        img.apply_orientation();
        assert_eq!(&img.to_string(), &(s_rot.to_string() + "\n"));
        img.rotate();
        img.rotate();
        img.rotate();
        img.apply_orientation();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
    }

    #[test]
    fn flip_image() {
        let s = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let s_flip = ".#..#.##..
.....#..##
.#..##...#
#...#.####
.###.##.##
###.#...##
##..#.#.#.
..#....#..
.#.#...###
###..###..";

        let mut img: Image = s.parse().unwrap();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
        img.flip_pixels();
        assert_eq!(&img.to_string(), &(s_flip.to_string() + "\n"));
        img.flip_pixels();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
    }

    #[test]
    fn flip_image_orientation() {
        let s = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let s_flip = ".#..#.##..
.....#..##
.#..##...#
#...#.####
.###.##.##
###.#...##
##..#.#.#.
..#....#..
.#.#...###
###..###..";

        let mut img: Image = s.parse().unwrap();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
        img.flip();
        img.apply_orientation();
        assert_eq!(&img.to_string(), &(s_flip.to_string() + "\n"));
        img.flip();
        img.apply_orientation();
        assert_eq!(&img.to_string(), &(s.to_string() + "\n"));
    }

    #[test]
    fn edge_keys() {
        let s = "..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let img: Image = s.parse().unwrap();
        assert_eq!(img.calc_key(Edge::Top), 210);
        assert_eq!(img.calc_key(Edge::Right), 89);
        assert_eq!(img.calc_key(Edge::Bottom), 231);
        assert_eq!(img.calc_key(Edge::Left), 498);
    }

    #[test]
    fn p1ex1() {
        let inp = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(part1(inp).unwrap(), 20899048083289);
        assert_eq!(part2(inp).unwrap(), 273);
    }
}
