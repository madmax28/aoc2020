use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn is_valid_p1(&self) -> bool {
        !self.byr.is_empty()
            && !self.iyr.is_empty()
            && !self.eyr.is_empty()
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }

    fn is_valid_p2(&self) -> bool {
        let in_range = |v: &str, min, max| {
            matches!(
                v.parse::<i32>().map(|i| i >= min && i <= max),
                Ok(true)
            )
        };

        if !in_range(&self.byr, 1920, 2002) {
            return false;
        }

        if !in_range(&self.iyr, 2010, 2020) {
            return false;
        }

        if !in_range(&self.eyr, 2020, 2030) {
            return false;
        }

        if let Some((i, _)) = self.hgt.chars().enumerate().find(|(_, c)| !c.is_numeric()) {
            if !match &self.hgt[i..] {
                "cm" => in_range(&self.hgt[..i], 150, 193),
                "in" => in_range(&self.hgt[..i], 59, 76),
                _ => false,
            } {
                return false;
            }
        } else {
            return false;
        }

        {
            if self.hcl.len() != 7 {
                return false;
            }

            let mut chars = self.hcl.chars();
            if chars.next() != Some('#') {
                return false;
            }

            if !chars.all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
                return false;
            }
        }

        const ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !ECLS.contains(&self.ecl.as_str()) {
            return false;
        }

        if self.pid.len() != 9 || !self.pid.chars().all(|c| c.is_numeric()) {
            return false;
        }

        true
    }
}

struct Collection(Vec<Passport>);

impl FromStr for Collection {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ps = Vec::new();
        let mut p = Passport::default();
        for token in s.split(|c: char| c.is_whitespace()) {
            if token == "" {
                ps.push(p);
                p = Passport::default();
                continue;
            }

            let kv = token.split(':').collect::<Vec<_>>();
            if kv.len() != 2 {
                return Err(crate::Error::boxed(Error::InvalidInput));
            }

            match (kv[0], kv[1]) {
                ("byr", v) => p.byr = v.to_string(),
                ("iyr", v) => p.iyr = v.to_string(),
                ("eyr", v) => p.eyr = v.to_string(),
                ("hgt", v) => p.hgt = v.to_string(),
                ("hcl", v) => p.hcl = v.to_string(),
                ("ecl", v) => p.ecl = v.to_string(),
                ("pid", v) => p.pid = v.to_string(),
                ("cid", v) => p.cid = v.to_string(),
                _ => return Err(crate::Error::boxed(Error::InvalidInput)),
            }
        }
        ps.push(p);
        Ok(Collection(ps))
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let ps = Collection::from_str(input)?;
    Ok(ps.0.iter().filter(|p| p.is_valid_p1()).count() as i32)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let ps = Collection::from_str(input)?;
    Ok(ps.0.iter().filter(|p| p.is_valid_p2()).count() as i32)
}
