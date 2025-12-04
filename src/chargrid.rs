use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct CharGrid {
    grid: Vec<Vec<char>>,
}

pub struct Symbol {
    symbol: char,
    coords: (i32, i32),
}

impl Symbol {
    pub fn get_symbol(&self) -> char {
        return self.symbol;
    }
    pub fn get_coords(&self) -> (i32, i32) {
        return self.coords;
    }
}

impl CharGrid {
    pub fn height(&self) -> usize {
        return self.grid.len();
    }
    pub fn width(&self) -> usize {
        return self.grid[0].len();
    }
    pub fn get(&self, l: usize, c: usize) -> Option<char> {
        if !self.is_in_grid(l, c) {
            return None;
        }
        return Some(self.grid[l][c]);
    }
    pub fn find_all(&self, ch: char) -> Vec<(i32, i32)> {
        let mut sortie: Vec<(i32, i32)> = Vec::new();
        for l in 0..self.height() {
            for c in 0..self.grid[l].len() {
                if self.get(l, c).unwrap() == ch {
                    sortie.push((l as i32, c as i32));
                }
            }
        }
        sortie
    }
    pub fn find_all_hset(&self, ch: char) -> HashSet<(i32, i32)> {
        let mut sortie: HashSet<(i32, i32)> = HashSet::new();
        for l in 0..self.height() {
            for c in 0..self.grid[l].len() {
                if self.get(l, c).unwrap() == ch {
                    sortie.insert((l as i32, c as i32));
                }
            }
        }
        sortie
    }
    pub fn find_all_symbols(&self, default_char: Option<char>) -> Vec<Symbol> {
        let mut sortie: Vec<Symbol> = Vec::new();
        for l in 0..self.grid.len() {
            for c in 0..self.grid[l].len() {
                if default_char.is_none() {
                    sortie.push(Symbol {
                        symbol: self.grid[l][c],
                        coords: (l as i32, c as i32),
                    });
                } else if self.grid[l][c] != default_char.unwrap() {
                    sortie.push(Symbol {
                        symbol: self.grid[l][c],
                        coords: (l as i32, c as i32),
                    });
                }
            }
        }
        sortie
    }
    pub fn get_hashmap(&self, default_char: Option<char>) -> HashMap<char, Vec<(i32, i32)>> {
        let mut sortie: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        self.find_all_symbols(default_char).iter().for_each(|s| {
            sortie
                .entry(s.get_symbol())
                .or_insert(Vec::new())
                .push(s.get_coords())
        });
        sortie
    }
    pub fn is_in_grid(&self, l: usize, c: usize) -> bool {
        return l < self.height() && c < self.width();
    }
}

pub fn make_grid(input: &Vec<String>) -> CharGrid {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in input {
        grid.push(i.chars().collect());
    }
    CharGrid { grid }
}
