struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>
}

impl Grid {
    pub fn new(lines: &[&str]) -> Self {
        let height = lines.len();
        let width = if height > 0 { lines[0].len() } else { 0 };

        let cells = lines.iter()
            .map(|line| line.chars().collect())
            .collect();

        Self { width, height, cells }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }


    pub fn replace(&mut self, row: usize, col: usize, with: char) {
        if row < self.height && col < self.width {
            self.cells[row][col] = with
        }    
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if row < self.height && col < self.width {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    pub fn is_inside_grid(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32
        && col >= 0 && col < self.width as i32
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    let grid = Grid::new(&lines);
    
    let mut start = None;

    for i in 0..grid.width() {
        if let Some(ch) = grid.get(0, i) {
            if ch == 'S' {
                start = Some((0usize, i));
                break;
            }
        }
    };

    let start = start.unwrap();
    let mut beams = vec![start];
    
    let mut new_beams = Vec::new();

    loop {
        new_beams.clear();

        beams.retain_mut(|beam| {
            // going down
            beam.0 += 1;
            
            if let Some(beam_char) = grid.get(beam.0, beam.1) {
                if beam_char == '^' {
                    println!("Split at {:?}", beam);
                    total += 1;
                    let beam_left = (beam.0, beam.1 - 1);
                    let beam_right = (beam.0, beam.1 + 1);
                    new_beams.push(beam_left);
                    new_beams.push(beam_right);
                    false
                } else {
                    true
                }
            } else { false }
        });
        
        // leave only unique beams once
        'blk: loop {
            let mut removed = false;
            for i in 0..new_beams.len() {
                for j in 0..new_beams.len() {
                    if i == j {
                        continue;
                    };

                    if new_beams[i] == new_beams[j] {
                        removed = true;
                        new_beams.remove(j);
                        continue 'blk;
                    }
                }
            };

            if !removed {
                break;
            }
        };

        //println!("beams: {}", new_beams.len());
        //total += new_beams.len();
        if !new_beams.is_empty() {
            for new_beam in &new_beams {
                // Check if beam already exists before pushing
                let mut already_exists = false;
                for old_beam in &beams {
                    if old_beam == new_beam {
                        already_exists = true;
                        break;
                    }
                }
                
                if !already_exists {
                    beams.push(*new_beam);
                }
            }
        };

        if beams.is_empty() {
            break;
        }
    }
    println!("start: {:?}", start);


    total
}

#[derive(Debug, Clone)]
pub struct Beam {
    coords: (usize, usize),
    copies: usize
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;
    let mut total2 = 0;

    let grid = Grid::new(&lines);
    
    let mut start = None;

    for i in 0..grid.width() {
        if let Some(ch) = grid.get(0, i) {
            if ch == 'S' {
                start = Some((0usize, i));
                break;
            }
        }
    };

    let start = start.unwrap();
    let mut beams = vec![Beam { coords: start, copies: 1 }];
    
    let mut new_beams = Vec::new();

    loop {
        new_beams.clear();

        beams.retain_mut(|beam| {
            // going down
            beam.coords.0 += 1;

            if beam.coords.0 >= grid.height() {
                total += beam.copies;
                println!("Reached the end: {}", beam.copies);
                return false;
            };
            
            if let Some(beam_char) = grid.get(beam.coords.0, beam.coords.1) {
                if beam_char == '^' {
                    //println!("Split at {:?}", beam);
                    let beam_left = (beam.coords.0, beam.coords.1 - 1);
                    let beam_right = (beam.coords.0, beam.coords.1 + 1);
                    
                    
                    new_beams.push(Beam { coords: beam_left, copies: beam.copies });
                    new_beams.push(Beam { coords: beam_right, copies: beam.copies });
                    false
                } else {
                    true
                }
            } else { 
                panic!("something bad")
            }
        });
        
        //

        
        // Merge duplicates
        'blk: loop {
            for i in 0..beams.len() {
                for j in 0..beams.len() {
                    if i == j {
                        continue
                    };

                    if beams[i].coords == beams[j].coords {
                        beams[i].copies += beams[j].copies;
                        beams.remove(j);
                        continue 'blk;
                    };
                }
            };

            break
        }

        if !new_beams.is_empty() {
            for new_beam in &new_beams {
                beams.push(new_beam.clone());
            }
        };

        if beams.is_empty() {
            break;
        }
    }
    println!("start: {:?}", start);

    dbg!(total);
    total
}
