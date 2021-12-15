use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day25.txt").unwrap();
    println!("day9-1: {}", run_1(&input)?);
    println!("day9-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {}
