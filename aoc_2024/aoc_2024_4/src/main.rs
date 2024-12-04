use core::str;

static INPUT: &'static str = include_str!("../input.txt");
static WORD: &str = "MAS";

fn dfs(p: &Vec<Vec<&str>>, i: isize, j: isize, current: usize, dir: &'static str) -> bool {
    if i >= 0
        && i < p.len() as isize
        && j < p[0].len() as isize
        && j >= 0
        && p[i as usize][j as usize].as_bytes()[0] == WORD.as_bytes()[current]
    {
        if current == WORD.len() - 1 {
            return true;
        }

        return match dir {
            "vu" => dfs(p, i + 1, j, current + 1, dir),
            "vd" => dfs(p, i - 1, j, current + 1, dir),
            "hr" => dfs(p, i, j + 1, current + 1, dir),
            "hl" => dfs(p, i, j - 1, current + 1, dir),
            "dul" => dfs(p, i - 1, j - 1, current + 1, dir),
            "dur" => dfs(p, i - 1, j + 1, current + 1, dir),
            "ddl" => dfs(p, i + 1, j - 1, current + 1, dir),
            "ddr" => dfs(p, i + 1, j + 1, current + 1, dir),
            _ => false,
        };
    }

    return false;
}

fn main() {
    let puzzle = INPUT
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| line.split("").filter(|c| !c.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (i, l) in puzzle.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            match c.as_bytes()[0] {
                b'M' => {
                    if dfs(&puzzle, i as isize, j as isize, 0, "ddr")
                        && (dfs(&puzzle, i as isize, j as isize + 2, 0, "ddl")
                            || dfs(&puzzle, i as isize + 2, j as isize, 0, "dur"))
                    {
                        ans += 1;
                    }
                }
                b'S' => {
                    if dfs(&puzzle, i as isize + 2, j as isize + 2, 0, "dul")
                        && (dfs(&puzzle, i as isize, j as isize + 2, 0, "ddl")
                            || dfs(&puzzle, i as isize + 2, j as isize, 0, "dur"))
                    {
                        ans += 1;
                    }
                }
                _ => {}
            }
        }
    }

    println!("{ans}");
}
