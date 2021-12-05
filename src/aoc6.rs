use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day6.txt").unwrap();
    println!("6:1: {}", run_1(&input)?);
    println!("6:2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc6_parse() {}

    #[test]
    fn aoc6_run_1() {}

    #[test]
    fn aoc6_run_2() {}
}
