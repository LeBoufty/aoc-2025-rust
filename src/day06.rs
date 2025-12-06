use std::error;

use crate::chargrid::{Symbol, make_grid};
use crate::inputs::read_lines;

#[derive(Clone)]
struct Problem {
    numbers: Vec<u64>,
    symbol: char,
}

impl Problem {
    fn value(&self) -> u64 {
        if self.symbol == '+' {
            return self.numbers.iter().sum();
        } else {
            return self.numbers.iter().fold(1, |acc, x| acc * x);
        }
    }
}

fn parse_input(test: bool) -> Result<Vec<Problem>, Box<dyn error::Error>> {
    let mut lines = read_lines(6, test)?;
    let mut sortie: Vec<Problem> = vec![];
    let symbols = lines.pop().unwrap();
    for c in symbols.replace(" ", "").chars() {
        sortie.push(Problem {
            numbers: vec![],
            symbol: c,
        });
    }
    for l in lines {
        let numbers: Vec<u64> = l
            .split(" ")
            .into_iter()
            .filter(|x| !x.eq(&""))
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        for i in 0..numbers.len() {
            sortie[i].numbers.push(numbers[i]);
        }
    }
    Ok(sortie)
}

fn parse_input_p2(test: bool) -> Result<Vec<Problem>, Box<dyn error::Error>> {
    let lines = read_lines(6, test)?;
    let grid = make_grid(&lines);
    let mut sortie: Vec<Problem> = vec![];
    let max_len = lines
        .iter()
        .fold(0, |acc, x| if x.len() > acc { x.len() } else { acc });
    let symbols = grid.find_all_symbols(Some(' '));
    let mut currentpb: Problem = Problem {
        numbers: vec![],
        symbol: '.',
    };
    for i in (0..max_len).rev() {
        let mut done = false;
        let mut col = symbols
            .iter()
            .filter(|x| x.get_coords().1 == i as i32)
            .collect::<Vec<&Symbol>>();
        if col.len() > 0 {
            col.sort_by(|a, b| a.get_coords().1.cmp(&b.get_coords().1));
            let mut colstr = col.iter().map(|x| x.get_symbol()).collect::<String>();
            if colstr.ends_with('*') || colstr.ends_with('+') {
                currentpb.symbol = colstr.remove(colstr.len() - 1);
                done = true;
            }
            currentpb.numbers.push(colstr.parse::<u64>().unwrap());
            if done {
                sortie.push(currentpb.clone());
                currentpb = Problem {
                    numbers: vec![],
                    symbol: '.',
                };
            }
        }
    }
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let problems = parse_input(test)?;
    let sortie = problems.iter().map(|x| x.value()).sum();
    Ok(sortie)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let problems = parse_input_p2(test)?;
    let sortie = problems.iter().map(|x| x.value()).sum();
    Ok(sortie)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 4277556);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 3263827);
}
