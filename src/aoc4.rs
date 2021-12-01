use std::fs;

pub fn run() {
    let input = fs::read_to_string("day4.txt").unwrap();
    println!("4:1 {}", run_1(&input));
    println!("4:2 {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    0
}

fn run_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {}
