use std::{
    collections::HashMap,
    io::{self},
};

use crate::utils::read_lines;

#[derive(Debug, PartialEq)]
enum Operation {
    GreaterThan,
    LowerThan,
}

#[derive(Debug)]
struct Condition {
    variable: char,
    value: u32,
    operation: Operation,
    destination: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    destination: String,
}

pub fn solve(run_as: char) -> Result<(), io::Error> {
    let input_file = format!("src/day19/input_{run_as}.txt");

    let lines = read_lines(&input_file);
    let split_index = lines.iter().position(|x| x == "").unwrap();
    let workflows_str = lines[..split_index].to_vec();

    let mut workflows = vec![];
    for workflow_str in workflows_str {
        let conditions_start = workflow_str.chars().position(|x| x == '{').unwrap();
        let name: String = workflow_str.chars().collect::<Vec<char>>()[..conditions_start]
            .iter()
            .collect();

        let conditions_string = workflow_str.chars().collect::<Vec<char>>()
            [conditions_start + 1..workflow_str.len() - 1]
            .iter()
            .collect::<String>();

        let mut conditions_str = conditions_string.split(",").collect::<Vec<&str>>();

        let destination = conditions_str.pop().unwrap().to_string();

        let mut conditions = vec![];
        for condition_str in conditions_str {
            let condition = Condition {
                variable: condition_str.chars().nth(0).unwrap(),
                operation: if condition_str.chars().nth(1).unwrap() == '<' {
                    Operation::LowerThan
                } else {
                    Operation::GreaterThan
                },
                value: condition_str[2..condition_str.chars().position(|x| x == ':').unwrap()]
                    .parse::<u32>()
                    .unwrap(),
                destination: condition_str
                    [condition_str.chars().position(|x| x == ':').unwrap() + 1..]
                    .to_string(),
            };

            conditions.push(condition);
        }

        workflows.push(Workflow {
            name,
            destination,
            conditions,
        });
    }

    let ratings_str = lines[split_index + 1..]
        .iter()
        .map(|x| {
            let mut chars = x.chars();
            chars.next();
            chars.next_back();
            chars.collect()
        })
        .collect::<Vec<String>>();

    let mut ratings = vec![];

    for rating_str in ratings_str {
        let mut rating: HashMap<char, u32> = HashMap::new();
        let rs = rating_str.split(",").collect::<Vec<&str>>();

        for r in rs {
            let mut chars = r.chars();
            let key = chars.next().unwrap();
            chars.next();
            let value = chars.collect::<String>().parse::<u32>().unwrap();

            rating.insert(key, value);
        }

        ratings.push(rating);
    }

    let mut count = part1(&workflows, &ratings);
    println!("Part 1: {}", count);

    // let lines2 = read_lines_iterable(input_file)?;
    // count = part2(lines2);
    // println!("Part 2: {}", count);

    Ok(())
}

fn part1(workflows: &Vec<Workflow>, ratings: &Vec<HashMap<char, u32>>) -> u32 {
    let start_workflow = workflows.iter().find(|&x| x.name == "in").unwrap();

    let mut result = 0;

    for rating in ratings {
        result += execute_step(start_workflow, rating, workflows);
    }

    result
}

fn execute_step(
    workflow: &Workflow,
    rating: &HashMap<char, u32>,
    workflows: &Vec<Workflow>,
) -> u32 {
    for condition in &workflow.conditions {
        let value = rating.get(&condition.variable).unwrap();
        let mut is_hit = false;

        if condition.operation == Operation::GreaterThan && value > &condition.value {
            is_hit = true;
        } else if condition.operation == Operation::LowerThan && value < &condition.value {
            is_hit = true;
        }

        if is_hit {
            return evaluate_destination(condition.destination.clone(), rating, workflows);
        }
    }

    evaluate_destination(workflow.destination.clone(), rating, workflows)
}

fn evaluate_destination(
    destination: String,
    rating: &HashMap<char, u32>,
    workflows: &Vec<Workflow>,
) -> u32 {
    if destination == "A" {
        return rating.values().sum();
    } else if destination == "R" {
        return 0;
    }

    execute_step(
        workflows.iter().find(|&x| x.name == destination).unwrap(),
        rating,
        workflows,
    )
}

// fn part2(lines: Lines<BufReader<File>>) -> u32 {
//     0
// }
