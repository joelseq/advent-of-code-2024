use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
2 5 4 3 2
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();

        let mut safe_count = 0;

        for line in lines {
            let nums: Result<Vec<usize>, _> = line?
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .collect();
            let nums = nums?;

            if is_safe(&nums, false) {
                safe_count += 1;
            }
        }

        Ok(safe_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();

        let mut safe_count = 0;

        for line in lines {
            let nums: Result<Vec<usize>, _> = line?
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .collect();
            let nums = nums?;

            if is_safe(&nums, true) {
                safe_count += 1;
            }
        }

        Ok(safe_count)
    }

    assert_eq!(5, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

enum Order {
    Ascending,
    Descending,
}

fn is_safe(levels: &[usize], retry: bool) -> bool {
    let (safe, index) = is_safe_impl(levels);

    if safe {
        return true;
    }

    if retry {
        // Check if removing the first problematic index makes the level safe.
        let (safe, _) = is_safe_impl(&filter_idx(levels, index));

        if safe {
            return true;
        }

        // Otherwise check if removing the second problematic index makes the
        // level safe.
        let (safe, _) = is_safe_impl(&filter_idx(levels, index + 1));

        if safe {
            return true;
        }

        // Check the index before the first problematic index (if there is one).
        if index > 0 {
            let (safe, _) = is_safe_impl(&filter_idx(levels, index - 1));

            return safe;
        }
    }

    false
}

// Returns whether the levels are safe and if not, the index of the first unsafe
// level.
fn is_safe_impl(levels: &[usize]) -> (bool, usize) {
    if levels.len() < 2 {
        return (false, 0);
    }

    if levels[0] == levels[1] {
        return (false, 0);
    }

    let first = levels[0];
    let second = levels[1];

    let order = if first < second {
        Order::Ascending
    } else {
        Order::Descending
    };

    let mut index = 1;

    while index < levels.len() {
        let current = levels[index];
        let previous = levels[index - 1];

        match order {
            Order::Ascending => {
                if current < previous {
                    return (false, index - 1);
                }
            }
            Order::Descending => {
                if current > previous {
                    return (false, index - 1);
                }
            }
        }

        let diff = current.abs_diff(previous);

        if diff < 1 || diff > 3 {
            return (false, index - 1);
        }

        index += 1;
    }

    return (true, 0);
}

fn filter_idx(levels: &[usize], idx: usize) -> Vec<usize> {
    levels
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i != idx)
        .map(|(_, v)| *v)
        .collect()
}
