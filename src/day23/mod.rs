use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
struct Node {
    left: usize,
    right: usize,
}

impl Node {
    fn new(left: usize, right: usize) -> Self {
        Node { left, right }
    }
}

#[derive(Debug)]
struct CircularList<T> {
    nodes: Vec<Node>,
    storage: Vec<T>,
    idx_map: HashMap<T, usize>,
}

impl<T> CircularList<T>
where
    T: fmt::Debug + PartialEq + Eq + Hash,
{
    fn find_idx(&self, v: &T) -> usize {
        *self.idx_map.get(v).unwrap()
    }

    fn right(&self, idx: usize) -> usize {
        self.nodes[idx].right
    }

    fn remove(&mut self, idx: usize) {
        let left = self.nodes[idx].left;
        let right = self.nodes[idx].right;

        self.nodes[left].right = right;
        self.nodes[right].left = left;
    }

    fn insert(&mut self, idx: usize, at: usize) {
        let left = self.nodes[at].left;
        let right = at;

        self.nodes[idx].left = left;
        self.nodes[idx].right = right;

        self.nodes[left].right = idx;
        self.nodes[right].left = idx;
    }

    fn get(&self, idx: usize) -> &T {
        &self.storage[idx]
    }
}

impl<T> From<Vec<T>> for CircularList<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    fn from(values: Vec<T>) -> Self {
        assert!(!values.is_empty());

        let idx_map = values
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        let mut nodes: Vec<Node> = Vec::new();
        for idx in 0..values.len() {
            let left = (idx + values.len() - 1) % values.len();
            let right = (idx + 1) % values.len();

            let node = Node::new(left, right);
            nodes.push(node);
        }

        CircularList {
            nodes,
            storage: values,
            idx_map,
        }
    }
}

#[derive(Debug)]
struct Game {
    nums: CircularList<i32>,
    len: usize,
    cur_idx: usize,
}

impl Game {
    fn new(nums: Vec<i32>) -> crate::Result<Self> {
        if nums.is_empty() {
            Err(crate::Error::boxed(Error::InvalidInput))
        } else {
            let len = nums.len();
            Ok(Game {
                nums: nums.into(),
                len,
                cur_idx: 0,
            })
        }
    }

    fn step(&mut self) {
        let mut tmp = Vec::new();
        {
            let mut idx = self.cur_idx;
            let mut right = self.nums.right(idx);
            for _ in 0..3 {
                idx = right;
                right = self.nums.right(idx);
                self.nums.remove(idx);
                tmp.push(idx);
            }
        }

        let dst = {
            let removed: Vec<i32> = tmp.iter().map(|idx| *self.nums.get(*idx)).collect();

            let cur = *self.nums.get(self.cur_idx);
            let mut dst = (cur - 2).rem_euclid(self.len as i32) + 1;
            while removed.contains(&dst) {
                dst = (dst - 2).rem_euclid(self.len as i32) + 1;
            }
            dst
        };

        let mut dst_idx = {
            let mut dst_idx = self.nums.find_idx(&dst);
            dst_idx = self.nums.right(dst_idx);
            dst_idx
        };

        for _ in 0..3 {
            let idx = tmp.pop().unwrap();
            self.nums.insert(idx, dst_idx);
            dst_idx = idx;
        }

        self.cur_idx = self.nums.right(self.cur_idx);
    }

    fn calc_p1(&self) -> String {
        let mut res = String::new();
        let mut idx = self.nums.find_idx(&1);

        idx = self.nums.right(idx);
        for _ in 0..self.len - 1 {
            res += &self.nums.get(idx).to_string();
            idx = self.nums.right(idx);
        }

        res
    }

    fn calc_p2(&self) -> i64 {
        let mut idx = self.nums.find_idx(&1);

        idx = self.nums.right(idx);
        let n1 = *self.nums.get(idx);
        idx = self.nums.right(idx);
        let n2 = *self.nums.get(idx);

        n1 as i64 * n2 as i64
    }
}

#[derive(Debug)]
struct Nums(Vec<i32>);

impl FromStr for Nums {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<i32> = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|n| n as i32)
                    .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))
            })
            .collect::<Result<_, _>>()?;
        Ok(Nums(nums))
    }
}

pub fn part1(input: &str) -> crate::Result<String> {
    let mut game: Game = Game::new(input.parse::<Nums>()?.0)?;
    for _ in 0..100 {
        game.step();
    }
    Ok(game.calc_p1())
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let mut nums = input.parse::<Nums>()?.0;
    nums.extend(nums.len() as i32 + 1..=1_000_000);
    let mut game: Game = Game::new(nums)?;
    for _ in 0..10_000_000 {
        game.step();
    }
    Ok(game.calc_p2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1ex1() {
        let inp = "389125467";
        assert_eq!(&part1(inp).unwrap(), "67384529")
    }
}
