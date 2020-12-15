mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

use std::{env, error, fmt, fs, result};

#[derive(Debug)]
struct UsageError;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct Error<T> {
    err: T,
}

impl<T> Error<T> {
    fn boxed(err: T) -> Box<Self> {
        Box::new(Self { err })
    }
}

impl<T: fmt::Debug> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.err)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}

fn time<F: Fn(A) -> B, A, B>(f: F, a: A) -> B {
    let now = std::time::Instant::now();
    let res = f(a);
    let d = now.elapsed();
    println!(
        "> {}.{:03} {:03} {:03} seconds",
        d.as_secs(),
        d.subsec_millis(),
        d.subsec_micros() % 1_000,
        d.subsec_nanos() % 1_000,
    );
    res
}

fn usage() -> Result<()> {
    eprintln!("usage: aoc2020 <day> [<input>]");
    Err(Error::boxed(UsageError {}))
}

fn main() -> Result<()> {
    let (day, input) = {
        let mut args = env::args().skip(1);
        let d = if let Some(d) = args.next() {
            if let Ok(d) = d.parse() {
                d
            } else {
                eprintln!("Could not parse day: '{}'", d);
                return usage();
            }
        } else {
            eprintln!("Not enough arguments");
            return usage();
        };

        let i = args.next().unwrap_or_else(|| format!("input/day{:02}", d));
        let i = if let Ok(i) = fs::read_to_string(&i) {
            i
        } else {
            eprintln!("No such file: '{}'", &i);
            return usage();
        };

        (d, i)
    };

    match day {
        1 => {
            println!("Part 1: {}", time(day01::part1, input.trim())?);
            println!("Part 2: {}", time(day01::part2, input.trim())?);
        }
        2 => {
            println!("Part 1: {}", time(day02::part1, input.trim())?);
            println!("Part 2: {}", time(day02::part2, input.trim())?);
        }
        3 => {
            println!("Part 1: {}", time(day03::part1, input.trim())?);
            println!("Part 2: {}", time(day03::part2, input.trim())?);
        }
        4 => {
            println!("Part 1: {}", time(day04::part1, input.trim())?);
            println!("Part 2: {}", time(day04::part2, input.trim())?);
        }
        5 => {
            println!("Part 1: {}", time(day05::part1, input.trim())?);
            println!("Part 2: {}", time(day05::part2, input.trim())?);
        }
        6 => {
            println!("Part 1: {}", time(day06::part1, input.trim())?);
            println!("Part 2: {}", time(day06::part2, input.trim())?);
        }
        7 => {
            println!("Part 1: {}", time(day07::part1, input.trim())?);
            println!("Part 2: {}", time(day07::part2, input.trim())?);
        }
        8 => {
            println!("Part 1: {}", time(day08::part1, input.trim())?);
            println!("Part 2: {}", time(day08::part2, input.trim())?);
        }
        9 => {
            println!("Part 1: {}", time(day09::part1, input.trim())?);
            println!("Part 2: {}", time(day09::part2, input.trim())?);
        }
        10 => {
            println!("Part 1: {}", time(day10::part1, input.trim())?);
            println!("Part 2: {}", time(day10::part2, input.trim())?);
        }
        11 => {
            println!("Part 1: {}", time(day11::part1, input.trim())?);
            println!("Part 2: {}", time(day11::part2, input.trim())?);
        }
        12 => {
            println!("Part 1: {}", time(day12::part1, input.trim())?);
            println!("Part 2: {}", time(day12::part2, input.trim())?);
        }
        13 => {
            println!("Part 1: {}", time(day13::part1, input.trim())?);
            println!("Part 2: {}", time(day13::part2, input.trim())?);
        }
        14 => {
            println!("Part 1: {}", time(day14::part1, input.trim())?);
            println!("Part 2: {}", time(day14::part2, input.trim())?);
        }
        15 => {
            println!("Part 1: {}", time(day15::part1, input.trim())?);
            println!("Part 2: {}", time(day15::part2, input.trim())?);
        }
        _ => unimplemented!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day01p1() {
        let inp = include_str!("../input/day01");
        assert_eq!(crate::day01::part1(inp.trim()).unwrap(), 876459);
    }

    #[test]
    fn day01p2() {
        let inp = include_str!("../input/day01");
        assert_eq!(crate::day01::part2(inp.trim()).unwrap(), 116168640);
    }

    #[test]
    fn day02p1() {
        let inp = include_str!("../input/day02");
        assert_eq!(crate::day02::part1(inp.trim()).unwrap(), 614);
    }

    #[test]
    fn day02p2() {
        let inp = include_str!("../input/day02");
        assert_eq!(crate::day02::part2(inp.trim()).unwrap(), 354);
    }

    #[test]
    fn day03p1() {
        let inp = include_str!("../input/day03");
        assert_eq!(crate::day03::part1(inp.trim()).unwrap(), 173);
    }

    #[test]
    fn day03p2() {
        let inp = include_str!("../input/day03");
        assert_eq!(crate::day03::part2(inp.trim()).unwrap(), 4385176320);
    }

    #[test]
    fn day04p1() {
        let inp = include_str!("../input/day04");
        assert_eq!(crate::day04::part1(inp.trim()).unwrap(), 239);
    }

    #[test]
    fn day04p2() {
        let inp = include_str!("../input/day04");
        assert_eq!(crate::day04::part2(inp.trim()).unwrap(), 188);
    }

    #[test]
    fn day05p1() {
        let inp = include_str!("../input/day05");
        assert_eq!(crate::day05::part1(inp.trim()).unwrap(), 926);
    }

    #[test]
    fn day05p2() {
        let inp = include_str!("../input/day05");
        assert_eq!(crate::day05::part2(inp.trim()).unwrap(), 657);
    }

    #[test]
    fn day06p1() {
        let inp = include_str!("../input/day06");
        assert_eq!(crate::day06::part1(inp.trim()).unwrap(), 6335);
    }

    #[test]
    fn day06p2() {
        let inp = include_str!("../input/day06");
        assert_eq!(crate::day06::part2(inp.trim()).unwrap(), 3392);
    }

    #[test]
    fn day07p1() {
        let inp = include_str!("../input/day07");
        assert_eq!(crate::day07::part1(inp.trim()).unwrap(), 242);
    }

    #[test]
    fn day07p2() {
        let inp = include_str!("../input/day07");
        assert_eq!(crate::day07::part2(inp.trim()).unwrap(), 176035);
    }

    #[test]
    fn day08p1() {
        let inp = include_str!("../input/day08");
        assert_eq!(crate::day08::part1(inp.trim()).unwrap(), 1928);
    }

    #[test]
    fn day08p2() {
        let inp = include_str!("../input/day08");
        assert_eq!(crate::day08::part2(inp.trim()).unwrap(), 1319);
    }

    #[test]
    fn day09p1() {
        let inp = include_str!("../input/day09");
        assert_eq!(crate::day09::part1(inp.trim()).unwrap(), 41682220);
    }

    #[test]
    fn day09p2() {
        let inp = include_str!("../input/day09");
        assert_eq!(crate::day09::part2(inp.trim()).unwrap(), 5388976);
    }

    #[test]
    fn day10p1() {
        let inp = include_str!("../input/day10");
        assert_eq!(crate::day10::part1(inp.trim()).unwrap(), 2100);
    }

    #[test]
    fn day10p2() {
        let inp = include_str!("../input/day10");
        assert_eq!(crate::day10::part2(inp.trim()).unwrap(), 16198260678656);
    }

    #[test]
    fn day11p1() {
        let inp = include_str!("../input/day11");
        assert_eq!(crate::day11::part1(inp.trim()).unwrap(), 2424);
    }

    #[test]
    fn day11p2() {
        let inp = include_str!("../input/day11");
        assert_eq!(crate::day11::part2(inp.trim()).unwrap(), 2208);
    }

    #[test]
    fn day12p1() {
        let inp = include_str!("../input/day12");
        assert_eq!(crate::day12::part1(inp.trim()).unwrap(), 938);
    }

    #[test]
    fn day12p2() {
        let inp = include_str!("../input/day12");
        assert_eq!(crate::day12::part2(inp.trim()).unwrap(), 54404);
    }

    #[test]
    fn day13p1() {
        let inp = include_str!("../input/day13");
        assert_eq!(crate::day13::part1(inp.trim()).unwrap(), 3789);
    }

    #[test]
    fn day13p2() {
        let inp = include_str!("../input/day13");
        assert_eq!(crate::day13::part2(inp.trim()).unwrap(), 667437230788118);
    }

    #[test]
    fn day14p1() {
        let inp = include_str!("../input/day14");
        assert_eq!(crate::day14::part1(inp.trim()).unwrap(), 2346881602152);
    }

    #[test]
    fn day14p2() {
        let inp = include_str!("../input/day14");
        assert_eq!(crate::day14::part2(inp.trim()).unwrap(), 3885232834169);
    }

    #[test]
    fn day15p1() {
        let inp = include_str!("../input/day15");
        assert_eq!(crate::day15::part1(inp.trim()).unwrap(), 870);
    }

    #[test]
    fn day15p2() {
        let inp = include_str!("../input/day15");
        assert_eq!(crate::day15::part2(inp.trim()).unwrap(), 9136);
    }
}

// vim macro to prepare new day..
// }kyyp/unimplky3k3jp/daynG{ky2{Pzt7nnnnnn:w
