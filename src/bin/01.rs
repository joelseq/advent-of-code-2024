use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left_list, mut right_list) = get_lists(reader)?;

        // Sort the lists
        left_list.sort();
        right_list.sort();

        let mut dist: usize = 0;

        // Calculate distance
        for i in 0..left_list.len() {
            let left = left_list[i];
            let right = right_list[i];
            dist += left.abs_diff(right);
        }

        Ok(dist)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left_list, right_list) = get_lists(reader)?;

        // Get a map of number: count
        let num_map = right_list
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, &num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

        let mut similarity = 0;

        for num in left_list {
            if let Some(count) = num_map.get(&num) {
                if *count > 0 {
                    let inc = num * count;
                    similarity += inc;
                }
            }
        }

        Ok(similarity)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn get_lists<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>)> {
    let lines = reader.lines();
    let mut left_list = Vec::<usize>::new();
    let mut right_list = Vec::<usize>::new();

    for line in lines {
        let line = line?;
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_list.push(nums[0]);
        right_list.push(nums[1]);
    }

    Ok((left_list, right_list))
}
