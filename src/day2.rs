use std::cmp::Ordering;

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let mut temp_arr = [0i8; 255];
    let mut indx = 0;

    let mut res = 0;

    'line_loop: for nums in input.lines().map(|line| line.split(' '))
        .map(|x| x.map(|n| i8::from_str_radix(n, 10).unwrap())) 
    {
        indx = 0;
        for num in nums {
            temp_arr[indx] = num;
            indx += 1;
        }
        
        let mut state: Option<Ordering> = None;

        for arr in temp_arr[0..indx].windows(2) {
            let abs = (arr[1] - arr[0]).abs();

            if abs > 3 {
                continue 'line_loop;
            }

            let new_state = arr[0].cmp(&arr[1]);

            if let Some(state) = state {
                if new_state != state { continue 'line_loop };
            } else {
                state = Some(new_state);
            }
        }

        res += 1;
    }
    
    res
}

fn is_valid(arr: &[i8]) -> bool {
    let mut state: Option<Ordering> = None;
    for arr1 in arr.windows(2) {
        let abs = (arr1[1] - arr1[0]).abs();

        if abs > 3 {
            return false;
        }

        let new_state = arr1[0].cmp(&arr1[1]);

        if let Some(state) = state {
            if new_state != state { return false };
        } else {
            state = Some(new_state);
        }
    }

    true
}

fn is_all_safe(arr: &[i8]) -> bool {
    let mut skip_temp = [0i8; 255];
    let mut skin_indx = 0;
    

    (0..arr.len()).map(|i| {
        skin_indx = 0;
        for j in 0..arr.len() {
            if j == i {
                continue;
            }

            skip_temp[skin_indx] = arr[j];
            skin_indx += 1;
        }

        is_valid(&skip_temp[0..skin_indx])
    }).any(|x| x == true)
}


#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let mut temp_arr = [0i8; 255];
    let mut indx = 0;

    let mut res = 0;

    for nums in input.lines().map(|line| line.split(' '))
        .map(|x| x.map(|n| i8::from_str_radix(n, 10).unwrap()))
    {
        indx = 0;
        for num in nums {
            temp_arr[indx] = num;
            indx += 1;
        }

        if is_valid(&temp_arr[0..indx]) {
            res += 1;
        } else {
            if is_all_safe(&temp_arr[0..indx]) {
                res += 1;
            }
        }
    }
    
    res
}
