use std::{hash::BuildHasherDefault, iter::zip};

use fxhash::FxHasher32;
use hashbrown::HashMap;

#[inline]
fn parse_i32(n: u8) -> i32 {
    n as i32 - 0x30
}

#[inline]
fn parse_u32(n: u8) -> u32 {
    n as u32 - 0x30
}

#[inline]
fn atoi_i32(str: &[u8]) -> i32 {
    parse_i32(str[0]) * 10000 +
    parse_i32(str[1]) * 1000 +
    parse_i32(str[2]) * 100 +
    parse_i32(str[3]) * 10 +
    parse_i32(str[4]) 
}


#[inline]
fn atoi_u32(str: &[u8]) -> u32 {
    parse_u32(str[0]) * 10000 +
    parse_u32(str[1]) * 1000 +
    parse_u32(str[2]) * 100 +
    parse_u32(str[3]) * 10 +
    parse_u32(str[4]) 
}


#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let mut left = [0i32; 1000];
    let mut right = [0i32; 1000]; 

    let input: &[u8] = unsafe { std::mem::transmute(&input[0..]) };

    for (i, (x,y)) in (0..1000).map(|i| {
        &input[i * 14.. i * 14 + 13]
    })
    .map(|line| {
        (
            atoi_i32(&line[0..5]), 
            atoi_i32(&line[8..13])
        )
    }).enumerate() {
        left[i] = x;
        right[i] = y;
    }

    left.sort_unstable();
    right.sort_unstable();

    zip(left.iter(), right.iter())
        .fold(0, |acc, nums| acc + (nums.0 - nums.1).abs())
}

#[aoc(day1, part2)]
pub fn run(input: &str) -> u32 {
    let mut right: HashMap<u32, u32, BuildHasherDefault<FxHasher32>> = 
        HashMap::with_capacity_and_hasher(1000, BuildHasherDefault::default());

    let mut left_list = [0u32; 1000];

    let input: &[u8] = unsafe { std::mem::transmute(&input[0..]) };

    for (i, (x,y)) in (0..1000).map(|i| {
        &input[i * 14.. i * 14 + 13]
    })
    .map(|line| {
        (
            atoi_u32(&line[0..5]), 
            atoi_u32(&line[8..13])
        )
    }).enumerate() {
        right.entry(y).and_modify(|x| *x += 1).or_insert(1);
        left_list[i] = x;
    }

    left_list.iter().fold(0, |acc, num| acc + num * right.get(num).unwrap_or(&0))
}
