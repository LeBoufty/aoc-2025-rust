use std::error;

use crate::inputs::read_lines;

fn parse_input(test: bool) -> Result<Vec<Vec<u32>>, Box<dyn error::Error>> {
    let lines = read_lines(3, test)?;
    let sortie: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut score = 0;
    for v in values {
        let mut highest_dozen = v[0];
        let mut highest_unit = v[1];
        for i in 1..v.len() - 1 {
            if v[i] > highest_dozen {
                highest_dozen = v[i];
                highest_unit = v[i + 1];
            } else if v[i] > highest_unit {
                highest_unit = v[i];
            }
        }
        if v.last().unwrap() > &highest_unit {
            highest_unit = *v.last().unwrap();
        }
        score += highest_dozen * 10 + highest_unit;
    }
    Ok(score as u64)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut score: u64 = 0;
    for v in values {
        let mut battery: u64 = 0;
        let mut remaining_digits: u32 = 12;
        let mut start_index = 0;
        while remaining_digits > 0 {
            remaining_digits -= 1;
            let mut highest = v[start_index];
            for i in start_index + 1..v.len() - (remaining_digits as usize) {
                if v[i] > highest {
                    highest = v[i];
                    start_index = i;
                }
            }
            start_index += 1;
            battery += (highest as u64) * (10 as u64).pow(remaining_digits);
        }
        score += battery;
    }
    Ok(score)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 357);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 3121910778619);
}
