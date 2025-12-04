use std::error;

use crate::{
    chargrid::{CharGrid, make_grid},
    inputs::read_lines,
};

fn parse_input(test: bool) -> Result<CharGrid, Box<dyn error::Error>> {
    let input = read_lines(4, test)?;
    Ok(make_grid(&input))
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let grid = parse_input(test)?;
    let rolls = grid.find_all_hset('@');
    let mut score: u64 = 0;
    for roll in rolls.iter() {
        let adjacent: Vec<(i32, i32)> = vec![
            (roll.0 - 1, roll.1 - 1),
            (roll.0, roll.1 - 1),
            (roll.0 + 1, roll.1 - 1),
            (roll.0 - 1, roll.1),
            (roll.0 + 1, roll.1),
            (roll.0 - 1, roll.1 + 1),
            (roll.0, roll.1 + 1),
            (roll.0 + 1, roll.1 + 1),
        ];
        let adjcount = adjacent
            .iter()
            .filter(|x| rolls.contains(*x))
            .collect::<Vec<&(i32, i32)>>()
            .len();
        if adjcount < 4 {
            score += 1;
        }
    }
    Ok(score)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let grid = parse_input(test)?;
    let mut rolls = grid.find_all_hset('@');
    let mut score: u64 = 0;
    let mut done = false;
    while !done {
        done = true;
        for roll in rolls.clone().iter() {
            let adjacent: Vec<(i32, i32)> = vec![
                (roll.0 - 1, roll.1 - 1),
                (roll.0, roll.1 - 1),
                (roll.0 + 1, roll.1 - 1),
                (roll.0 - 1, roll.1),
                (roll.0 + 1, roll.1),
                (roll.0 - 1, roll.1 + 1),
                (roll.0, roll.1 + 1),
                (roll.0 + 1, roll.1 + 1),
            ];
            let adjcount = adjacent
                .iter()
                .filter(|x| rolls.contains(*x))
                .collect::<Vec<&(i32, i32)>>()
                .len();
            if adjcount < 4 {
                rolls.remove(roll);
                score += 1;
                done = false;
            }
        }
    }
    Ok(score)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 43);
}
