use std::fs;

pub fn run() {
    let input = fs::read_to_string("day3.txt").unwrap();
    let map = input;

    println!("3:1 - {}", run_1(&map));
    println!("3:2 - {}", run_2(&map));
}

fn run_1(map: &str) -> usize {
    0
}

fn run_2(map: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {}
