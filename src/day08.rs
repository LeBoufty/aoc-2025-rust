use std::error;

use crate::inputs::read_lines;

struct Circuits {
    coords: Vec<(i64, i64, i64)>,
    groups: Vec<Vec<usize>>,
}

impl Circuits {
    fn distances(&self, count: Option<usize>) -> Vec<(usize, usize, i64)> {
        let mut sortie: Vec<(usize, usize, i64)> = vec![];
        let available_starts = self
            .coords
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &(i64, i64, i64))>>();
        for start in available_starts {
            let already_done = sortie
                .iter()
                .filter(|x| x.1 == start.0)
                .map(|y| y.0)
                .collect::<Vec<usize>>();
            let mut distances = self
                .coords
                .iter()
                .enumerate()
                .filter(|(i, _)| !i.eq(&start.0) && !already_done.contains(i))
                .map(|(i, val)| {
                    (
                        start.0,
                        i,
                        (val.0 - start.1.0).pow(2)
                            + (val.1 - start.1.1).pow(2)
                            + (val.2 - start.1.2).pow(2),
                    )
                })
                .collect::<Vec<(usize, usize, i64)>>();
            sortie.append(&mut distances);
        }
        sortie.sort_by(|a, b| a.2.cmp(&b.2));
        sortie.reverse();
        if !count.is_none() {
            sortie = sortie.split_off(sortie.len() - count.unwrap());
        }
        sortie
    }

    fn link(&mut self, start: usize, dest: usize) -> bool {
        let start_group = self.groups.iter().position(|x| x.contains(&start)).unwrap();
        let dest_group = self.groups.iter().position(|x| x.contains(&dest)).unwrap();
        if start_group != dest_group {
            let mut dest_group_vec = self.groups[dest_group].clone();
            self.groups[start_group].append(&mut dest_group_vec);
            self.groups.remove(dest_group);
            return true;
        }
        return false;
    }
}

fn parse_input(test: bool) -> Result<Circuits, Box<dyn error::Error>> {
    let lines = read_lines(8, test)?;
    let mut coords: Vec<(i64, i64, i64)> = vec![];
    for l in lines {
        let coordvec = l
            .trim()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        coords.push((coordvec[0], coordvec[1], coordvec[2]));
    }
    let groups = (0..coords.len())
        .map(|i| vec![i])
        .collect::<Vec<Vec<usize>>>();
    let sortie = Circuits { coords, groups };
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut circuits = parse_input(test)?;
    let loop_size = if test { 10 } else { 1000 };
    let mut distances = circuits.distances(None);
    for _ in 0..loop_size {
        let closest = distances.pop().unwrap();
        circuits.link(closest.0, closest.1);
    }
    let mut groups = circuits
        .groups
        .clone()
        .iter()
        .map(|x| x.len() as u64)
        .collect::<Vec<u64>>();
    groups.sort();
    groups.reverse();
    let score = groups[0] * groups[1] * groups[2];
    Ok(score)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut circuits = parse_input(test)?;
    let mut distances = circuits.distances(None);
    let mut score = 0;
    while circuits.groups.len() > 1 {
        let closest = distances.pop().unwrap();
        circuits.link(closest.0, closest.1);
        score = (circuits.coords[closest.0].0 * circuits.coords[closest.1].0) as u64;
    }
    Ok(score)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 40);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 25272);
}
