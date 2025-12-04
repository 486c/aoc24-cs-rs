#[repr(u8)]
#[derive(Debug)]
enum State {
    MulOrDont,
    SearchDo,
    MulWithOpenBracket,
    FirstNum,
    SecondNum,
    Result,
    NoInputLeft,
}

struct Parser<'a> {
    state: State,
    pos: usize,
    result: usize,
    input: &'a [u8],

    first: usize,
    second: usize,

    is_can_be_disabled: bool,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8], is_can_be_disabled: bool) -> Self {
        Self {
            pos: 0,
            state: State::MulWithOpenBracket,
            result: 0,
            input,
            first: 0,
            second: 0,
            is_can_be_disabled,
        }
    }

    #[inline(always)]
    fn parse_mul_or_dont(&mut self) {
        loop {
            if self.pos + 7 > self.input.len() {
                self.state = State::NoInputLeft;
                return;
            }


            if self.input[self.pos] == b'd'
            && self.input[self.pos + 1] == b'o'
            && self.input[self.pos + 2] == b'n'
            && self.input[self.pos + 3] == b'\'' 
            && self.input[self.pos + 4] == b't' 
            && self.input[self.pos + 5] == b'(' 
            && self.input[self.pos + 6] == b')' 
            {
                self.state = State::SearchDo;
                self.pos += 7;
                break;
            }

            if self.pos + 4 > self.input.len() {
                self.state = State::NoInputLeft;
                return;
            }

            if self.input[self.pos] == b'm'
            && self.input[self.pos + 1] == b'u'
            && self.input[self.pos + 2] == b'l'
            && self.input[self.pos + 3] == b'(' {
                self.state = State::FirstNum;
                self.pos += 4;
                break;
            }

            self.pos += 1;
        }
    }


    #[inline(always)]
    fn parse_search_do(&mut self) {
        loop {
            if self.pos + 4 > self.input.len() {
                self.state = State::NoInputLeft;
                return;
            }

            if self.input[self.pos] == b'd'
            && self.input[self.pos + 1] == b'o'
            && self.input[self.pos + 2] == b'('
            && self.input[self.pos + 3] == b')' {
                self.state = State::MulOrDont;
                self.pos += 4;
                break;
            }


            self.pos += 1;
        }
    }
    
    #[inline(always)]
    fn reset(&mut self) {
        if self.is_can_be_disabled {
            self.state = State::MulOrDont;
        } else {
            self.state = State::MulWithOpenBracket;
        }
        self.first = 0;
        self.second = 0;
    }

    #[inline(always)]
    fn parse_mul(&mut self) {
        loop {
            if self.pos + 4 > self.input.len() {
                self.state = State::NoInputLeft;
                return;
            }

            if self.input[self.pos] == b'm'
            && self.input[self.pos + 1] == b'u'
            && self.input[self.pos + 2] == b'l'
            && self.input[self.pos + 3] == b'(' {
                self.state = State::FirstNum;
                self.pos += 4;
                break;
            }

            self.pos += 1;
        }
    }

    #[inline(always)]
    fn parse_first_num(&mut self) {
        loop {
            if self.input[self.pos].is_ascii_digit() {
                self.first = (self.first * 10) + (self.input[self.pos] - b'0') as usize;
            } else {
                if self.input[self.pos] == b',' {
                    self.pos += 1;
                    break
                } else {
                    self.pos += 1;
                    self.reset();
                    return;
                }
            }

            self.pos += 1;

        }

        self.state = State::SecondNum;
    }

    #[inline(always)]
    fn parse_second_num(&mut self) {
        loop {
            if self.input[self.pos].is_ascii_digit() {
                self.second = (self.second * 10) + (self.input[self.pos] - b'0') as usize;
            } else {
                if self.input[self.pos] == b')' {
                    self.pos += 1;
                    break
                } else {
                    self.pos += 1;
                    self.reset();
                    return;
                }
            }

            self.pos += 1;

        }

        self.state = State::Result;
    }

    pub fn parse(&mut self) -> usize {
        loop {
            match self.state {
                State::MulWithOpenBracket => self.parse_mul(),
                State::FirstNum => self.parse_first_num(),
                State::SecondNum => self.parse_second_num(),
                State::Result => {
                    self.result += self.first * self.second;
                    self.reset();
                },
                State::NoInputLeft => {
                    return self.result
                },
                _ => { panic!("shouldn't be here") }
            }
        }
    }

    pub fn parse2(&mut self) -> usize {
        self.state = State::MulOrDont;

        loop {
            match self.state {
                State::MulOrDont => self.parse_mul_or_dont(),
                State::SearchDo => self.parse_search_do(),
                State::MulWithOpenBracket => self.parse_mul(),
                State::FirstNum => self.parse_first_num(),
                State::SecondNum => self.parse_second_num(),
                State::Result => {
                    self.result += self.first * self.second;
                    self.reset();
                },
                State::NoInputLeft => {
                    return self.result
                },}
        }
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut p = Parser::new(unsafe { std::mem::transmute(input) }, false);
    p.parse()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut p = Parser::new(unsafe { std::mem::transmute(input) }, true);
    p.parse2()
}
