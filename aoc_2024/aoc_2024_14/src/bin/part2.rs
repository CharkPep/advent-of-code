use std::{collections::HashMap, vec};

use regex::Regex;

const N: isize = 103;
const M: isize = 101;
const I: isize = 100;

fn p(t: isize, p: (isize, isize), v: (isize, isize)) -> (isize, isize) {
    (
        isize::rem_euclid(t * v.0 + p.0, M),
        isize::rem_euclid(t * v.1 + p.1, N),
    )
}

fn g(v: Vec<(usize, usize)>) -> String {
    let mut p = vec![vec!["."; M as usize]; N as usize];
    for v in v.iter() {
        p[v.1][v.0] = "*";
    }

    p.into_iter()
        .map(|l| l.join(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let r = Regex::new(r"(-?\d+),(-?\d+)").unwrap();
    let mut input = Vec::new();
    r.captures_iter(include_str!("../../input.txt"))
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
        .collect::<Vec<_>>()
        .chunks(2)
        .for_each(|chunk| {
            let (p, v) = (chunk[0], chunk[1]);
            input.push((p, v));
        });

    // Just try find bottom of the tree, there probably only one such sequence and that is tree
    let r = Regex::new(r"\*{15,}").unwrap();
    let mut i = 1;
    loop {
        let pos = input
            .clone()
            .into_iter()
            .map(|v| {
                let v = p(i, v.0, v.1);
                (v.0 as usize, v.1 as usize)
            })
            .collect::<Vec<_>>();
        let s = &g(pos);
        if r.is_match(s) {
            println!("{i} {s}");
            break;
        }
        i += 1;
    }
}
