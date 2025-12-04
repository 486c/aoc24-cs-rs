fn is_has_repeating2(num: &str) -> bool {
    let n = num.len();

    if n % 2 != 0 {
        return false;
    }


    let mut res = true;
    

    let mut i = 0;
    let mut j = n / 2;

    while i < n/2 && j < n {
        let first = &num[i..i + 1];
        let second = &num[j..j + 1];

        if first != second {
            res = false;
            break;
        }


        i += 1;
        j += 1;

    }

    res
}

fn smallest_repeating(num: &str) -> Option<usize> {
    let n = num.len();

    for pattern_len in 1..=n/2 {

        if n % pattern_len != 0 {
            continue;
        }

        let pattern = &num[0..pattern_len];
        let mut is_valid = true;
        let mut repeats = 0;

        for chunk in 1..n / pattern_len {
            let start = chunk * pattern_len;
            let end = start + pattern_len;

            if pattern != &num[start..end] {
                is_valid = false;
                break;
            };

            repeats += 1;
        }

        if is_valid {
            return Some(repeats + 1)
        }
    }

    None
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut split = input.split(",");
    let mut sum = 0;

    while let Some(range) = split.next() {
        let mut range_split = range.split("-");

        let first = range_split.next().unwrap();
        let second = range_split.next().unwrap();

        let range_start = usize::from_str_radix(&first, 10)
            .unwrap_or_else(|_| panic!("failed to radix: {first}"));
        let range_end = usize::from_str_radix(&second, 10).unwrap();

        println!("{range_start} - {range_end}");

        for num in range_start..=range_end {
            let st = format!("{num}");

            if is_has_repeating2(&st) {
                println!("Good - {num}");
                sum += num;
            }
        }
    };

    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut split = input.split(",");
    let mut sum = 0;

    while let Some(range) = split.next() {
        let mut range_split = range.split("-");

        let first = range_split.next().unwrap();
        let second = range_split.next().unwrap();

        let range_start = usize::from_str_radix(&first, 10)
            .unwrap_or_else(|_| panic!("failed to radix: {first}"));
        let range_end = usize::from_str_radix(&second, 10).unwrap();

        println!("{range_start} - {range_end}");

        for num in range_start..=range_end {
            let st = format!("{num}");

            let repeats = smallest_repeating(&st);

            if let Some(repeats) = repeats {
                if repeats >= 2 {
                    println!("Good - {num}");
                    sum += num;
                }
            }
        }
    };

    sum
}

#[test]
fn repeating() {
    //dbg!(is_has_repeating_periodic("12341234"));
    //dbg!(is_has_repeating_periodic("2121212122"));
    dbg!(smallest_repeating("11"));
    //dbg!(smallest_repeating("111"));
    //dbg!(smallest_repeating("2121"));
}

//#[aoc(day2, part2)]
//pub fn part2(input: &str) -> i32 {
    //0
//}
