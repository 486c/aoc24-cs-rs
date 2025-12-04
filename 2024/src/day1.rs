use std::iter::zip;

#[inline(always)]
unsafe fn parse_two_numbers_unrolled_u32(s: &[u8]) -> (u32, u32) {
    (
        // LEFT
        (s[0] as u32 - 0x30) * 10000
            + (s[1] as u32 - 0x30) * 1000
            + (s[2] as u32 - 0x30) * 100
            + (s[3] as u32 - 0x30) * 10
            + (s[4] as u32 - 0x30) * 1,
        // RIGHT
        (s[8] as u32 - 0x30) * 10000
            + (s[9] as u32 - 0x30) * 1000
            + (s[10] as u32 - 0x30) * 100
            + (s[11] as u32 - 0x30) * 10
            + (s[12] as u32 - 0x30) * 1,
    )
}

#[inline(always)]
unsafe fn parse_two_numbers_unrolled_i32(s: &[u8]) -> (i32, i32) {
    (
        // LEFT
        (s[0] as i32 - 0x30) * 10000
            + (s[1] as i32 - 0x30) * 1000
            + (s[2] as i32 - 0x30) * 100
            + (s[3] as i32 - 0x30) * 10
            + (s[4] as i32 - 0x30) * 1,
        // RIGHT
        (s[8] as i32 - 0x30) * 10000
            + (s[9] as i32 - 0x30) * 1000
            + (s[10] as i32 - 0x30) * 100
            + (s[11] as i32 - 0x30) * 10
            + (s[12] as i32 - 0x30) * 1,
    )
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    static mut LEFT: [i32; 1000] = [0i32; 1000];
    static mut RIGHT: [i32; 1000] = [0i32; 1000]; 

    let input: &[u8] = unsafe { std::mem::transmute(&input[0..]) };

    for (i, (x,y)) in (0..1000).map(|i| {
        &input[i * 14.. i * 14 + 13]
    })
    .map(|line| unsafe { parse_two_numbers_unrolled_i32(line) })
    .enumerate() {
        unsafe {
            LEFT[i] = x;
            RIGHT[i] = y;
        }
    }
    
    unsafe {
        LEFT.sort_unstable();
        RIGHT.sort_unstable();
    
        zip(LEFT.iter(), RIGHT.iter())
            .fold(0, |acc, nums| acc + (nums.0 - nums.1).abs())
    }
}


#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    static mut RIGHT: [u32; 100_000] = [0u32; 100_000];
    let mut left_list = [0; 1000];

    let input: &[u8] = unsafe { std::mem::transmute(&input[0..]) };

    for (i, (x,y)) in (0..1000).map(|i| {
        &input[i * 14.. i * 14 + 13]
    })
    .map(|line| unsafe { parse_two_numbers_unrolled_u32(line) })
    .enumerate() {
        unsafe {
            RIGHT[y as usize] += 1;
        }
        left_list[i] = x;
    }

    left_list.iter().fold(0, |acc, num| acc + num * unsafe { RIGHT[*num as usize] })
}
