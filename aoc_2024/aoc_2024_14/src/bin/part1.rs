use std::collections::HashMap;

use regex::Regex;

const N: isize = 103;
const M: isize = 101;
// const N: isize = 7;
// const M: isize = 11;
const I: isize = 100;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
    None,
}

fn main() {
    let r = Regex::new(r"(-?\d+),(-?\d+)").unwrap();
    let mut a = vec![vec![0; M as usize]; N as usize];
    let ans = r
        .captures_iter(include_str!("../../input.txt"))
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let (p, v) = (chunk[0], chunk[1]);
            let p = (
                isize::rem_euclid(I * v.0 + p.0, M),
                isize::rem_euclid(I * v.1 + p.1, N),
            );

            a[p.1 as usize][p.0 as usize] += 1;
            if p.0 < M / 2 && p.1 < N / 2 {
                Quadrant::Q1
            } else if p.0 > M / 2 && p.1 < N / 2 {
                Quadrant::Q2
            } else if p.0 < M / 2 && p.1 > N / 2 {
                Quadrant::Q3
            } else if p.0 > M / 2 && p.1 > N / 2 {
                Quadrant::Q4
            } else {
                Quadrant::None
            }
        })
        .fold(HashMap::new(), |mut acc, q| {
            acc.entry(q).and_modify(|v| *v += 1).or_insert(1);
            acc
        })
        .into_iter()
        .filter_map(|(k, v)| {
            if let Quadrant::None = k {
                return None;
            } else {
                return Some(v);
            }
        })
        .product::<usize>();
    println!("{ans}");
}
