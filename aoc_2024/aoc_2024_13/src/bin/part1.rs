use std::{ops::Add, sync::LazyLock};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

const A_COST: usize = 3;
const B_COST: usize = 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct I64Vector2(i64, i64);

impl Add for I64Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        I64Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Input {
    action_a: I64Vector2,
    action_b: I64Vector2,
    prize: I64Vector2,
}

static INPUT: std::sync::LazyLock<Vec<Input>> = LazyLock::new(|| {
    let input = include_str!("../../input.txt");
    let r = Regex::new(r"(\d+).*?(\d+)").unwrap();
    r.captures_iter(input)
        .map(|caps| caps.extract())
        .map(|(_, [x, y])| (x, y))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let a = chunk[0];
            let b = chunk[1];
            let prize = chunk[2];
            Input {
                action_a: I64Vector2(a.0.parse().unwrap(), a.1.parse().unwrap()),
                action_b: I64Vector2(b.0.parse().unwrap(), b.1.parse().unwrap()),
                prize: I64Vector2(prize.0.parse().unwrap(), prize.1.parse().unwrap()),
            }
        })
        .collect()
});

fn resolve(dp: &mut Vec<Vec<I64Vector2>>) {
    let a = dp[0][1];
    let b = dp[1][0];
    for i in 2..dp.len() {
        dp[i][0] = dp[i - 1][0] + b;
    }

    for j in 2..dp[0].len() {
        dp[0][j] = dp[0][j - 1] + a;
    }

    for i in 1..dp.len() {
        for j in 1..dp[i].len() {
            dp[i][j] = dp[i][j - 1] + a;
        }
    }
}

fn find_min(dp: &Vec<Vec<I64Vector2>>, goal: I64Vector2) -> Option<usize> {
    let mut mn = None;
    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            if dp[i][j] == goal && i * B_COST + j * A_COST < mn.unwrap_or(1e9 as usize) {
                mn = Some(i * B_COST + j * A_COST as usize);
            }
        }
    }

    mn
}

fn main() {
    let ans = INPUT
        .iter()
        .map(|input| {
            let (n, m) = (
                usize::min(
                    usize::max(
                        (input.prize.0 / input.action_b.0) as usize + 1,
                        (input.prize.1 / input.action_b.1) as usize + 1,
                    ),
                    100,
                ),
                usize::min(
                    usize::max(
                        (input.prize.0 / input.action_a.0) as usize + 1,
                        (input.prize.1 / input.action_a.1) as usize + 1,
                    ),
                    100,
                ),
            );

            move || {
                let mut dp = vec![vec![I64Vector2(0, 0); m]; n];
                dp[0][1] = input.action_a;
                dp[1][0] = input.action_b;
                resolve(&mut dp);
                find_min(&dp, input.prize)
            }
        })
        .collect::<Vec<_>>()
        .par_iter()
        .map(|f| f())
        .filter_map(|c| c)
        .sum::<usize>();

    println!("{}", ans);
}
