use crate::inputs::read_lines;
use std::error;

fn parse_input(test: bool) -> Result<Vec<i32>, Box<dyn error::Error>> {
    let lines = read_lines(1, test)?;
    let mut sortie = vec![];
    for mut l in lines {
        let value: i32 = l.split_off(1).parse()?;
        if l.starts_with("L") {
            sortie.push(-value);
        } else {
            sortie.push(value);
        }
    }
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut score = 0;
    let mut current = 50;
    let mut tours;
    for v in values {
        current += v;
        if current < 0 {
            tours = current.div_euclid(100).abs();
            current += tours * 100;
        }
        if current >= 100 {
            tours = current.div_euclid(100);
            current -= tours * 100;
        }
        if current == 0 {
            score += 1;
        }
    }
    Ok(score as u64)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut score = 0;
    let mut current = 50;
    let mut tours;
    let mut started_at_0;
    for v in values {
        tours = 0;
        started_at_0 = current == 0;
        current += v;
        if current < 0 {
            tours = current.div_euclid(100).abs();
            current += tours * 100;
            if started_at_0 {
                tours -= 1;
            }
            if current == 0 {
                score += 1;
            }
        } else if current >= 100 {
            tours = current.div_euclid(100);
            current -= tours * 100;
        } else if current == 0 {
            score += 1;
        }
        score += tours;
    }
    Ok(score as u64)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 6);
}
