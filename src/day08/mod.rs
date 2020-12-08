use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
enum Res {
    Loop(i32),
    Halt(i32),
}

fn run(program: &[(&str, i32)]) -> crate::Result<Res> {
    let mut acc = 0;
    let mut ip = 0;
    let mut visited = HashSet::new();
    loop {
        if ip == program.len() as i32 {
            return Ok(Res::Halt(acc));
        } else if !visited.insert(ip) {
            return Ok(Res::Loop(acc));
        }

        match program
            .get(ip as usize)
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        {
            ("jmp", n) => ip += n,
            ("acc", n) => {
                acc += n;
                ip += 1;
            }
            ("nop", _) => ip += 1,
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        }
    }
}

fn parse(s: &str) -> crate::Result<Vec<(&str, i32)>> {
    let prog = s
        .lines()
        .map(|l| -> crate::Result<_> {
            let insn = l.trim_end_matches(|c: char| {
                c.is_digit(10) || c == '-' || c == '+' || c.is_whitespace()
            });
            let num = l
                .split(' ')
                .nth(1)
                .and_then(|w| w.parse().ok())
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
            Ok((insn, num))
        })
        .collect::<Result<_, _>>()?;
    Ok(prog)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let program = parse(input)?;

    match run(&program)? {
        Res::Loop(acc) => Ok(acc),
        _ => Err(crate::Error::boxed(Error::InvalidInput)),
    }
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut program = parse(input)?;

    for i in 0..program.len() {
        program[i] = match program[i] {
            ("jmp", n) => ("nop", n),
            ("nop", n) => ("jmp", n),
            _ => continue,
        };

        if let Res::Halt(acc) = run(&program)? {
            return Ok(acc);
        }

        program[i] = match program[i] {
            ("jmp", n) => ("nop", n),
            ("nop", n) => ("jmp", n),
            _ => unreachable!(),
        };
    }

    Err(crate::Error::boxed(Error::InvalidInput))
}
