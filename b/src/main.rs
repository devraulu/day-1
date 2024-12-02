use a::{parse_input, read_input};
use anyhow::Result;
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let (mut first, mut second) = read_input()?;

    let score: i32 = first
        .into_iter()
        .map(|x| (second.iter().filter(|&y| x.eq(y)).count() as i32) * x)
        .sum();

    println!("result {}", score);

    Ok(())
}
