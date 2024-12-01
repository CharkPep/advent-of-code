use std::{collections::HashMap, str::FromStr};

#[allow(warnings)]
static INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> i32 {
    let (mut ids_1, mut ids_2) = INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let l = line.split("   ").collect::<Vec<_>>();
            (
                <i32 as FromStr>::from_str(l[0]).unwrap(),
                <i32 as FromStr>::from_str(l[1]).unwrap(),
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    ids_1.sort();
    ids_2.sort();

    return ids_1
        .into_iter()
        .zip(ids_2)
        .map(|(id_1, id_2)| return i32::abs(id_1 - id_2))
        .sum();
}

fn part_2() -> i32 {
    let (mut ids_1, mut ids_2) = INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let l = line.split("   ").collect::<Vec<_>>();
            (
                <i32 as FromStr>::from_str(l[0]).unwrap(),
                <i32 as FromStr>::from_str(l[1]).unwrap(),
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let mut entries = HashMap::new();
    ids_2.into_iter().for_each(|id| {
        *entries.entry(id).or_insert(0) += 1;
    });

    ids_1
        .into_iter()
        .map(|id| id * entries.get(&id).unwrap_or(&0))
        .sum()
}
