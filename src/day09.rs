use std::error;

use crate::inputs::read_lines;

fn parse_input(test: bool) -> Result<Vec<(u64, u64)>, Box<dyn error::Error>> {
    let lines = read_lines(9, test)?;
    let mut sortie = lines
        .iter()
        .map(|x| {
            x.trim()
                .split(",")
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|x| (x[0], x[1]))
        .collect::<Vec<(u64, u64)>>();
    sortie.push(sortie[0]);
    Ok(sortie)
}

fn area(p1: &(u64, u64), p2: (u64, u64)) -> u64 {
    (p2.0.max(p1.0) - p2.0.min(p1.0) + 1) * (p2.1.max(p1.1) - p2.1.min(p1.1) + 1)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut score: u64 = 0;
    for v in values.clone() {
        score = values.iter().fold(
            score,
            |acc, x| if area(x, v) > acc { area(x, v) } else { acc },
        );
    }
    Ok(score)
}

fn hits_boundaries(p1: &(u64, u64), p2: (u64, u64), sides: &Vec<((u64, u64), (u64, u64))>) -> bool {
    let pmin = (p1.0.min(p2.0), p1.1.min(p2.1));
    let pmax = (p1.0.max(p2.0), p1.1.max(p2.1));
    for v in sides {
        let vmin = (v.0.0.min(v.1.0), v.0.1.min(v.1.1));
        let vmax = (v.0.0.max(v.1.0), v.0.1.max(v.1.1));
        if pmax.1 < vmin.1 + 1 || pmin.1 > vmax.1 - 1 || pmax.0 < vmin.0 + 1 || pmin.0 > vmax.0 - 1
        {
            continue;
        }
        return true;
    }
    return false;
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let sides: Vec<((u64, u64), (u64, u64))> = (0..values.len() - 1)
        .map(|i| (values[i], values[i + 1]))
        .collect();
    let mut score: u64 = 0;
    for v in values.clone() {
        score = values.iter().fold(score, |acc, x| {
            if area(x, v) > acc && !hits_boundaries(x, v, &sides) {
                area(x, v)
            } else {
                acc
            }
        });
    }
    Ok(score)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 50);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 24);
}
