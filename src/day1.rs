use std::{collections::HashMap, iter::zip};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {

    let mut left = [0i32; 1000];
    let mut right = [0i32; 1000]; 

    input.lines()
        .filter_map(|line| line.split_once("   "))
        .map(|line| {
            (i32::from_str_radix(line.0, 10).unwrap(), i32::from_str_radix(line.1, 10).unwrap())
        })
        .enumerate()
        .for_each(|(i, nums)| {
            left[i] = nums.0;
            right[i] = nums.1;
        });

    left.sort_unstable();
    right.sort_unstable();

    zip(left.iter(), right.iter())
        .fold(0, |acc, nums| acc + (nums.0 - nums.1).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left_list = [0u32; 1000];
    let mut right = hashbrown::HashMap::with_capacity(1000);

    input.lines()
        .filter_map(|line| line.split_once("   "))
        .map(|line| {
            (u32::from_str_radix(line.0, 10).unwrap(), u32::from_str_radix(line.1, 10).unwrap())
        })
        .enumerate()
        .for_each(|(i, nums)| {
            right.entry(nums.1).and_modify(|x| *x += 1).or_insert(1);
            left_list[i] = nums.0;
        });

    left_list.iter().fold(0, |acc, num| acc + num * right.get(num).unwrap_or(&0))
}
