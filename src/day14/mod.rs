use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
enum Instruction {
    Mask((i64, i64)),
    Store((i64, i64)),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(" = ").collect();
        if tokens.len() != 2 {
            return Err(crate::Error::boxed(Error::InvalidInput));
        }

        if tokens[0] == "mask" {
            let value = i64::from_str_radix(
                &tokens[1]
                    .chars()
                    .map(|c| match c {
                        'X' => '0',
                        c => c,
                    })
                    .collect::<String>(),
                2,
            )?;

            let mask = i64::from_str_radix(
                &tokens[1]
                    .chars()
                    .map(|c| match c {
                        'X' => '1',
                        _ => '0',
                    })
                    .collect::<String>(),
                2,
            )?;

            Ok(Instruction::Mask((value, mask)))
        } else if tokens[0].starts_with("mem") {
            let address = tokens[0]
                .chars()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()?;
            let value = tokens[1].parse()?;

            Ok(Instruction::Store((address, value)))
        } else {
            Err(crate::Error::boxed(Error::InvalidInput))
        }
    }
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let insns: Vec<Instruction> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    let mut mem = HashMap::new();

    let mut mask = (0, 0);
    for i in insns {
        match i {
            Instruction::Mask(m) => mask = m,
            Instruction::Store((a, v)) => {
                let v = mask.0 | (mask.1 & v);
                mem.insert(a, v);
            }
        }
    }

    Ok(mem.values().sum())
}

fn float(v: i64, xs: i64) -> Vec<i64> {
    let mut vs = vec![v];
    for b in 0..36 {
        if xs >> b & 1 == 0 {
            continue;
        }

        let mut tmp = Vec::new();
        for v in &vs {
            tmp.push(v ^ (1 << b));
        }
        vs.extend(tmp);
    }
    vs
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let insns: Vec<Instruction> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    let mut mem = HashMap::new();

    let mut mask = (0, 0);
    for i in insns {
        match i {
            Instruction::Mask(m) => mask = m,
            Instruction::Store((a, v)) => {
                for a in float(mask.0 | a, mask.1) {
                    mem.insert(a, v);
                }
            }
        }
    }

    Ok(mem.values().sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn ex1() {
        let prog = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(super::part2(prog).unwrap(), 208);
    }
}
