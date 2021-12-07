use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day8.txt")?;
    println!("day8-1: {}", run_1(&input)?);
    println!("day8-2: {}", run_2(&input)?);
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
    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn aoc8_run_1() {
        // assert_eq!(super::run_1(INPUT), 5);
    }

    #[test]
    fn aoc8_run_2() {
        // assert_eq!(super::run_2(INPUT), 8);
    }
}
