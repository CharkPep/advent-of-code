use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

const EMPTY: char = '.';

lazy_static! {
    static ref INPUT: Vec<Vec<char>> = {
        include_str!("../../input.txt")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let mut r = line.trim().chars().collect::<Vec<char>>();
                r.insert(0, '.');
                r.push('.');
                r
            })
            .collect::<Vec<_>>()
    };
}

fn bfs(g: &mut Vec<Vec<char>>, start: (usize, usize)) -> (usize, usize) {
    let mut q = VecDeque::new();
    let region_sig = g[start.0][start.1];
    let mut area = 0;
    let mut perimiter = 0;
    let mut region = HashMap::new();
    region.insert(start, true);
    q.push_front(start);
    while let Some(v) = q.pop_front() {
        area += 1;
        let idxs = [
            (v.0 + 1, v.1),
            (v.0 - 1, v.1),
            (v.0, v.1 + 1),
            (v.0, v.1 - 1),
        ];
        for idx in idxs {
            let visited = region.contains_key(&idx);
            if g[idx.0][idx.1] == region_sig && !visited {
                region.entry(idx).or_insert(true);
                g[idx.0][idx.1] = EMPTY;
                q.push_back(idx);
            } else if !visited {
                perimiter += 1;
            }
        }
    }

    (area, perimiter)
}

fn main() {
    let mut garden = (*INPUT).clone();
    garden.insert(0, (0..INPUT.len() + 2).map(|_| EMPTY).collect());
    garden.push((0..INPUT.len() + 2).map(|_| EMPTY).collect());
    let mut ans = 0;
    for i in 1..garden.len() - 1 {
        for j in 1..garden[i].len() - 1 {
            if garden[i][j] != EMPTY {
                let (area, perimeter) = bfs(&mut garden, (i, j));
                ans += area * perimeter;
            }
        }
    }

    println!("{ans}");
}
