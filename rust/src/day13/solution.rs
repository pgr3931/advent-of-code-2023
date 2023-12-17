use std::io::{self};

use crate::utils::read_lines_iterable;
#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    fn new() -> Pattern {
        Pattern {
            rows: vec![],
            cols: vec![],
        }
    }
}

pub fn solve(run_as: char) -> Result<(), io::Error> {
    let input_file = format!("src/day13/input_{run_as}.txt");

    let lines = read_lines_iterable(&input_file)?;

    let mut patterns = vec![];
    let mut pattern = Pattern::new();
    for line in lines {
        if let Ok(l) = line {
            if l == "" {
                patterns.push(pattern);
                pattern = Pattern::new();
                continue;
            }

            pattern.rows.push(l);
        }
    }

    patterns.push(pattern);

    // pivot the map to get the columns
    for p in &mut patterns {
        for x in 0..p.rows[0].len() {
            let mut col = "".to_string();

            for y in 0..p.rows.len() {
                col.push(p.rows[y].chars().collect::<Vec<char>>()[x]);
            }

            p.cols.push(col);
        }
    }

    let mut count = part1(&patterns);
    println!("Part 1: {}", count);

    // let lines2 = read_lines_iterable(input_file)?;
    // count = part2(lines2);
    // println!("Part 2: {}", count);

    Ok(())
}

fn part1(patterns: &Vec<Pattern>) -> i32 {
    let mut result = 0;

    for pattern in patterns {
        let mut reflection = find_reflection(pattern.cols.clone());

        if reflection.is_some() {
            result += reflection.unwrap();
        } else {
            reflection = find_reflection(pattern.rows.clone());
            result += reflection.unwrap() * 100;
        }
    }

    result
}

fn find_reflection(pattern: Vec<String>) -> Option<i32> {
    'outer: for i in 1..pattern.len() {
        for j in i..pattern.len() {
            // if everything until the left edge was valid, we end up here - that's our reflection line
            if i - (j - i) == 0 {
                return Some(i as i32);
            }

            let left = &pattern[i - (j - i) - 1];
            let right = &pattern[j];

            // if the pair doesn't match, got to the next pair
            if left != right {
                continue 'outer;
            }
        }

        // if everything until the right edge was valid, we end up here
        return Some(i as i32);
    }

    // otherwise there is no reflection on this axis
    None
}

// fn part2(lines: Lines<BufReader<File>>) -> u32 {}