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

fn corner_oob(
    altp: (u64, u64),
    values: &Vec<(u64, u64)>,
    sides: &Vec<((u64, u64), (u64, u64))>,
) -> bool {
    if !values.contains(&altp) {
        let hits = sides
            .iter()
            .filter(|x| {
                x.1.1 == x.0.1 // Boundary is horizontal
                    && x.0.0.min(x.1.0) <= altp.0 // Its leftmost point is to the left of or on altp
                    && x.0.0.max(x.1.0) > altp.0 // Its rightmost point is to the right of or on altp
                    && x.1.1 >= altp.1 // altp sits above or on it
                    || x.1.0 == x.0.0 // Or bouncary is vertical
                        && x.1.0 == altp.0 // altp sits on its axis
                        && x.0.1.max(x.1.1) >= altp.1 // and it ends below altp
            })
            .collect::<Vec<&((u64, u64), (u64, u64))>>();
        let hitscount: u64 = hits.iter().filter(|x| {
            x.1.1 == x.0.1 // Boundary is horizontal
                || hits.iter().any(|y| y.0.0.max(y.1.0) == x.0.0.min(x.1.0) && y.0.1 == y.1.1 && y.0.1 == x.0.1.min(x.1.1))
        }).map(|_| 1).sum();
        if hitscount % 2 == 0 {
            println!("Corner {:?} out of bounds ({} hits)", altp, hitscount);
            return true;
        }
    }
    return false;
}

fn hits_boundaries(
    p1: &(u64, u64),
    p2: (u64, u64),
    values: &Vec<(u64, u64)>,
    sides: &Vec<((u64, u64), (u64, u64))>,
) -> bool {
    let pmin = (p1.0.min(p2.0), p1.1.min(p2.1));
    let pmax = (p1.0.max(p2.0), p1.1.max(p2.1));
    let altp1 = (p1.0, p2.1);
    let altp2 = (p2.0, p1.1);
    if corner_oob(altp1, &values, &sides) {
        return true;
    }
    if corner_oob(altp2, &values, &sides) {
        return true;
    }
    for v in sides {
        let vmin = (v.0.0.min(v.1.0), v.0.1.min(v.1.1));
        let vmax = (v.0.0.max(v.1.0), v.0.1.max(v.1.1));
        if pmin.eq(&vmin) || pmax.eq(&vmax) {
            // Boundary part of corner
            continue;
        }
        if vmax.0 >= pmax.0 && vmin.0 <= pmin.0 || vmax.1 >= pmax.1 && vmin.1 <= pmin.1 {
            // Edge of rectangle sitting on boundary
            continue;
        }
        if vmax.0 == vmin.0 // Boundary is vertical
            && vmax.0 < pmax.0 // ...
            && vmin.0 > pmin.0 // ... and in bounds
            && (vmin.1 <= pmin.1 && vmax.1 > pmin.1 || vmax.1 >= pmax.1 && vmin.1 < pmax.1)
            || vmax.1 == vmin.1 // Boundary is horizontal
                && vmax.1 < pmax.1 // ...
                && vmin.1 > pmin.1 // ... and in bounds
                && (vmin.0 <= pmin.0 && vmax.0 > pmin.0 || vmax.0 >= pmax.0 && vmin.0 < pmax.0)
        {
            println!(
                "Rectangle {:?} hits boundary {:?} (check 2)",
                (p1, p2),
                (v.0, v.1)
            );
            return true;
        }
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
            if area(x, v) > acc && !hits_boundaries(x, v, &values, &sides) {
                if area(x, v) > 24 {
                    println!("Rectangle {:?} is in bounds.", (x, v));
                }
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
