use std::{
    collections::HashMap,
    io::{self},
};

use crate::utils::read_lines;

#[derive(PartialEq, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

// ...........
// .S-------7.
// .|F-----7|.
// .||OOOOO||.
// .||OOOOO||.
// .|L-7OF-J|.
// .|II|O|II|.
// .L--JOL--J.
// .....O.....

pub fn solve(run_as: char) -> Result<(), io::Error> {
    let input_file = format!("src/naughty_even_days/day10/input_{run_as}.txt");

    let lines = read_lines(&input_file);
    let pipe_map = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = part1(pipe_map.clone());
    println!("Part 1: {}", count);

    // let lines2 = read_lines_iterable(input_file)?;
    count = part2(pipe_map);
    println!("Part 2: {}", count);

    Ok(())
}

fn part1(pipe_map: Vec<Vec<char>>) -> u32 {
    let mut result = 0;

    let mut start = (0, 0);
    'outer: for y in 0..pipe_map.len() {
        for x in 0..pipe_map[y].len() {
            if pipe_map[y][x] == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut coming_from = get_start_dir(pipe_map.clone(), start);
    let mut curr_tile = get_next_coordinates(start, &get_start_dir(pipe_map.clone(), start));

    while pipe_map[curr_tile.1][curr_tile.0] != 'S' {
        coming_from = get_next_dir(pipe_map[curr_tile.1][curr_tile.0], coming_from);
        curr_tile = get_next_coordinates(curr_tile, &coming_from);
        result += 1;
    }

    ((result as f32) / 2.0).ceil() as u32
}

pub fn part2(pipe_map: Vec<Vec<char>>) -> u32 {
    let mut result: HashMap<(usize, usize), i32> = HashMap::new();
    let mut path_tiles: Vec<(usize, usize)> = vec![];

    let mut start = (0, 0);
    'outer: for y in 0..pipe_map.len() {
        for x in 0..pipe_map[y].len() {
            if pipe_map[y][x] == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    path_tiles.push(start);

    let mut coming_from = get_start_dir(pipe_map.clone(), start);
    let mut curr_tile = get_next_coordinates(start, &get_start_dir(pipe_map.clone(), start));

    while pipe_map[curr_tile.1][curr_tile.0] != 'S' {
        path_tiles.push(curr_tile);
        // look right
        let inner_dirs = get_inner_dirs(pipe_map[curr_tile.1][curr_tile.0], &coming_from);

        for dir in inner_dirs {
            if dir == Dir::North {
                for y in 0..curr_tile.1 {
                    if (curr_tile.0, y) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((curr_tile.0, y)).or_insert(0) += 1;
                }

                for y in (curr_tile.1 + 1)..pipe_map.len() {
                    if (curr_tile.0, y) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((curr_tile.0, y)).or_insert(0) -= 1;
                }
            }

            if dir == Dir::East {
                for x in (curr_tile.0 + 1)..pipe_map[0].len() {
                    if (x, curr_tile.1) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((x, curr_tile.1)).or_insert(0) += 1;
                }

                for x in 0..curr_tile.0 {
                    if (x, curr_tile.1) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((x, curr_tile.1)).or_insert(0) -= 1;
                }
            }

            if dir == Dir::South {
                for y in (curr_tile.1 + 1)..pipe_map.len() {
                    if (curr_tile.0, y) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((curr_tile.0, y)).or_insert(0) += 1;
                }

                for y in 0..curr_tile.1 {
                    if (curr_tile.0, y) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((curr_tile.0, y)).or_insert(0) -= 1;
                }
            }

            if dir == Dir::West {
                for x in 0..curr_tile.0 {
                    if (x, curr_tile.1) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((x, curr_tile.1)).or_insert(0) += 1;
                }

                for x in (curr_tile.0 + 1)..pipe_map[0].len() {
                    if (x, curr_tile.1) == (7, 6) {
                        println!("{:?}", curr_tile);
                    }
                    //insert or increment
                    *result.entry((x, curr_tile.1)).or_insert(0) -= 1;
                }
            }
        }

        coming_from = get_next_dir(pipe_map[curr_tile.1][curr_tile.0], coming_from);
        curr_tile = get_next_coordinates(curr_tile, &coming_from);
    }

    let mut a = result.iter().map(|x| x.1).collect::<Vec<&i32>>();
    a.sort();
    println!("{:?}", a);

    result
        .into_iter()
        .filter(|&x| x.1 == 4 && !path_tiles.contains(&x.0))
        .map(|x| x.1)
        .collect::<Vec<i32>>()
        .len() as u32
}

fn get_start_dir(pipe_map: Vec<Vec<char>>, start: (usize, usize)) -> Dir {
    if start.1 > 0 {
        let tile = (start.0, start.1 - 1);
        if ['|', 'F', '7'].contains(&pipe_map[tile.1][tile.0]) {
            return Dir::South;
        }
    }

    if start.1 < pipe_map.len() - 1 {
        let tile = (start.0, start.1 + 1);
        if ['|', 'L', 'J'].contains(&pipe_map[tile.1][tile.0]) {
            return Dir::North;
        }
    }

    if start.0 > 0 {
        let tile = (start.0 - 1, start.1);
        if ['-', 'L', 'F'].contains(&pipe_map[tile.1][tile.0]) {
            return Dir::East;
        }
    }

    if start.0 < pipe_map[0].len() - 1 {
        let tile = (start.0 + 1, start.1);
        if ['-', '7', 'J'].contains(&pipe_map[tile.1][tile.0]) {
            return Dir::West;
        }
    }

    Dir::North
}

fn get_next_coordinates(coordinates: (usize, usize), coming_from: &Dir) -> (usize, usize) {
    match coming_from {
        Dir::South => (coordinates.0, coordinates.1 - 1),
        Dir::North => (coordinates.0, coordinates.1 + 1),
        Dir::East => (coordinates.0 - 1, coordinates.1),
        Dir::West => (coordinates.0 + 1, coordinates.1),
    }
}

fn get_next_dir(tile: char, coming_from: Dir) -> Dir {
    match coming_from {
        Dir::North => match tile {
            '|' => Dir::North,
            'L' => Dir::West,
            'J' => Dir::East,
            _ => panic!(),
        },
        Dir::South => match tile {
            '|' => Dir::South,
            'F' => Dir::West,
            '7' => Dir::East,
            _ => panic!(),
        },
        Dir::East => match tile {
            '-' => Dir::East,
            'L' => Dir::South,
            'F' => Dir::North,
            _ => panic!(),
        },
        Dir::West => match tile {
            '-' => Dir::West,
            'J' => Dir::South,
            '7' => Dir::North,
            _ => panic!(),
        },
    }
}

fn get_inner_dirs(tile: char, coming_from: &Dir) -> Vec<Dir> {
    match coming_from {
        Dir::North => match tile {
            '|' => vec![Dir::West],
            'L' => vec![Dir::West, Dir::South],
            'J' => vec![Dir::West, Dir::North],
            _ => panic!(),
        },
        Dir::South => match tile {
            '|' => vec![Dir::East],
            'F' => vec![Dir::East, Dir::South],
            '7' => vec![Dir::East, Dir::North],
            _ => panic!(),
        },
        Dir::East => match tile {
            '-' => vec![Dir::North],
            'L' => vec![Dir::North, Dir::East],
            'F' => vec![Dir::North, Dir::West],
            _ => panic!(),
        },
        Dir::West => match tile {
            '-' => vec![Dir::South],
            'J' => vec![Dir::South, Dir::East],
            '7' => vec![Dir::South, Dir::West],
            _ => panic!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs() {
        let mut map = vec![
            vec!['.', '|', '.'],
            vec!['.', 'S', '.'],
            vec!['.', '.', '.'],
        ];

        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::South);

        map[0][1] = 'F';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::South);

        map[0][1] = '7';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::South);

        map[0][1] = '-';
        map[1][0] = '-';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::East);

        map[1][0] = 'L';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::East);

        map[1][0] = 'F';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::East);

        map[1][0] = '|';
        map[2][1] = '|';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::North);

        map[2][1] = 'L';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::North);

        map[2][1] = 'J';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::North);

        map[2][1] = '-';
        map[1][2] = '-';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::West);

        map[1][2] = '7';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::West);

        map[1][2] = 'J';
        assert_eq!(get_start_dir(map.clone(), (1, 1)), Dir::West);
    }
}
