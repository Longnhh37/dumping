use std::hint::black_box;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Op {
    Add(i64),
    Mul(i64),
    Xor(i64),
    Rare,
}

fn parse(line: &str) -> Option<Op> {
    let mut it = line.split_whitespace();
    match (it.next()?, it.next()) {
        ("add", Some(n)) => n.parse().ok().map(Op::Add),
        ("mul", Some(n)) => n.parse().ok().map(Op::Mul),
        ("xor", Some(n)) => n.parse().ok().map(Op::Xor),
        ("rare", None) => Some(Op::Rare),
        _ => None,
    }
}

fn step(acc: i64, op: Op) -> i64 {
    match op {
        Op::Add(x) => acc.wrapping_add(x),
        Op::Mul(x) => acc.wrapping_mul(x),
        Op::Xor(x) => acc ^ x,
        Op::Rare => acc.rotate_left(7) ^ 0xdead_beef,
    }
}

fn main() {
    let ops: Vec<Op> = io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parse(&line))
        .collect();

    let mut acc = 0i64;
    for _ in 0..50 {
        for &op in &ops {
            acc = step(acc, black_box(op));
        }
    }

    println!("ops={} acc={acc}", ops.len());
}
