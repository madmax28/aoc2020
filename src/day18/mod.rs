#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn eval_p1(vals: Vec<i64>, ops: Vec<Op>) -> i64 {
    let mut val = vals[0];
    for (o, v) in ops.into_iter().zip(vals.into_iter().skip(1)) {
        val = o.apply(val, v);
    }
    val
}

fn eval_p2(mut vals: Vec<i64>, mut ops: Vec<Op>) -> i64 {
    for op in &[Op::Add, Op::Mul] {
        let mut i = 0;
        while i < ops.len() {
            let (o, a, b) = (ops[i], vals[i], vals[i + 1]);

            if o == *op {
                ops.remove(i);
                vals.remove(i);
                vals[i] = o.apply(a, b);
            } else {
                i += 1;
            }
        }
    }
    vals[0]
}

fn solve<T, F>(it: &mut T, eval: F) -> i64
where
    T: Iterator<Item = char>,
    F: Copy + FnOnce(Vec<i64>, Vec<Op>) -> i64,
{
    let mut vals = Vec::new();
    let mut ops = Vec::new();

    while let Some(c) = it.next() {
        match c {
            '0'..='9' => vals.push(c.to_digit(10).unwrap() as i64),
            '*' => ops.push(Op::Mul),
            '+' => ops.push(Op::Add),
            '(' => vals.push(solve(it, eval)),
            ')' => break,
            _ => (),
        }
    }

    eval(vals, ops)
}

pub fn part1(input: &str) -> crate::Result<i64> {
    Ok(input.lines().map(|l| solve(&mut l.chars(), eval_p1)).sum())
}

pub fn part2(input: &str) -> crate::Result<i64> {
    Ok(input.lines().map(|l| solve(&mut l.chars(), eval_p2)).sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn p2ex1() {
        let expr = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(super::solve(&mut expr.chars(), super::eval_p2), 51);
    }

    #[test]
    fn p2ex2() {
        let expr = "2 * 3 + (4 * 5)";
        assert_eq!(super::solve(&mut expr.chars(), super::eval_p2), 46);
    }
}
