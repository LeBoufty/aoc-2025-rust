use crate::inputs::read_input;
use std::error;

struct Range {
    start: u64,
    end: u64,
}

fn parse_input(test: bool) -> Result<Vec<Range>, Box<dyn error::Error>> {
    let mut sortie = vec![];
    let input = read_input(2, test)?;
    for r in input.split(",") {
        let numbers: Vec<u64> = r
            .split("-")
            .map(|n| n.trim().parse::<u64>().unwrap_or(0))
            .collect();
        sortie.push(Range {
            start: numbers[0],
            end: numbers[1],
        })
    }
    Ok(sortie)
}

fn duplicate(left_half: u64) -> u64 {
    left_half * (10 as u64).pow(left_half.ilog10() + 1) + left_half
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let ranges = parse_input(test)?;
    let mut invalid = vec![];
    for r in ranges {
        let mut start = r.start;
        let end = r.end;
        let mut power = start.ilog10();
        let endpower = end.ilog10();
        let mut first_round = true;
        while power < endpower {
            // On ne fait quelque chose que s'il y a un nombre pair de chiffres.
            if power % 2 == 1 {
                // Action différente pour le premier passage car on commence pas à 10^p
                if first_round {
                    // On divise le nombre en deux
                    let half_divider = (10 as u64).pow(power.div_euclid(2) + 1);
                    let right_half = start % half_divider;
                    let left_half = start.div_euclid(half_divider);
                    // Si par exemple le départ est 4443, 4444 comptera donc il faut l'ajouter.
                    if right_half <= left_half {
                        invalid.push(duplicate(left_half));
                    }
                    // En suite on collecte tous les nombres doubles, donc par exemple de 4545 à 9999.
                    for i in (left_half + 1)..(10 as u64).pow(half_divider.ilog10()) {
                        invalid.push(duplicate(i));
                    }
                } else {
                    // Pareil qu'au premier tour sauf qu'on sait qu'on aura tout de 100100 à 999999.
                    for left_half in (10 as u64).pow(power.div_euclid(2))
                        ..(10 as u64).pow(power.div_euclid(2) + 1)
                    {
                        invalid.push(duplicate(left_half));
                    }
                }
            }
            // On incrémente la puissance et passe au tour suivant.
            power += 1;
            start = (10 as u64).pow(power);
            first_round = false;
        }
        // Fin de la boucle, on sait qu'on est sur la même puissance de 10.
        // Si cette puissance est paire, nombre impair de chiffres on s'en fiche
        if power % 2 == 1 {
            // On fait la même chose qu'au premier tour
            let half_divider = (10 as u64).pow(power.div_euclid(2) + 1);
            let start_right_half = start % half_divider;
            let start_left_half = start.div_euclid(half_divider);
            let end_right_half = end % half_divider;
            let end_left_half = end.div_euclid(half_divider);
            if start_right_half <= start_left_half && duplicate(start_left_half) <= end {
                invalid.push(duplicate(start_left_half));
            }
            if end_right_half >= end_left_half && end_left_half != start_left_half {
                invalid.push(duplicate(end_left_half));
            }
            for i in (start_left_half + 1)..end_left_half {
                invalid.push(duplicate(i));
            }
        }
    }
    Ok(invalid.iter().sum())
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let ranges = parse_input(test)?;
    let mut invalid = vec![];
    for r in ranges {
        for i in r.start..r.end + 1 {
            let power = i.ilog10();
            for p in 1..power + 1 {
                let mut number = i;
                let mut rests = vec![];
                while number.div_euclid((10 as u64).pow(p)) != 0 {
                    rests.push(number % (10 as u64).pow(p));
                    number = number.div_euclid((10 as u64).pow(p));
                }
                rests.push(number);
                if rests.iter().all(|&x| x == rests[0]) {
                    if rests.iter().max().unwrap().ilog10().eq(&(p - 1)) {
                        invalid.push(i);
                        break;
                    }
                }
            }
        }
    }
    Ok(invalid.iter().sum())
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 1227775554);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 4174379265);
}
