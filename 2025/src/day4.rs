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

    pub fn total_around(&self, row: usize, col: usize) -> usize {
        let neigbors = [
            (row as i32 - 1, col as i32), // above
            (row as i32 + 1, col as i32), // below
            (row as i32, col as i32 + 1), // right
            (row as i32, col as i32 - 1), // left 

            (row as i32 - 1, col as i32 - 1), // above left
            (row as i32 - 1, col as i32 + 1), // above right

            (row as i32 + 1, col as i32 - 1), // below left
            (row as i32 + 1, col as i32 + 1), // below right
        ];
        
        let mut total = 0;
        for (nrow, ncol) in neigbors {
            if self.is_inside_grid(nrow, ncol) {
                //println!("[{nrow}, {ncol}] is inside grid");
                let c = self.get(nrow as usize, ncol as usize).unwrap();
                if c == '@' {
                    //println!("[{nrow}, {ncol}] is paper");
                    total += 1;
                };
            }
        };

        total

    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut count = 0;
    
    let grid = Grid::new(&lines);


    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let ch = grid.get(row, col).unwrap();

            if ch == '@' {
                let total_around = grid.total_around(row, col);

                if total_around < 4 {
                    count += 1;
                }
            }
        }
    };

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut count = 0;
    
    let mut grid = Grid::new(&lines);
    
    loop {
        let mut total_removed = 0;

        for row in 0..grid.height() {
            for col in 0..grid.width() {
                let ch = grid.get(row, col).unwrap();

                if ch == '@' {
                    let total_around = grid.total_around(row, col);

                    if total_around < 4 {
                        grid.replace(row, col, '.');
                        total_removed += 1;
                    }
                }
            }
        };

        count += total_removed;

        if total_removed == 0 {
            break;
        }
    }

    count
}
