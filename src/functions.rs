use lazy_static::lazy_static;
use std::collections::HashMap;

type DayPartFn = fn(bool) -> Result<u64, Box<dyn std::error::Error>>;

lazy_static! {
    pub static ref FUNCTIONS: HashMap<(&'static str, &'static str), DayPartFn> = {
        let mut m = HashMap::new();
        m.insert(("01", "1"), crate::day01::part1 as DayPartFn);
        m.insert(("01", "2"), crate::day01::part2 as DayPartFn);
        m.insert(("02", "1"), crate::day02::part1 as DayPartFn);
        m.insert(("02", "2"), crate::day02::part2 as DayPartFn);
        m.insert(("03", "1"), crate::day03::part1 as DayPartFn);
        m.insert(("03", "2"), crate::day03::part2 as DayPartFn);
        m.insert(("04", "1"), crate::day04::part1 as DayPartFn);
        m.insert(("04", "2"), crate::day04::part2 as DayPartFn);
        m.insert(("05", "1"), crate::day05::part1 as DayPartFn);
        m.insert(("05", "2"), crate::day05::part2 as DayPartFn);
        m
    };
}
