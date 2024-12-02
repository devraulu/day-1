use anyhow::Result;
use std::io::BufRead;

pub fn parse_input(reader: impl BufRead) -> Result<(Vec<i32>, Vec<i32>)> {
    let result: (Vec<_>, Vec<_>) = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut cols = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok());
            let first = cols.next().unwrap_or_default();
            let second = cols.next().unwrap_or_default();
            (first, second)
        })
        .unzip();

    Ok(result)
}

pub fn read_input() -> Result<(Vec<i32>, Vec<i32>)> {
    let f = std::fs::File::open("../input.txt")?;
    let reader = std::io::BufReader::new(f);

    parse_input(reader)
}
