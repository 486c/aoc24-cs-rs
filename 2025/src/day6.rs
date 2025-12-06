use std::fmt::Write;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut result = 0;
    let mut columns: Vec<Vec<usize>> = Vec::new();
    input.lines()
        .for_each(|line| {
            println!("=======");
            let mut split = line.split_whitespace();
            let mut column = 0;
            while let Some(num) = split.next() {
                match num {
                    "*" => {
                        let res = columns[column].clone().into_iter().reduce(|acc, x| acc * x).unwrap();
                        println!("column {column}: {res}");
                        column += 1;
                        result += res;
                        continue
                    }
                    "+" => {
                        let res = columns[column].clone().into_iter().reduce(|acc, x| acc + x).unwrap();
                        println!("column {column}: {res}");
                        column += 1;
                        result += res;
                        continue
                    }
                    _ => {}
                }

                let num = usize::from_str_radix(num, 10).unwrap();

                match columns.get_mut(column) {
                    Some(v) => v.push(num),
                    None => columns.push(vec![num]),
                }

                column += 1;
            }
        });

    result
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Sum,
    Mult,
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut result = 0;
    let mut max_lengths: Vec<usize> = Vec::new();
    let mut columns: Vec<Vec<Vec<char>>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();

    // #1: Get column lengths for later parsing
    input.lines()
        .for_each(|line| {
            let mut split = line.split_whitespace();

            let mut column = 0;
            while let Some(num) = split.next() {
                if num == "*" || num == "+" {
                    column += 1;
                    continue;
                };

                match max_lengths.get_mut(column) {
                    Some(v) => {
                        let copy = *v;
                        *v = copy.max(num.len())
                    },
                    None => max_lengths.push(num.len()),
                };

                column += 1;
            }
        });


    // #2: Read chars correctly with fixed known length
    input.lines()
        .for_each(|line| {
            let mut column = 0;
            let mut start = 0;
            loop {
                if column > max_lengths.len() - 1 {
                    break;
                }

                let end = start + max_lengths[column];

                if line[start..end].contains("*") {
                    match operators.get(column) {
                        Some(_) => panic!("operators should not be filled"),
                        None => {
                            println!("[{column}] assign mult: `{}`", &line[start..end]);
                            operators.push(Operator::Mult)
                        },
                    }
                    column += 1;
                    start = end + 1;
                    continue
                };

                if line[start..end].contains("+") {
                    match operators.get(column) {
                        Some(_) => panic!("operators should not be filled"),
                        None => {
                            println!("[{column}] assign sum: `{}`", &line[start..end]);
                            operators.push(Operator::Sum)
                        },
                    }
                    column += 1;
                    start = end + 1;
                    continue
                };

                let num: Vec<char> = line[start..end].chars().collect();
                match columns.get_mut(column) {
                    Some(v) => v.push(num),
                    None => columns.push(vec![num]),
                } 

                column += 1;
                start = end + 1;
            }
        });


    

    for idx in 0..columns.len() {
        // Extract max length of the number
        let max_length = columns[idx].iter().map(|num| num.len()).max().unwrap();
        for num in &mut columns[idx] {
            if max_length > num.len() {
                for _ in 0..max_length - num.len() {
                    num.insert(0, 'x');
                }
            }
        };
            
        //println!("=========");
        

        // Subcolumns parsing
        let operator_for_column = operators[idx];
        let mut extracted = Vec::new();
        //println!("Column {idx} max len: {max_length}");
        for subcolumn in 0..max_length {
            let mut final_num = String::new();

            for num in &columns[idx] {
                //println!("trying to take subcolumn {subcolumn} from the number {:?}", num);

                match num.get(subcolumn) {
                    Some(v) => { 
                        if *v != ' ' {
                            let _ = write!(final_num, "{}", v); 
                        }
                    },
                    None => panic!("no filled number found"),
                }
            };

            let parsed_num = usize::from_str_radix(&final_num, 10).unwrap();
            extracted.push(parsed_num);

            //println!("column: {idx}, subcolumn: {subcolumn}, num: {final_num}");
        };

        match operator_for_column {
            Operator::Sum => {
                let res = extracted.clone().into_iter().reduce(|acc, x| acc + x).unwrap();
                println!("sum: {:?} res: {res}", &extracted);
                result += res;
            },
            Operator::Mult => {
                let res = extracted.clone().into_iter().reduce(|acc, x| acc * x).unwrap();
                println!("mult: {:?} res: {res}", &extracted);
                result += res;
            },
        }

    }


    result
}
