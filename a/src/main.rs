use anyhow::{Context, Result};

use a::{parse_input, read_input};

fn main() -> Result<()> {
    let (mut first, mut second): (Vec<_>, Vec<_>) =
        read_input().with_context(|| "Error parsing input")?;

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
