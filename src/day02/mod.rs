use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
struct Entry {
    min: i32,
    max: i32,
    c: char,
    pw: String,
}

impl Entry {
    fn new(min: i32, max: i32, c: char, pw: String) -> Self {
        Entry { min, max, c, pw }
    }

    fn is_valid_p1(&self) -> bool {
        let cnt = self.pw.chars().filter(|&c| c == self.c).count() as i32;
        cnt >= self.min && cnt <= self.max
    }

    fn is_valid_p2(&self) -> bool {
        let c_at_pos = |s: &str, n, c| s.chars().nth(n as usize).map(|cc| cc == c).unwrap_or(false);

        c_at_pos(&self.pw, self.min - 1, self.c) != c_at_pos(&self.pw, self.max - 1, self.c)
    }
}

impl FromStr for Entry {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let min = chars
            .by_ref()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()?;
        let max = chars
            .by_ref()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()?;
        let c = chars
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
        let pw = chars.skip(1).collect::<String>().trim().to_string();

        Ok(Entry::new(min, max, c, pw))
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let es = input
        .lines()
        .map(Entry::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(es.iter().filter(|e| e.is_valid_p1()).count() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let es = input
        .lines()
        .map(Entry::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(es.iter().filter(|e| e.is_valid_p2()).count() as i32)
}
