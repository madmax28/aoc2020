use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Turn(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s
            .chars()
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
        let n = s[1..].parse()?;

        match c {
            'N' => Ok(Instruction::North(n)),
            'S' => Ok(Instruction::South(n)),
            'E' => Ok(Instruction::East(n)),
            'W' => Ok(Instruction::West(n)),
            'L' => Ok(Instruction::Turn(((-n / 90) + 4) % 4)),
            'R' => Ok(Instruction::Turn(n / 90)),
            'F' => Ok(Instruction::Forward(n)),
            _ => Err(crate::Error::boxed(Error::InvalidInput)),
        }
    }
}

#[derive(Debug)]
enum Facing {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct ShipP1 {
    pos: (i32, i32),
    facing: Facing,
}

impl ShipP1 {
    fn new() -> Self {
        ShipP1 {
            pos: (0, 0),
            facing: Facing::East,
        }
    }

    fn turn(&mut self, n: i32) {
        for _ in 0..n {
            self.facing = match self.facing {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North,
            }
        }
    }

    fn mv(&mut self, n: i32) {
        match self.facing {
            Facing::North => self.pos.1 -= n,
            Facing::South => self.pos.1 += n,
            Facing::East => self.pos.0 += n,
            Facing::West => self.pos.0 -= n,
        }
    }

    fn instruct(&mut self, insns: &[Instruction]) {
        for i in insns {
            match i {
                Instruction::North(n) => self.pos.1 -= n,
                Instruction::South(n) => self.pos.1 += n,
                Instruction::East(n) => self.pos.0 += n,
                Instruction::West(n) => self.pos.0 -= n,
                Instruction::Turn(n) => self.turn(*n),
                Instruction::Forward(n) => self.mv(*n),
            }
        }
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let insns: Vec<_> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    let mut ship = ShipP1::new();
    ship.instruct(&insns);
    Ok(ship.pos.0.abs() + ship.pos.1.abs())
}

#[derive(Debug)]
struct ShipP2 {
    pos: (i32, i32),
    wp: (i32, i32),
}

impl ShipP2 {
    fn new() -> Self {
        ShipP2 {
            pos: (0, 0),
            wp: (10, -1),
        }
    }

    fn turn(&mut self, n: i32) {
        for _ in 0..n {
            let tmp = self.wp.1;
            self.wp.1 = self.wp.0;
            self.wp.0 = -tmp;
        }
    }

    fn mv(&mut self, n: i32) {
        self.pos.0 += self.wp.0 * n;
        self.pos.1 += self.wp.1 * n;
    }

    fn instruct(&mut self, insns: &[Instruction]) {
        for i in insns {
            match i {
                Instruction::North(n) => self.wp.1 -= n,
                Instruction::South(n) => self.wp.1 += n,
                Instruction::East(n) => self.wp.0 += n,
                Instruction::West(n) => self.wp.0 -= n,
                Instruction::Turn(n) => self.turn(*n),
                Instruction::Forward(n) => self.mv(*n),
            }
        }
    }
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let insns: Vec<_> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    let mut ship = ShipP2::new();
    ship.instruct(&insns);
    Ok(ship.pos.0.abs() + ship.pos.1.abs())
}
