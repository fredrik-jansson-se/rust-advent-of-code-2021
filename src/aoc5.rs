use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day5.txt").unwrap();

    println!("5:1 {}", run_1(&input)?);
    println!("5:2 {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc5_run_1() {}
}
