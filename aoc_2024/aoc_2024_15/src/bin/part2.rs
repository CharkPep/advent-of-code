use std::{
    fmt::Debug,
    ops::{Add, Index, IndexMut},
};

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

impl Add<Direction> for I64Vector2 {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        I64Vector2(
            self.0 + MOVEMENT[rhs as usize].0,
            self.1 + MOVEMENT[rhs as usize].1,
        )
    }
}

impl Add<I64Vector2> for Direction {
    type Output = I64Vector2;
    fn add(self, rhs: I64Vector2) -> Self::Output {
        I64Vector2(
            MOVEMENT[self as usize].0 + rhs.0,
            MOVEMENT[self as usize].1 + rhs.1,
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MapElem {
    Edge,
    BoxL,
    BoxR,
    Empty,
}

impl ToString for MapElem {
    fn to_string(&self) -> String {
        match self {
            Self::Empty => ".".to_string(),
            Self::BoxL => "[".to_string(),
            Self::BoxR => "]".to_string(),
            Self::Edge => "#".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn change_position(&mut self, pos: I64Vector2) {
        self.pos = pos;
    }
}

impl Iterator for Robot {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.movement.len() <= self.idx {
            return None;
        }

        let d = self.movement[self.idx];
        self.idx += 1;
        return Some(d);
    }
}

struct Map(Vec<Vec<MapElem>>);

impl Index<I64Vector2> for Map {
    type Output = MapElem;
    fn index(&self, index: I64Vector2) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<I64Vector2> for Map {
    fn index_mut(&mut self, index: I64Vector2) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

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
    fn create_propagation_group(
        &self,
        elem_pos: I64Vector2,
        direction: Direction,
        traversing_direction: Direction,
    ) -> (Vec<(I64Vector2, MapElem)>, bool) {
        match self[elem_pos] {
            MapElem::Edge => (vec![], false),
            MapElem::BoxL => {
                let (mut v, mut s) = (vec![], true);
                if traversing_direction != Direction::Left {
                    (v, s) = self.create_propagation_group(
                        elem_pos + Direction::Right,
                        direction,
                        Direction::Right,
                    );
                }

                v.push((elem_pos, self[elem_pos]));
                if self[elem_pos + direction] != self[elem_pos] || traversing_direction == direction
                {
                    let (vd, sd) =
                        self.create_propagation_group(elem_pos + direction, direction, direction);

                    v.extend(vd);
                    if !sd {
                        s = sd;
                    }
                }

                (v, s)
            }
            MapElem::BoxR => {
                let (mut v, mut s) = (vec![], true);
                if traversing_direction != Direction::Right {
                    (v, s) = self.create_propagation_group(
                        elem_pos + Direction::Left,
                        direction,
                        Direction::Left,
                    );
                }

                v.push((elem_pos, self[elem_pos]));
                if self[elem_pos + direction] != self[elem_pos] || traversing_direction == direction
                {
                    let (vd, sd) =
                        self.create_propagation_group(elem_pos + direction, direction, direction);

                    v.extend(vd);
                    if !sd {
                        s = sd;
                    }
                }

                (v, s)
            }
            MapElem::Empty => (vec![], true),
            _ => (vec![], true),
        }
    }

    fn move_direction(&mut self, elem_pos: I64Vector2, direction: Direction) -> bool {
        let (g, c) = self.create_propagation_group(elem_pos, direction, direction);
        if !c {
            return c;
        }

        for p in g.iter() {
            self[p.0] = MapElem::Empty;
        }

        for p in g.iter() {
            self[p.0 + direction] = p.1;
        }

        c
    }

    fn is_empty(&mut self, elem_pos: I64Vector2) -> bool {
        if let MapElem::Empty = self[elem_pos] {
            true
        } else {
            false
        }
    }
}

struct Game {
    map: Map,
    robot: Robot,
}

impl Game {
    fn update(&mut self) -> &Map {
        let Some(d) = self.robot.next() else {
            return &self.map;
        };

        if self.map.is_empty(self.robot.pos + d) || self.map.move_direction(self.robot.pos + d, d) {
            self.robot.change_position(self.robot.pos + d);
        }

        &self.map
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let r = Regex::new("((?m)^#.*#$)").unwrap();
    let mut mp = vec![vec![MapElem::Empty; M * 2]; N];
    let mut r_pos = I64Vector2(0, 0);
    for (i, l) in r
        .captures_iter(input)
        .into_iter()
        .map(|c| c.extract())
        .map(|(_, [l])| l)
        .enumerate()
    {
        for (j, c) in l.chars().enumerate().map(|(i, c)| (2 * i, c)) {
            mp[i][j] = match c {
                '#' => MapElem::Edge,
                'O' => MapElem::BoxL,
                '@' => {
                    r_pos = I64Vector2(i as i64, j as i64);
                    MapElem::Empty
                }
                _ => MapElem::Empty,
            };

            mp[i][j + 1] = match c {
                '#' => MapElem::Edge,
                'O' => MapElem::BoxR,
                _ => MapElem::Empty,
            };
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

    println!("Start\n{:?}\n{}", g.robot.pos, g.map.to_string());
    for i in 0..n {
        println!("{}\n{}", i, g.update().to_string());
    }

    let mut ans = 0;
    for i in 0..N {
        for j in 0..2 * M {
            if let MapElem::BoxL = g.map.0[i][j] {
                ans += i * 100 + j;
            }
        }
    }

    println!("{ans}");
}
