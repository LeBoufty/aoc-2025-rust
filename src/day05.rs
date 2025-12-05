use std::error;

use crate::inputs::read_lines;

#[derive(Clone, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

struct Inventory {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

fn parse_input(test: bool) -> Result<Inventory, Box<dyn error::Error>> {
    let birthday_lines = read_lines(5, test)?;
    let mut sortie_d_anniversaire = Inventory {
        ranges: vec![],
        ingredients: vec![],
    };
    let mut drapeau_d_anniversaire = false;
    for l in birthday_lines {
        if l.trim() == "" {
            drapeau_d_anniversaire = true;
        } else if !drapeau_d_anniversaire {
            let birthday_numbers: Vec<u64> = l
                .split("-")
                .map(|n| n.trim().parse::<u64>().unwrap_or(0))
                .collect();
            sortie_d_anniversaire.ranges.push(Range {
                start: birthday_numbers[0],
                end: birthday_numbers[1],
            })
        } else {
            let birthday_ingredient: u64 = l.trim().parse::<u64>().unwrap_or(0);
            sortie_d_anniversaire.ingredients.push(birthday_ingredient);
        }
    }
    Ok(sortie_d_anniversaire)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let inventaire_d_anniversaire = parse_input(test)?;
    let mut score_d_anniversaire: u64 = 0;
    for i in inventaire_d_anniversaire.ingredients {
        let ranges_d_anniversaire: Vec<&Range> = inventaire_d_anniversaire
            .ranges
            .iter()
            .filter(|x| i <= x.end && i >= x.start)
            .collect();
        if ranges_d_anniversaire.len() > 0 {
            score_d_anniversaire += 1;
        }
    }
    Ok(score_d_anniversaire)
}

fn remove_birthday_range(range: Range, others: Vec<Range>) -> Vec<Range> {
    let mut sortie_d_anniversaire: Vec<Range> = vec![];
    for o in others {
        // OOOOO####RRR
        if range.start <= o.end && range.end >= o.end {
            let to_add = Range {
                start: o.start,
                end: range.start - 1,
            };
            if !sortie_d_anniversaire.contains(&to_add) {
                sortie_d_anniversaire.push(to_add)
            }
        // RRRRR#####OOOOOO
        } else if range.end >= o.start && range.start <= o.start {
            let to_add = Range {
                start: range.end + 1,
                end: o.end,
            };
            if !sortie_d_anniversaire.contains(&to_add) {
                sortie_d_anniversaire.push(to_add)
            }
        // OOOOO#######OOOOO
        } else if range.end <= o.end && range.start >= o.start {
            let range_1 = Range {
                start: o.start,
                end: range.start - 1,
            };
            let range_2 = Range {
                start: range.end + 1,
                end: o.end,
            };
            if !sortie_d_anniversaire.contains(&range_1) {
                sortie_d_anniversaire.push(range_1)
            }
            if !sortie_d_anniversaire.contains(&range_2) {
                sortie_d_anniversaire.push(range_2)
            }
        // RRRRR.....OOOOO
        } else if range.start > o.end || range.end < o.start {
            if !sortie_d_anniversaire.contains(&o) {
                sortie_d_anniversaire.push(o.clone());
            }
        }
    }
    sortie_d_anniversaire
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut ranges_d_anniversaire = parse_input(test)?.ranges;
    let mut score_d_anniversaire: u64 = 0;
    while !ranges_d_anniversaire.is_empty() {
        let frais_d_anniversaire = ranges_d_anniversaire.pop().unwrap();
        if frais_d_anniversaire.end >= frais_d_anniversaire.start {
            score_d_anniversaire += frais_d_anniversaire.end - frais_d_anniversaire.start + 1;
            ranges_d_anniversaire =
                remove_birthday_range(frais_d_anniversaire, ranges_d_anniversaire);
        }
    }
    Ok(score_d_anniversaire)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 14);
}
