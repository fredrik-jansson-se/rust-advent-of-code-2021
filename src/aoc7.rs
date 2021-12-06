use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day7.txt")?;
    println!("day7-1: {}", run_1(&input)?);
    println!("day7-2: {}", run_2(&input)?);
    Ok(())
}

pub fn run_1(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

pub fn run_2(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc7_parse() {}
    #[test]
    fn aoc7_run_1() {}

    #[test]
    fn aoc7_run_2() {}
}
