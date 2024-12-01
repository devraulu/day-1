use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let (mut first, mut second): (Vec<_>, Vec<_>) = reader
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

    first.sort_unstable();
    second.sort_unstable();

    let difference: i32 = first
        .iter()
        .zip(&second)
        .map(|(first, second)| (first - second).abs())
        .sum();

    println!("{}", difference);
    Ok(())
}
