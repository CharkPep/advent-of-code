use std::{fmt::Debug, ops::Add};

use regex::Regex;

const N: usize = 50;
const M: usize = 50;

const MOVEMENT: &[I64Vector2] = &[
    I64Vector2(0, -1),
    I64Vector2(0, 1),
    I64Vector2(-1, 0),
    I64Vector2(1, 0),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct I64Vector2(i64, i64);

impl Add for I64Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        I64Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum MapElem {
    Edge,
    Box,
    Empty,
}

impl ToString for MapElem {
    fn to_string(&self) -> String {
        match self {
            Self::Empty => ".".to_string(),
            Self::Box => "O".to_string(),
            Self::Edge => "#".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

struct Robot {
    pos: I64Vector2,
    movement: Vec<Direction>,
    idx: usize,
}

impl Robot {
    fn update(&mut self, map: &mut Map) {
        let dir = self.movement[self.idx];
        // Move any boxes in the direction
        map.move_dir(dir, self.pos + MOVEMENT[dir as usize]);
        // Move robot itself
        self.move_dir(self.movement[self.idx], map);
        self.idx += 1;
    }

    fn move_dir(&mut self, dir: Direction, map: &Map) {
        println!("{:?}", dir);
        let m = self.pos + MOVEMENT[dir as usize];
        match map.0[m.0 as usize][m.1 as usize] {
            MapElem::Empty => self.pos = m,
            _ => {}
        }
    }
}

struct Map(Vec<Vec<MapElem>>);

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(N * M);
        for l in self.0.iter() {
            for c in l.iter() {
                s.push_str(&c.to_string());
            }
            s.push('\n');
        }
        s
    }
}

impl Map {
    // Move MapElem on the map if any in the direction following propagation.
    fn move_dir(&mut self, dir: Direction, p: I64Vector2) -> I64Vector2 {
        match self.0[p.0 as usize][p.1 as usize] {
            MapElem::Empty => return p,
            MapElem::Edge => return p,
            _ => {}
        }

        let m = p + MOVEMENT[dir as usize];
        match self.0[m.0 as usize][m.1 as usize] {
            MapElem::Box => {
                let p1 = self.move_dir(dir, m);
                if p1 != m {
                    self.0[m.0 as usize][m.1 as usize] = self.0[p.0 as usize][p.1 as usize];
                    self.0[p.0 as usize][p.1 as usize] = MapElem::Empty;
                    return p1;
                }
                p
            }
            MapElem::Empty => {
                self.0[m.0 as usize][m.1 as usize] = self.0[p.0 as usize][p.1 as usize];
                self.0[p.0 as usize][p.1 as usize] = MapElem::Empty;
                return m;
            }
            _ => return p,
        }
    }
}

struct Game {
    map: Map,
    robot: Robot,
}

impl Game {
    fn update(&mut self) -> &Map {
        self.robot.update(&mut self.map);
        println!("{}", self.map.to_string());
        println!("{:?}", self.robot.pos);
        &self.map
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let r = Regex::new("((?m)^#.*#$)").unwrap();
    let mut mp = vec![vec![MapElem::Empty; M]; N];
    let mut r_pos = I64Vector2(0, 0);
    for (i, l) in r
        .captures_iter(input)
        .into_iter()
        .map(|c| c.extract())
        .map(|(_, [l])| l)
        .enumerate()
    {
        println!("{}, {}", i, l);
        for (j, c) in l.chars().enumerate() {
            mp[i][j] = match c {
                '#' => MapElem::Edge,
                'O' => MapElem::Box,
                '@' => {
                    r_pos = I64Vector2(i as i64, j as i64);
                    MapElem::Empty
                }
                _ => MapElem::Empty,
            }
        }
    }

    let r = Regex::new("([><^v])").unwrap();
    let moves = r
        .captures_iter(input)
        .into_iter()
        .map(|c| c.extract())
        .map(|(_, [d])| d)
        .map(|d| match d {
            ">" => Direction::Right,
            "<" => Direction::Left,
            "v" => Direction::Down,
            "^" => Direction::Up,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mp = Map(mp);
    let n = moves.len();
    let mut g = Game {
        map: mp,
        robot: Robot {
            pos: r_pos,
            idx: 0,
            movement: moves,
        },
    };

    for i in 0..n {
        println!("{}", i);
        g.update();
    }

    println!("{}{:?}", g.map.to_string(), g.robot.pos);
    let mut ans = 0;
    for i in 0..N {
        for j in 0..M {
            if let MapElem::Box = g.map.0[i][j] {
                ans += i * 100 + j;
            }
        }
    }

    println!("{ans}");
}
