
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut turns = 0;
    let mut dial: i32 = 50;
    input
        .lines()
        .for_each(|line| {
            let num = i32::from_str_radix(&line[1..], 10).unwrap();

            match &line[0..1] {
                "R" => {
                    dial += num;

                    loop {
                        if dial < 100 {
                            break;
                        };


                        dial = dial - 100;
                    }
                }
                "L" => {

                    dial -= num;
                    loop {
                        if dial >= 0 {
                            break;
                        };

                        dial = 100 - dial.abs();
                    }
                }
                _ => {}
            };

            if dial == 100 {
                dial = 0;
            }

            if dial == 0 {
                turns += 1;
            }
        });

    turns
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut turns = 0;
    let mut dial: i32 = 50;
    input
        .lines()
        .for_each(|line| {
            let num = i32::from_str_radix(&line[1..], 10).unwrap();

            match &line[0..1] {
                "R" => {
                    dial += num;

                    println!("[{line}] dial: {dial}");

                    loop {
                        if dial <= 100 {
                            break;
                        };
                        println!("[R] LOOP: {dial}");


                        dial = dial - 100;
                        turns += 1;
                        println!("inc");
                    }
                }
                "L" => {
                    println!("[{line}] dial: {dial}");

                    let mut is_countable = dial != 0;

                    dial -= num;
                    loop {
                        println!("[L] LOOP: {dial}");
                        if dial >= 0 {
                            break;
                        };


                        dial = 100 - dial.abs();

                        if is_countable {
                            turns +=1 ;
                        } else {
                            is_countable = true
                        }
                        println!("inc");
                    }
                }
                _ => {}
            };

            if dial == 100 {
                dial = 0;
            }

            if dial == 0 {
                println!("Points at => 0");
                turns += 1;
            }
        });

    turns
}
