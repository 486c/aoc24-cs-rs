use hashbrown::HashSet;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut is_available = false;
    let mut fresh_ranges = Vec::new();
    let mut total_fresh = 0;

    input
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                is_available = true;
                return;
            };

            match is_available {
                false => {
                    let mut split = line.split("-");
                    let first = usize::from_str_radix(split.next().unwrap(), 10).unwrap();
                    let second = usize::from_str_radix(split.next().unwrap(), 10).unwrap();
                    let range = first..=second;
                    fresh_ranges.push(range);
                },
                true => {
                    let num = usize::from_str_radix(line, 10).unwrap();
                    for range in &fresh_ranges {
                        if range.contains(&num) {
                            total_fresh += 1;
                            break;
                        }
                    }
                },
            }
        });

    total_fresh
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut is_available = false;
    let mut fresh_ranges = Vec::new();
    let mut fresh_set: HashSet<usize> = HashSet::new();

    input
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                is_available = true;
                return;
            };

            if is_available {
                return;
            }

            let mut split = line.split("-");
            let first = usize::from_str_radix(split.next().unwrap(), 10).unwrap();
            let second = usize::from_str_radix(split.next().unwrap(), 10).unwrap();
            let range = first..=second;

            fresh_ranges.push(range);

        });

    fresh_ranges.sort_by(|r1, r2| {
        r1.start().cmp(r2.start())
    });

    println!("Raw len: {}", fresh_ranges.len());

    // Attempt to optimize 1: Remove fully included ranges that are included
    // in some other range
    loop {
        let mut to_remove = None;
        'blk: for i in 0..fresh_ranges.len() {
            for j in 0..fresh_ranges.len() {
                if i == j {
                    continue;
                };

                if fresh_ranges[j].contains(&fresh_ranges[i].start()) 
                    && fresh_ranges[j].contains(&fresh_ranges[i].end()) {
                        println!("Range {:?} fully contains", &fresh_ranges[i]);
                        to_remove = Some(i);
                        break 'blk;
                }
            }
        };

        if let Some(to_remove) = to_remove {
            fresh_ranges.remove(to_remove);
        } else {
            break;
        }
    }

    println!("Attempt1 len: {}", fresh_ranges.len());

    // Attempt to optimize 2: Merge ranges
    loop {
        let mut merged = false;
        let mut to_remove = None;

        'blk: for i in 0..fresh_ranges.len() {
            for j in 0..fresh_ranges.len() {
                if i == j {
                    continue;
                };

                let trying_to_merge = fresh_ranges[i].clone();
                let to_merge = fresh_ranges[j].clone();

                if to_merge.contains(trying_to_merge.end()) {
                    fresh_ranges[j] = *trying_to_merge.start()..=*to_merge.end();
                    merged = true;
                    to_remove = Some(i);
                    break 'blk;
                }
            }
        };

        if let Some(to_remove) = to_remove {
            fresh_ranges.remove(to_remove);
        }

        if !merged {
            break;
        }
    }

    println!("Attempt2 len: {}", fresh_ranges.len());

    dbg!(&fresh_ranges);
    
    let mut total = 0;
    for range in fresh_ranges.iter() {
        total += (range.end() - range.start()) + 1;
    }

    total
}
