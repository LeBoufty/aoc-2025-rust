use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn open_input(day: u8, test: bool) -> std::io::Result<BufReader<Box<File>>> {
    let path: String;
    if test {
        path = format!("test_inputs/day{:02}.txt", day);
    } else {
        path = format!("inputs/day{:02}.txt", day);
    }
    let file = File::open(path)?;
    Ok(BufReader::new(Box::new(file)))
}

pub fn read_lines(day: u8, test: bool) -> std::io::Result<Vec<String>> {
    let reader = open_input(day, test)?;
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

pub fn read_input(day: u8, test: bool) -> std::io::Result<String> {
    let mut reader = open_input(day, test)?;
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    Ok(input)
}
