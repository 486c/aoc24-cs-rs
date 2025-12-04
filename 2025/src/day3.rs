#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {

    let mut sum = 0;
    
    input
        .lines()
        .for_each(|line| {
            let mut max_num = 0;
            for i in 0..line.len() {

                let first = &line[i..i+1];
                for j in 0..line.len() {
                    if j <= i {
                        continue;
                    };

                    let second = &line[j..j+1];

                    let num_str = format!("{first}{second}");
                    let num = i32::from_str_radix(&num_str, 10).unwrap();

                    max_num = max_num.max(num);
                };
            };

            sum += max_num;
        });


    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {

    let mut sum = 0;
    
    input
        .lines()
        .for_each(|line| {
            let largest = find_largest(&line);
            sum += largest;
        });


    sum
}

fn find_largest(num: &str) -> usize {
    let n = num.len();
    let k = 12;
    let digits: Vec<char> = num.chars().collect();
    
    let mut stack = Vec::with_capacity(12);
    let mut to_remove = n - k;

    for digit in digits.iter() {
        while to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &digit {
            stack.pop();
            to_remove -= 1;
        };
        stack.push(digit);
    }

    if to_remove > 0 {
        stack.truncate(stack.len() - to_remove);
    }

    stack.truncate(k);
    
    let s: String = stack.into_iter().collect();

    usize::from_str_radix(&s, 10).unwrap()
}

#[test]
fn find_largest_for_part2() {
    assert_eq!(find_largest("987654321111111"), 987654321111);
    assert_eq!(find_largest("811111111111119"), 811111111119);
    assert_eq!(find_largest("234234234234278"), 434234234278);
    assert_eq!(find_largest("818181911112111"), 888911112111);
}
