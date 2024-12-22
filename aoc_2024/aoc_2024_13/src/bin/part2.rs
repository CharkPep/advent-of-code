use core::f64;

use ndarray_linalg::Solve;
use regex::Regex;

const A_COST: usize = 3;
const B_COST: usize = 1;
const HELL_CONST: usize = 10000000000000;

#[derive(Debug, Clone)]
struct Input {
    a: ndarray::Array2<f64>,
    b: ndarray::Array1<f64>,
}

fn main() {
    let input = include_str!("../../input.txt");
    let r = Regex::new(r"(\d+).*?(\d+)").unwrap();
    let ans = r
        .captures_iter(input)
        .map(|caps| caps.extract())
        .map(|(_, [x, y])| (x, y))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let a = chunk[0];
            let b = chunk[1];
            let prize = chunk[2];
            Input {
                a: ndarray::arr2(&[
                    [a.0.parse().unwrap(), b.0.parse().unwrap()],
                    [a.1.parse().unwrap(), b.1.parse().unwrap()],
                ]),
                b: ndarray::arr1(&[
                    prize.0.parse::<f64>().unwrap() + HELL_CONST as f64,
                    prize.1.parse::<f64>().unwrap() + HELL_CONST as f64,
                ]),
            }
        })
        .filter_map(|Input { a, b }| {
            if (a[[0, 0]] * a[[1, 1]] - a[[0, 1]] * a[[1, 0]]) == 0.0 {
                return None;
            }

            let det = 1.0 / (a[[0, 0]] * a[[1, 1]] - a[[0, 1]] * a[[1, 0]]);
            let inv = ndarray::arr2(&[[a[[1, 1]], -a[[0, 1]]], [-a[[1, 0]], a[[0, 0]]]]) * det;
            let res = inv.dot(&b);
            let (a, b) = (res[0], res[1]);

            if a < 0.0
                || b < 0.0
                || f64::abs(f64::round(a) - a) > 1e-2
                || f64::abs(f64::round(b) - b) > 1e-2
            {
                return None;
            }

            return Some((f64::round(a), f64::round(b)));
        })
        .map(|c| c.0 as usize * A_COST + c.1 as usize * B_COST)
        .sum::<usize>();

    println!("{}", ans);
}
