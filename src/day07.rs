use std::{collections::HashSet, error};

use crate::{chargrid::make_grid, inputs::read_lines};

#[derive(PartialEq)]
struct Beam {
    coords: (i32, i32),
    count: u64,
}

struct Manifold {
    beams: Vec<(i32, i32)>,
    beams2: Vec<Beam>,
    splitters: HashSet<(i32, i32)>,
    width: usize,
    height: usize,
}

impl Manifold {
    fn advance(&mut self) -> u64 {
        let mut splits: u64 = 0;
        let mut newbeams: Vec<(i32, i32)> = vec![];
        for b in &self.beams {
            if self.splitters.contains(&(b.0 + 1, b.1)) {
                splits += 1;
                let beam1 = (b.0 + 1, b.1 + 1);
                let beam2 = (b.0 + 1, b.1 - 1);
                if beam1.1 < self.width as i32 && !newbeams.contains(&beam1) {
                    newbeams.push(beam1);
                }
                if beam2.1 >= 0 && !newbeams.contains(&beam2) {
                    newbeams.push(beam2);
                }
            } else {
                if !newbeams.contains(&(b.0 + 1, b.1)) {
                    newbeams.push((b.0 + 1, b.1));
                }
            }
        }
        self.beams = newbeams;
        splits
    }
    fn advance_p2(&mut self) -> u64 {
        let mut newbeams: Vec<Beam> = vec![];
        for b in &self.beams2 {
            if self.splitters.contains(&(b.coords.0 + 1, b.coords.1)) {
                let beam1 = (b.coords.0 + 1, b.coords.1 + 1);
                let beam2 = (b.coords.0 + 1, b.coords.1 - 1);
                if beam1.1 < self.width as i32 {
                    if let Some(r) = newbeams.iter().position(|x| x.coords.eq(&beam1)) {
                        newbeams[r].count += b.count;
                    } else {
                        newbeams.push(Beam {
                            coords: beam1,
                            count: b.count,
                        });
                    }
                }
                if beam2.1 >= 0 {
                    if let Some(r) = newbeams.iter().position(|x| x.coords.eq(&beam2)) {
                        newbeams[r].count += b.count;
                    } else {
                        newbeams.push(Beam {
                            coords: beam2,
                            count: b.count,
                        });
                    }
                }
            } else {
                let beam = (b.coords.0 + 1, b.coords.1);
                if let Some(r) = newbeams.iter().position(|x| x.coords.eq(&beam)) {
                    newbeams[r].count += b.count;
                } else {
                    newbeams.push(Beam {
                        coords: beam,
                        count: b.count,
                    });
                }
            }
        }
        self.beams2 = newbeams;
        self.beams2.iter().map(|x| x.count).sum()
    }
}

fn parse_input(test: bool) -> Result<Manifold, Box<dyn error::Error>> {
    let lines = read_lines(7, test)?;
    let grid = make_grid(&lines);
    let beam = grid.find_all('S')[0];
    Ok(Manifold {
        beams: vec![beam],
        beams2: vec![Beam {
            coords: beam,
            count: 1,
        }],
        splitters: grid.find_all_hset('^'),
        width: grid.width(),
        height: grid.height(),
    })
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut manifold = parse_input(test)?;
    let mut score: u64 = 0;
    for _ in 0..manifold.height {
        score += manifold.advance();
    }
    Ok(score)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut manifold = parse_input(test)?;
    let mut score: u64 = 0;
    for _ in 0..manifold.height {
        score = manifold.advance_p2();
    }
    Ok(score)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 40);
}
