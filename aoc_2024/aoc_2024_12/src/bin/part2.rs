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

fn check_inner_edge(g: &mut Vec<Vec<char>>, p: (usize, usize)) -> usize {
    let region = g[p.0][p.1];
    let mut edges = 0;
    let dirs = &[
        ((1, 0), (0, -1)),  // SW
        ((1, 0), (0, 1)),   // SE
        ((-1, 0), (0, -1)), // NW
        ((-1, 0), (0, 1)),  // NE
    ];
    for (d1, d2) in dirs.iter() {
        let p1 = (
            (p.0 as isize + d1.0) as usize,
            (p.1 as isize + d1.1) as usize,
        );
        let p2 = (
            (p.0 as isize + d2.0) as usize,
            (p.1 as isize + d2.1) as usize,
        );
        let di = (
            (p.0 as isize + d1.0) as usize,
            (p.1 as isize + d2.1) as usize,
        );

        if g[p1.0][p1.1] == region && g[p2.0][p2.1] == region && g[di.0][di.1] != region {
            edges += 1;
        }
    }

    edges
}

fn check_outer_edge(g: &mut Vec<Vec<char>>, p: (usize, usize)) -> usize {
    let region = g[p.0][p.1];
    let mut edges = 0;
    let dirs = &[
        ((1, 0), (0, -1)),  // SW
        ((1, 0), (0, 1)),   // SE
        ((-1, 0), (0, -1)), // NW
        ((-1, 0), (0, 1)),  // NE
    ];
    for (d1, d2) in dirs.iter() {
        let p1 = (
            (p.0 as isize + d1.0) as usize,
            (p.1 as isize + d1.1) as usize,
        );
        let p2 = (
            (p.0 as isize + d2.0) as usize,
            (p.1 as isize + d2.1) as usize,
        );

        if g[p1.0][p1.1] != region && g[p2.0][p2.1] != region {
            edges += 1;
        }
    }

    edges
}

fn bfs(g: &mut Vec<Vec<char>>, start: (usize, usize)) -> (usize, usize) {
    let mut q = VecDeque::new();
    q.push_back(start);
    let region = g[start.0][start.1];
    let (mut area, mut edges) = (0, 0);
    let dirs: &[(isize, isize)] = &[(0, -1), (0, 1), (1, 0), (-1, 0)];
    // after visiting region to track which region was visited or not it needs to be flooded -
    // Emptied with EMPTY value
    let mut to_flood = HashMap::new();
    to_flood.insert(start, true);
    while let Some(node) = q.pop_front() {
        edges += check_inner_edge(g, node) + check_outer_edge(g, node);
        area += 1;
        for d in dirs.iter() {
            let p = (
                (node.0 as isize + d.0) as usize,
                (node.1 as isize + d.1) as usize,
            );
            if g[p.0][p.1] == region && !to_flood.contains_key(&p) {
                to_flood.insert(p, true);
                q.push_back(p);
            }
        }
    }

    for (k, _) in to_flood {
        g[k.0][k.1] = EMPTY;
    }

    return (area, edges);
}

fn main() {
    let mut garden = (*INPUT).clone();
    garden.insert(0, (0..INPUT.len() + 2).map(|_| EMPTY).collect());
    garden.push((0..INPUT.len() + 2).map(|_| EMPTY).collect());
    let mut ans = 0;
    for i in 1..garden.len() - 1 {
        for j in 1..garden[i].len() - 1 {
            if garden[i][j] != EMPTY {
                let (area, edges) = bfs(&mut garden, (i, j));
                ans += area * edges;
            }
        }
    }

    println!("{ans}");
}
