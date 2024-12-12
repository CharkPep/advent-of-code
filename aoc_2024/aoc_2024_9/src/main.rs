// the further i go the shittier solutions become.
use std::collections::VecDeque;

const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, Debug)]
enum Block {
    Empty,
    Occupied(VecDeque<i32>),
}

#[derive(Debug)]
struct Memory(Vec<Block>);

impl Memory {
    pub fn append(&mut self, block: Block) {
        self.0.push(block);
    }

    pub fn compact(&mut self) {
        let n = self.0.len();
        for (i, b) in self.0.clone().into_iter().rev().enumerate() {
            if let Block::Occupied(mut block) = b {
                while let Some(f) = self.find_free() {
                    let Some(b) = block.pop_back() else { break };
                    f.push_back(b);
                }

                self.0[n - i - 1] = Block::Empty;
                if block.len() != 0 {
                    self.0[n - i - 1] = Block::Occupied(block);
                    break;
                }
            }
        }
    }

    fn find_free(&mut self) -> Option<&mut VecDeque<i32>> {
        for b in self.0.iter_mut() {
            match b {
                Block::Occupied(b) if b.len() != b.capacity() => return Some(b),
                _ => {}
            }
        }

        return None;
    }
}

fn main() {
    let mut blocks = INPUT
        .split("")
        .filter(|c| !c.trim().is_empty())
        .map(|c| str::parse::<i32>(c).unwrap())
        .collect::<Vec<_>>();

    let mut idxs = (blocks[1..])
        .into_iter()
        .cloned()
        .fold(vec![blocks[0]], |mut acc, x| {
            acc.push(acc.last().unwrap() + x);
            acc
        });

    let n = (*idxs.last().unwrap()) as usize;
    let mut expanded: Vec<i32> = vec![0; n];
    for i in (0..blocks.len()).step_by(2).rev() {
        let mut moved = false;
        for j in (1..i).step_by(2) {
            if blocks[j] >= blocks[i] {
                blocks[j] -= blocks[i];
                let idx = idxs[j - 1];
                for k in idx..idx + blocks[i] {
                    expanded[k as usize] = (i / 2) as i32;
                }

                idxs[j - 1] += blocks[i];
                moved = true;
                break;
            }
        }

        if !moved {
            let idx = if i == 0 { 0 } else { idxs[i - 1] };
            for k in idx..idx + blocks[i] {
                expanded[k as usize] = (i / 2) as i32;
            }
        }
    }

    let mut ans = 0usize;
    for (i, v) in expanded.into_iter().enumerate() {
        ans += i * v as usize;
    }

    println!("{ans}");

    // let mut memory = Memory(Vec::with_capacity(blocks.len()));
    // for i in 0..blocks.len() {
    //     if i % 2 == 0 {
    //         memory.append(Block::Occupied((0..blocks[i]).fold(
    //             VecDeque::with_capacity(blocks[i] as usize),
    //             |mut acc, _| {
    //                 acc.push_back((i / 2) as i32);
    //                 acc
    //             },
    //         )));
    //     } else if blocks[i] != 0 {
    //         memory.append(Block::Occupied(VecDeque::with_capacity(blocks[i] as usize)));
    //     }
    // }
    //
    // memory.compact();
    // let mut i = 0;
    // let mut ans = 0usize;
    // for b in memory.0 {
    //     let Block::Occupied(mut block) = b else {
    //         continue;
    //     };
    //     while let Some(v) = block.pop_front() {
    //         ans += (v * i) as usize;
    //         i += 1;
    //     }
    // }
    //
    // println!("{ans}");
}
