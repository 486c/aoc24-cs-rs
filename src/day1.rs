use std::{collections::HashMap, iter::zip};

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {

    let mut left = [0i32; 1000];
    let mut right = [0i32; 1000]; 

    for (i, line) in input.lines().enumerate() {
        let mut split = line.split("   ");

        let num1: i32 = split.next().unwrap().parse().unwrap();
        let num2: i32 = split.next().unwrap().parse().unwrap();

        left[i] = num1;
        right[i] = num2;
    }

    left.sort();
    right.sort();

    let zipped = zip(left.iter(), right.iter());
    let mut sum = 0;

    for (first, second) in zipped {
        sum += (first - second).abs();
    }


    sum
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut left_list = Vec::with_capacity(1000);
    let mut right = HashMap::with_capacity(1000);

    for line in input.lines() {
        let mut split = line.split("   ");

        let num1: u32 = split.next().unwrap().parse().unwrap();
        let num2: u32 = split.next().unwrap().parse().unwrap();

        right.entry(num2).and_modify(|x| *x += 1).or_insert(1);

        left_list.push(num1);
    }
    
    let mut sum = 0;

    for n in left_list { 
        sum += n * right.get(&n).unwrap_or(&0);
    }

    sum
}
