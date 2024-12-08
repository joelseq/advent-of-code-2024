use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let parser = Parser::new(input.chars().collect(), vec![Instruction::Mul]);
        let answer = parser.parse()?;

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let parser = Parser::new(
            input.chars().collect(),
            vec![Instruction::Mul, Instruction::Do],
        );
        let answer = parser.parse()?;

        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

struct Scanner {
    input: Vec<char>,
    index: usize,
}

impl Scanner {
    fn new(input: Vec<char>) -> Self {
        Self { input, index: 0 }
    }

    fn get_next_char(&mut self) -> Option<&char> {
        let ch = self.input.get(self.index);
        self.index += 1;
        ch
    }

    fn peek_next_char(&self) -> Option<&char> {
        self.input.get(self.index)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn peek_and_advance_token(&mut self, ch: char) -> Option<()> {
        if self.peek_next_char() == Some(&ch) {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn peek_and_advance_tokens(&mut self, ch: &[char]) -> Option<()> {
        for c in ch {
            if self.peek_and_advance_token(*c).is_none() {
                return None;
            }
        }

        Some(())
    }

    fn extract_digits(&mut self, end_char: char) -> Option<usize> {
        let mut num = 0;
        while let Some(digit) = self.peek_next_char() {
            if digit.is_digit(10) {
                num = num * 10 + digit.to_digit(10).unwrap() as usize;
                self.advance();
            } else if *digit == end_char {
                self.advance();
                return Some(num);
            } else {
                break;
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul,
    Do,
}

struct Parser {
    input: Vec<char>,
    instructions: Vec<Instruction>,
}

impl Parser {
    fn new(input: Vec<char>, instructions: Vec<Instruction>) -> Self {
        Self {
            input,
            instructions,
        }
    }

    fn parse(&self) -> Result<usize> {
        let mut scanner = Scanner::new(self.input.clone());
        let mut sum = 0;
        let mut enabled = true;

        while let Some(char) = scanner.get_next_char() {
            // Cases:
            // - If character is 'm', check if the next three characters are
            //   'ul('.
            // - If we have 'mul(', then check if next characters are digits
            //   until next character is ','.
            // - If we have 'mul(digits,', then check if next characters are
            //   digits until next character is ')'.
            // If all of the above is true, compute multiplication of the digits
            match char {
                'm' => {
                    if !self.instructions.contains(&Instruction::Mul) {
                        continue;
                    }

                    if scanner
                        .peek_and_advance_tokens(&vec!['u', 'l', '('])
                        .is_some()
                    {
                        // We have mul(
                        // Check if next characters are a number
                        if let Some(first_num) = scanner.extract_digits(',') {
                            if let Some(second_num) = scanner.extract_digits(')') {
                                if enabled {
                                    sum += first_num * second_num;
                                }
                            }
                        }
                    }
                }
                'd' => {
                    if !self.instructions.contains(&Instruction::Do) {
                        continue;
                    }

                    if scanner.peek_and_advance_token('o').is_some() {
                        if scanner.peek_and_advance_tokens(&vec!['(', ')']).is_some() {
                            enabled = true;
                        } else if scanner
                            .peek_and_advance_tokens(&vec!['n', '\'', 't', '(', ')'])
                            .is_some()
                        {
                            enabled = false;
                        }
                    }
                }
                _ => {
                    // Do nothing
                }
            }
        }

        Ok(sum)
    }
}
